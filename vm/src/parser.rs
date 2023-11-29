use std::{fmt::Error, fs::File, io::BufRead, io::BufReader};

pub struct Parser {
    pub commands: Vec<String>,
    pub commnad_number: usize,
}

pub enum CommandType {
    CArithmetic,
    CPush,
    CPop,
    CLabel,
    CGoto,
    CIf,
    CFunction,
    CReturn,
    CCall,
}

impl Parser {
    pub fn new(filename: &str) -> Result<Self, Error> {
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let commands: Vec<String> = reader
                    .lines()
                    .map(|line| line.unwrap().replace("\n", ""))
                    .filter(|line| !Parser::is_commnet_or_whitespace(&line))
                    .collect();
                Ok(Parser {
                    commands,
                    commnad_number: 0,
                })
            }
            _ => Err(Error),
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.commnad_number < self.commands.len()
    }

    pub fn advance(&mut self) {
        self.commnad_number += 1;
    }

    pub fn command_type(&self) -> CommandType {
        if self.command().starts_with("push") {
            CommandType::CPush
        } else if self.command().starts_with("pop") {
            CommandType::CPop
        } else if self.command().starts_with("label") {
            CommandType::CLabel
        } else if self.command().starts_with("goto") {
            CommandType::CGoto
        } else if self.command().starts_with("if") {
            CommandType::CIf
        } else if self.command().starts_with("function") {
            CommandType::CFunction
        } else if self.command().starts_with("return") {
            CommandType::CReturn
        } else if self.command().starts_with("call") {
            CommandType::CCall
        } else {
            CommandType::CArithmetic
        }
    }

    pub fn arg1(&self) -> String {
        match self.command_type() {
            CommandType::CArithmetic => {
                self.command().split(" ").collect::<Vec<&str>>()[0].to_string()
            }
            _ => self.command().split(" ").collect::<Vec<&str>>()[1].to_string(),
        }
    }

    pub fn arg2(&self) -> i32 {
        self.command().split(" ").collect::<Vec<&str>>()[2]
            .parse()
            .unwrap()
    }

    fn is_commnet_or_whitespace(command: &str) -> bool {
        command.starts_with("//") || command == ""
    }

    pub fn command(&self) -> String {
        self.commands[self.commnad_number - 1].clone()
    }
}
