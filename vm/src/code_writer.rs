use std::fs::File;
use std::io::{self, Write};

use crate::parser::CommandType;

pub struct CodeWriter {
    file: File,
    filename: String,
    label_counter: usize,
}

impl CodeWriter {
    pub fn new(filename: &str) -> Result<Self, io::Error> {
        let striped_filename = filename
            .replace(".asm", "")
            .rsplit('/')
            .next()
            .unwrap()
            .to_string();
        match File::create(filename) {
            Ok(file) => Ok(CodeWriter {
                file,
                filename: striped_filename,
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

    fn write_init(&mut self) {}

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
                self.pop_to_d();
                self.file.write_all("M=!D\n".as_bytes());
                self.sp_add1();
            }
            "neg" => {
                self.pop_to_d();
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

    pub fn write_push_pop(&mut self, command_type: CommandType, segment: &str, index: usize) {
        match command_type {
            CommandType::CPush => match segment {
                "constant" => {
                    self.file
                        .write_all(("@".to_string() + &index.to_string() + "\n").as_bytes());
                    self.file.write_all("D=A\n".as_bytes());

                    self.write_d_to_stack();
                }
                "local" => self.push("LCL", index),
                "argument" => self.push("ARG", index),
                "this" => self.push("THIS", index),
                "that" => self.push("THAT", index),
                "pointer" => self.push("pointer", index),
                "temp" => self.push("temp", index),
                "static" => self.push("static", index),
                _ => (),
            },
            CommandType::CPop => match segment {
                "local" => self.pop("LCL", index),
                "argument" => self.pop("ARG", index),
                "this" => self.pop("THIS", index),
                "that" => self.pop("THAT", index),
                "pointer" => self.pop("pointer", index),
                "temp" => self.pop("temp", index),
                "static" => self.pop("static", index),
                _ => (),
            },
            _ => (),
        }
    }

    pub fn write_label(&mut self, label_name: &str) {
        self.file
            .write_all(("(".to_string() + label_name + ")\n").as_bytes());
        // self.label_counter += 1;
    }

    pub fn write_goto(&mut self, label_name: &str) {
        self.file
            .write_all(("@".to_string() + label_name + "\n").as_bytes());
        self.file.write_all("0;JMP\n".as_bytes());
    }

    pub fn write_if(&mut self, label_name: &str) {
        self.pop_to_d();
        self.file
            .write_all(("@".to_string() + label_name + "\n").as_bytes());
        self.file.write_all("D;JNE\n".as_bytes());
    }

    pub fn write_function(&mut self, func_name: &str, local_num: usize) {
        self.file
            .write_all(("(".to_string() + func_name + ")\n").as_bytes());
        self.file.write_all("D=0\n".as_bytes());
        for _ in 0..local_num {
            self.write_d_to_stack();
        }
    }

    pub fn write_return(&mut self) {
        self.file.write_all("@LCL\n".as_bytes());
        self.file.write_all("D=M\n".as_bytes());
        self.file.write_all("@FRAME\n".as_bytes());
        self.file.write_all("M=D\n".as_bytes());

        self.pop_to_d();
        self.write_d_to_pointed("ARG");

        self.file.write_all("@ARG\n".as_bytes());
        self.file.write_all("A=M\n".as_bytes());
        self.file.write_all("A=A+1\n".as_bytes());
        self.file.write_all("D=A\n".as_bytes());

        self.write_from_d("SP");

        self.sub1("FRAME");
        self.write_pointed_to_d("FRAME");
        self.write_from_d("THAT");

        self.sub1("FRAME");
        self.write_pointed_to_d("FRAME");
        self.write_from_d("THIS");

        self.sub1("FRAME");
        self.write_pointed_to_d("FRAME");
        self.write_from_d("ARG");

        self.sub1("FRAME");
        self.write_pointed_to_d("FRAME");
        self.write_from_d("LCL");

        self.sub1("FRAME");
        self.write_pointed_to_d("FRAME");
        self.file.write_all("@RET\n".as_bytes());
        self.file.write_all("M=D\n".as_bytes());

        self.file.write_all("A=M\n".as_bytes());
        self.file.write_all("0;JMP\n".as_bytes());
    }

    pub fn write_call(&mut self, func_name: &str, arg_num: usize) {
        self.file
            .write_all(("@".to_string() + func_name + "_RET\n").as_bytes());
        self.file.write_all("D=A\n".as_bytes());
        self.write_d_to_stack();

        self.write_to_d("LCL");
        self.write_d_to_stack();

        self.write_to_d("ARG");
        self.write_d_to_stack();

        self.write_to_d("THIS");
        self.write_d_to_stack();

        self.write_to_d("THAT");
        self.write_d_to_stack();

        self.write_to_d("SP");
        for _ in 0..(arg_num + 5) {
            self.file.write_all("D=D-1\n".as_bytes());
        }

        self.write_from_d("ARG");

        self.write_to_d("SP");
        self.write_from_d("LCL");

        // self.write_goto(&(func_name.to_string() + &self.label_counter.to_string()));
        // self.write_goto(&(func_name.to_string() + "_RET"));
        self.write_goto(func_name);

        // self.write_label(&(func_name.to_string() + &self.label_counter.to_string()));
        self.write_label(&(func_name.to_string() + "_RET"));
    }

    fn add1(&mut self, dest: &str) {
        self.file
            .write_all(("@".to_string() + dest + "\n").as_bytes());
        self.file.write_all("M=M+1\n".as_bytes());
    }

    fn sub1(&mut self, dest: &str) {
        self.file
            .write_all(("@".to_string() + dest + "\n").as_bytes());
        self.file.write_all("M=M-1\n".as_bytes());
    }

    fn sp_add1(&mut self) {
        self.add1("SP");
    }

    fn sp_sub1(&mut self) {
        self.sub1("SP");
    }

    fn push(&mut self, dest: &str, offset: usize) {
        match dest {
            "pointer" | "temp" => {
                let base_address = if dest == "pointer" { 3 } else { 5 };

                self.file.write_all(
                    ("@".to_string() + &((base_address + offset).to_string()) + "\n").as_bytes(),
                );
                self.file.write_all("D=M\n".as_bytes());
                self.write_d_to_stack();
            }
            "static" => {
                self.file.write_all(
                    ("@".to_string() + &self.filename + "." + &offset.to_string() + "\n")
                        .as_bytes(),
                );
                self.file.write_all("D=M\n".as_bytes());
                self.write_d_to_stack();
            }
            _ => {
                self.file
                    .write_all(("@".to_string() + dest + "\n").as_bytes());
                self.file.write_all("D=M\n".as_bytes());

                self.file
                    .write_all(("@".to_string() + &offset.to_string() + "\n").as_bytes());
                self.file.write_all("D=D+A\n".as_bytes());

                self.file.write_all("A=D\n".as_bytes());
                self.file.write_all("D=M\n".as_bytes());

                self.write_d_to_stack();
            }
        }
    }

    fn pop(&mut self, dest: &str, offset: usize) {
        match dest {
            "pointer" | "temp" => {
                let base_address = if dest == "pointer" { 3 } else { 5 };

                self.pop_to_d();

                self.file.write_all(
                    ("@".to_string() + &((base_address + offset).to_string()) + "\n").as_bytes(),
                );
                self.file.write_all("M=D\n".as_bytes());
            }
            "static" => {
                self.pop_to_d();

                self.file.write_all(
                    ("@".to_string() + &self.filename + "." + &offset.to_string() + "\n")
                        .as_bytes(),
                );
                self.file.write_all("M=D\n".as_bytes());
            }
            _ => {
                self.file
                    .write_all(("@".to_string() + dest + "\n").as_bytes());
                self.file.write_all("D=M\n".as_bytes());

                self.file
                    .write_all(("@".to_string() + &offset.to_string() + "\n").as_bytes());
                self.file.write_all("D=D+A\n".as_bytes());

                self.file.write_all("@R13\n".as_bytes());
                self.file.write_all("M=D\n".as_bytes());

                self.pop_to_d();

                self.write_d_to_pointed("R13");
            }
        }
    }

    fn pop_to_d(&mut self) {
        self.sp_sub1();
        self.write_pointed_to_d("SP");
    }

    fn write_d_to_stack(&mut self) {
        self.write_d_to_pointed("SP");
        self.sp_add1();
    }

    fn write_to_d(&mut self, from: &str) {
        self.file
            .write_all(("@".to_string() + from + "\n").as_bytes());
        self.file.write_all("D=M\n".as_bytes());
    }

    fn write_from_d(&mut self, to: &str) {
        self.file
            .write_all(("@".to_string() + to + "\n").as_bytes());
        self.file.write_all("M=D\n".as_bytes());
    }

    fn write_pointed_to_d(&mut self, from: &str) {
        self.file
            .write_all(("@".to_string() + from + "\n").as_bytes());
        self.file.write_all("A=M\n".as_bytes());
        self.file.write_all("D=M\n".as_bytes());
    }

    fn write_d_to_pointed(&mut self, to: &str) {
        self.file
            .write_all(("@".to_string() + to + "\n").as_bytes());
        self.file.write_all("A=M\n".as_bytes());
        self.file.write_all("M=D\n".as_bytes());
    }

    fn write_arithmetic_to_d(&mut self, op: &str) {
        self.sp_sub1();
        self.pop_to_d();
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
