use crate::jack_tokenizer::JackTokenizer;
use crate::jack_tokenizer::Keywords;
use crate::jack_tokenizer::Token;
use crate::jack_tokenizer::Token::{Identifier, IntegerConstant, Keyword, StringConstant, Symbol};

use std::fs::File;
use std::io::Write;
use std::{fs, io};

pub struct CompilationEngine {
    pub file: File,
    pub tokenizer: JackTokenizer,
}

impl CompilationEngine {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let tokenizer = JackTokenizer::new(file_name)?;
        Ok(CompilationEngine {
            file: File::create(file_name)?,
            tokenizer,
        })
    }

    pub fn complie(&self) {
        while self.tokenizer.has_more_tokens() {
            match self.tokenizer.token_type() {
                Keyword(keyword) => match keyword {
                    Keywords::Class => self.compile_class(keyword),
                    Keywords::Constructor | Keywords::Function | Keywords::Method => {
                        self.compile_subroutine()
                    }
                    //TODO: write appropriate function
                    Keywords::Void => (),
                    Keywords::Field | Keywords::Static => self.compile_class_var_dec(),
                    Keywords::Var => (),
                    Keywords::Int | Keywords::Char | Keywords::Boolean => (),
                    Keywords::True | Keywords::False | Keywords::Null | Keywords::This => (),
                    Keywords::Let => (),
                    Keywords::Do => (),
                    Keywords::If => (),
                    Keywords::Else => (),
                    Keywords::While => (),
                    Keywords::Return => (),
                },
                Symbol(symbol) => (),
                IntegerConstant(value) => (),
                StringConstant(value) => (),
                Identifier(identifier) => (),
            }
        }
    }

    fn compile_class(&mut self, keyword: Keywords) {
        self.file.write_all("<class>\n".as_bytes());
        self.tokenizer.write_current_token(file)
    }
    fn compile_class_var_dec(&self) {}
    fn compile_subroutine(&self) {}
    fn compile_parameter_list(&self) {}
    fn compile_var_dec(&self) {}
    fn compile_statements(&self) {}
    fn compile_do(&self) {}
    fn compile_let(&self) {}
    fn compile_while(&self) {}
    fn compile_return(&self) {}
    fn compile_if(&self) {}
    fn compile_expression(&self) {}
    fn compile_term(&self) {}
    fn compile_expression_list(&self) {}
}
