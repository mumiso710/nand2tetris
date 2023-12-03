use std::fs::File;
use std::io::{self, Write};

use crate::parser::CommandType;

pub struct CodeWriter {
    file: File,
    label_counter: usize,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        match File::create(filename) {
            Ok(file) => Ok(CodeWriter {
                file,
                label_counter: 0,
            }),
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
                self.write_arithmetic_to_d("+");
                self.sp_sub1();
                self.write_d_to_stack();
            }
            "sub" => {
                self.write_arithmetic_to_d("-");
                self.sp_sub1();
                self.write_d_to_stack()
            }
            "not" => {
                self.sp_sub1();
                self.file.write_all("@SP\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("D=M\n".as_bytes());
                self.file.write_all("M=!D\n".as_bytes());
                self.sp_add1();
            }
            "neg" => {
                self.sp_sub1();
                self.file.write_all("@SP\n".as_bytes());
                self.file.write_all("A=M\n".as_bytes());
                self.file.write_all("D=M\n".as_bytes());
                self.file.write_all("M=-D\n".as_bytes());
                self.sp_add1();
            }
            "and" => {
                self.write_arithmetic_to_d("&");
                self.sp_sub1();
                self.write_d_to_stack();
            }
            "or" => {
                self.write_arithmetic_to_d("|");
                self.sp_sub1();
                self.write_d_to_stack();
            }
            "eq" => self.write_comparison_to_d("JEQ"),

            "gt" => self.write_comparison_to_d("JGT"),

            "lt" => self.write_comparison_to_d("JLT"),

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

                    self.file.write_all("@SP\n".as_bytes());
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
        self.file.write_all("@SP\n".as_bytes());
        self.file.write_all("M=M+1\n".as_bytes());
    }

    fn sp_sub1(&mut self) {
        self.file.write_all("@SP\n".as_bytes());
        self.file.write_all("M=M-1\n".as_bytes());
    }

    fn write_d_to_stack(&mut self) {
        self.file.write_all("@SP\n".as_bytes());
        self.file.write_all("A=M\n".as_bytes());
        self.file.write_all("M=D\n".as_bytes());
        self.sp_add1();
    }

    fn write_arithmetic_to_d(&mut self, op: &str) {
        self.sp_sub1();
        self.sp_sub1();
        self.file.write_all("@SP\n".as_bytes());
        self.file.write_all("A=M\n".as_bytes());
        self.file.write_all("D=M\n".as_bytes());
        self.sp_add1();
        self.file.write_all("@SP\n".as_bytes());
        self.file.write_all("A=M\n".as_bytes());
        self.file
            .write_all(("D=D".to_string() + op + "M\n").as_bytes());
    }

    fn write_comparison_to_d(&mut self, mnemonic: &str) {
        self.write_arithmetic_to_d("-");

        self.file
            .write_all(("@LABEL".to_string() + &self.label_counter.to_string() + "\n").as_bytes());
        self.label_counter += 1;

        self.file
            .write_all(("D;".to_string() + mnemonic + "\n").as_bytes());
        self.file.write_all("@0\n".as_bytes());
        self.file.write_all("D=A\n".as_bytes());
        self.file
            .write_all(("@LABEL".to_string() + &self.label_counter.to_string() + "\n").as_bytes());
        self.label_counter += 1;
        self.file.write_all("0;JMP\n".as_bytes());

        self.file.write_all(
            ("(LABEL".to_string() + &((self.label_counter - 2).to_string()) + ")\n").as_bytes(),
        );
        self.file.write_all("@1\n".as_bytes());
        self.file.write_all("D=-A\n".as_bytes());
        self.file.write_all(
            ("(LABEL".to_string() + &((self.label_counter - 1).to_string()) + ")\n").as_bytes(),
        );

        self.sp_sub1();
        self.write_d_to_stack();
    }

    pub fn close() {}
}
