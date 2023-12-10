use std::{env, fs, path::PathBuf};

use parser::Parser;

use crate::{code_writer::CodeWriter, parser::CommandType};

mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_filename = &args[1];
    let out_filename = in_filename.replace("vm", "asm");

    let mut parser = Parser::new(&in_filename).unwrap();
    let mut code_writer = CodeWriter::new(&out_filename).unwrap();

    while parser.has_more_commands() {
        parser.advance();
        match parser.command_type() {
            CommandType::CArithmetic => code_writer.write_arithmetic(&parser.command()),
            CommandType::CPush | CommandType::CPop => {
                code_writer.write_push_pop(parser.command_type(), &parser.arg1(), parser.arg2())
            }
            CommandType::CGoto => code_writer.write_goto(&parser.arg1()),
            CommandType::CLabel => code_writer.write_label(&parser.arg1()),
            CommandType::CIf => code_writer.write_if(&parser.arg1()),
            CommandType::CFunction => code_writer.write_function(&parser.arg1(), parser.arg2()),
            CommandType::CReturn => code_writer.write_return(),
            CommandType::CCall => code_writer.write_call(&parser.arg1(), parser.arg2()),
        }
    }

    println!("{:?}", parser.commands);
}

fn extrac_vm_files(dir_name: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let entries = fs::read_dir(dir_name)?;
    let files = Vec::new();

    for entry in entries {
        let file_path = entry?.path();
        if let Some(extension) = file_path.extension() {
            if extension == "vm" {
                files.push(file_path);
            }
        }
    }

    Ok(files)
}
