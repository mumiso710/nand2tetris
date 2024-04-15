use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Parser {
    pub commands: Vec<String>,
    pub commnad_number: usize,
}

#[derive(Debug)]
pub enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFilename(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFilename(ref s) => {
                write!(f, "ParseError: invalid filename {}", s)
            }
            ParseError::ReadLineError => {
                write!(f, "ParseError: reading line error")
            }
        }
    }
}

impl Error for ParseError {}

impl Parser {
    pub fn new(filename: &str) -> Result<Parser, ParseError> {
        match File::open(filename) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut commands = Vec::new();
                for line in reader.lines() {
                    let line = line
                        .unwrap()
                        .replace("\n", "")
                        .replace("\r", "")
                        .replace(" ", "");
                    if !Parser::is_commnet_or_whitespace(&line) {
                        commands.push(line);
                    }
                }
                Ok(Parser {
                    commands,
                    commnad_number: 0,
                })
            }
            _ => {
                let err = ParseError::InvalidFilename(filename.to_string());
                Err(err)
            }
        }
    }

    pub fn has_more_commands(&self) -> bool {
        self.commnad_number < self.commands.len()
    }

    pub fn advance(&mut self) {
        self.commnad_number += 1;
    }

    pub fn symbol(&self) -> String {
        let command = self.command();
        match self.command_type() {
            CommandType::ACommand => command.replace("@", ""), //@XXX
            CommandType::LCommand => command.replace("(", "").replace(")", ""), //(XXX)
            _ => panic!("only call when command_type is A_COMMAND or L_COMMAND"),
        }
    }

    pub fn dest(&self) -> String {
        let command = self.command();
        if !command.contains("=") {
            return "".to_string();
        }
        let mut dest = Vec::new();
        for c in command.chars() {
            if c == '=' {
                break;
            }
            dest.push(c);
        }
        let dest: String = dest.into_iter().collect();
        dest
    }

    pub fn comp(&self) -> String {
        let mut command = self.command();
        let mut dest = self.dest();
        dest.push_str("=");
        command = command.replace(&dest, "");
        let mut comp = Vec::new();
        for c in command.chars() {
            if c == ';' {
                break;
            }
            comp.push(c);
        }
        let comp: String = comp.into_iter().collect();
        comp
    }

    pub fn jump(&self) -> String {
        let mut command = self.command();

        if !command.contains(";") {
            return "".to_string();
        }
        let mut dest_comp = self.dest();
        if dest_comp != "" {
            dest_comp.push_str("=");
        }
        dest_comp.push_str(&self.comp());
        dest_comp.push_str(";");
        command = command.replace(&dest_comp, "");
        let mut jump = Vec::new();
        for c in command.chars() {
            if c == ' ' {
                break;
            }
            jump.push(c);
        }
        let jump: String = jump.into_iter().collect();
        jump
    }

    fn is_commnet_or_whitespace(command: &str) -> bool {
        command.starts_with("//") || command == ""
    }

    pub fn command_type(&self) -> CommandType {
        if self.command().starts_with("@") {
            CommandType::ACommand
        } else if self.command().starts_with("(") {
            CommandType::LCommand
        } else {
            CommandType::CCommand
        }
    }

    pub fn command(&self) -> String {
        self.commands[self.commnad_number - 1].clone()
    }
}
