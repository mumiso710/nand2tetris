use std::io::Write;
use std::{env, fs::File, process};

mod code;
mod parser;

use code::Code;
use parser::CommandType;
use parser::Parser;

mod symbol_table;
use symbol_table::SymbolTable;

const MEMORY_OFFSET: usize = 15;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_filename = &args[1];
    let out_filename = in_filename.replace("asm", "hack");
    let mut out_file = File::create(out_filename).unwrap();

    let mut parser = Parser::new(in_filename).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let code = Code::new();

    let mut symbol_table = SymbolTable::new();

    let mut line_number = 0;
    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::LCommand => {
                if !(symbol_table.contains(&parser.symbol())) {
                    symbol_table.add_entry(parser.symbol(), line_number);
                    line_number -= 1;
                }
            }
            _ => (),
        }
        line_number += 1;
    }

    parser.commnad_number = 0;
    let mut variable_num = 0;

    while parser.has_more_commands() {
        parser.advance();

        match parser.command_type() {
            CommandType::ACommand => {
                // 0 + vvvvvvvvvv
                let mut binary = "0".to_string();

                match parser.symbol().parse::<i32>() {
                    Ok(value) => {
                        let value = format!("{:015b}", value);
                        binary.push_str(&value);
                        binary.push_str(&"\n");
                        out_file.write_all(binary.as_bytes()).unwrap()
                    }
                    Err(_) => {
                        let symbol = parser.symbol();
                        if symbol_table.contains(&symbol) {
                            let address = symbol_table.get_address(&symbol);
                            let address = format!("{:015b}", address);
                            println!("symbol: {symbol}, address: {address}");
                            binary.push_str(&address);
                            binary.push_str(&"\n");
                            out_file.write_all(binary.as_bytes()).unwrap()
                        } else {
                            variable_num += 1;
                            let address = variable_num + MEMORY_OFFSET;
                            symbol_table.add_entry(symbol, address);
                            let address = format!("{:015b}", address);
                            binary.push_str(&address);
                            binary.push_str(&"\n");
                            out_file.write_all(binary.as_bytes()).unwrap()
                        }
                    }
                }
            }

            CommandType::CCommand => {
                let mut binary = "111".to_string();
                if parser.comp().contains("M") {
                    binary.push_str("1")
                } else {
                    binary.push_str("0")
                }

                binary.push_str(code.comp(&parser.comp()));
                binary.push_str(code.dest(&parser.dest()));
                binary.push_str(code.jump(&parser.jump()));
                binary.push_str(&"\n");
                out_file.write_all(binary.as_bytes()).unwrap();
            }

            CommandType::LCommand => (),
        }
    }
}
