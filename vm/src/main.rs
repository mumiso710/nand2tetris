use std::{env, fs, path::PathBuf};

use parser::Parser;

use crate::{code_writer::CodeWriter, parser::CommandType};

mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let file_name = &args[1];
    let dir_name = &args[1];

    match extrac_vm_files(dir_name) {
        Ok(mut vm_files) => {
            let out_filename = dir_name.to_string()
                + "/"
                + &dir_name.split("/").last().unwrap().to_string()
                + ".asm";

            let mut code_writer = CodeWriter::new(&out_filename).unwrap();

            if let Some(index) = vm_files
                .iter()
                .position(|file| file.to_str().unwrap() == dir_name.to_string() + "/Sys.vm")
            {
                // 要素を取り出す
                let removed_element = vm_files.remove(index);

                // 先頭に挿入する
                vm_files.insert(0, removed_element);
            }

            for file_name in vm_files {
                let file_name = file_name.to_str().unwrap();
                // let out_filename = dir_name.to_string()
                //     + "/"
                //     + &dir_name.split("/").last().unwrap().to_string()
                //     + ".asm";

                let mut parser = Parser::new(&file_name).unwrap();

                while parser.has_more_commands() {
                    parser.advance();
                    match parser.command_type() {
                        CommandType::CArithmetic => code_writer.write_arithmetic(&parser.command()),
                        CommandType::CPush | CommandType::CPop => code_writer.write_push_pop(
                            parser.command_type(),
                            &parser.arg1(),
                            parser.arg2(),
                        ),
                        CommandType::CGoto => code_writer.write_goto(&parser.arg1()),
                        CommandType::CLabel => code_writer.write_label(&parser.arg1()),
                        CommandType::CIf => code_writer.write_if(&parser.arg1()),
                        CommandType::CFunction => {
                            code_writer.write_function(&parser.arg1(), parser.arg2())
                        }
                        CommandType::CReturn => code_writer.write_return(),
                        CommandType::CCall => code_writer.write_call(&parser.arg1(), parser.arg2()),
                    }
                }
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn extrac_vm_files(dir_name: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let entries = fs::read_dir(dir_name)?;
    let mut files = Vec::new();

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
