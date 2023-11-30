use std::fs::File;
use std::io::{self, Write};

use crate::parser::CommandType;

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        match File::create(filename) {
            Ok(file) => Ok(CodeWriter { file }),
            Err(e) => Err(e),
        }
    }

    pub fn set_file_name(&mut self, filename: &str) -> Result<(), io::Error> {
        let file = File::create(filename)?;
        self.file = file;
        Ok(())
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        match command {
            "add" => {
                self.sp_sub1();
                self.sp_sub1();
                self.file.write_all("@R0\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("D=M\n".as_bytes());
                self.sp_add1();
                self.file.write_all("@R0\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("D=D+M\n".as_bytes());
                self.sp_sub1();
                self.file.write_all("@R0\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("M=D\n".as_bytes());
                self.sp_add1();
            }
            "sub" => {
                self.sp_sub1();
                self.sp_sub1();
                self.file.write_all("@R0\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("D=M\n".as_bytes());
                self.sp_add1();
                self.file.write_all("@R0\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("D=D-M\n".as_bytes());
                self.sp_sub1();
                self.file.write_all("@R0\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("M=D\n".as_bytes());
                self.sp_add1();
            }
            "eq" => (),
            "lt" => (),
            "gt" => (),
            "neg" => (),
            "and" => (),
            "or" => (),
            "not" => (),
            _ => (),
        }
    }

    pub fn write_push_pop(&mut self, command_type: CommandType, segment: &str, index: i32) {
        match command_type {
            CommandType::CPush => match segment {
                "constant" => {
                    self.file
                        .write_all(("@".to_string() + &index.to_string() + "\n").as_bytes());
                    self.file.write_all("D=A\n".as_bytes());

                    self.file.write_all("@R0\n".as_bytes());
                    self.file.write_all("A=M\n".as_bytes());
                    self.file.write_all("M=D\n".as_bytes());

                    self.sp_add1();
                }
                _ => {}
            },
            CommandType::CPop => {}
            _ => (),
        }
    }

    fn sp_add1(&mut self) {
        self.file.write_all("@R0\n".as_bytes());
        self.file.write_all("M=M+1\n".as_bytes());
    }

    fn sp_sub1(&mut self) {
        self.file.write_all("@R0\n".as_bytes());
        self.file.write_all("M=M-1\n".as_bytes());
    }

    pub fn close() {}
}
