use crate::jack_tokenizer::JackTokenizer;
use crate::jack_tokenizer::Token::{Identifier, IntegerConstant, Keyword, StringConstant, Symbol};
use crate::jack_tokenizer::{Keywords, Symbols, Token};

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

    pub fn compile_class(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<class>\n".as_bytes())?;
        self.write_token_and_advance();

        // write class name
        self.write_token_and_advance();
        // write "{"
        self.write_token_and_advance();

        while Self::is_class_var_dec_token(self.tokenizer.token_type()) {
            Self::compile_class_var_dec(&self);
        }

        while Self::is_subroutine_dec_token(self.tokenizer.token_type()) {
            Self::compile_subroutine(&self);
        }

        // write "}"
        self.write_token_and_advance();

        self.file.write_all("</class>\n".as_bytes())?;

        Ok(())
    }

    fn compile_class_var_dec(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<classVarDec>\n".as_bytes())?;

        // write ('static' | 'field')
        self.tokenizer.write_current_token(&mut self.file);
        // write type
        self.tokenizer.write_current_token(&mut self.file);
        // write varName
        self.tokenizer.write_current_token(&mut self.file);

        while self.has_more_var() {
            // write ","
            self.write_token_and_advance();
            // write var name
            self.write_token_and_advance();
        }

        // write ";"
        self.write_token_and_advance();

        self.file.write_all("</classVarDec>\n".as_bytes())?;
        Ok(())
    }
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

    fn write_token_and_advance(&mut self) {
        self.tokenizer.write_current_token(&mut self.file);
        self.tokenizer.advance();
    }

    fn is_class_var_dec_token(token: Token) -> bool {
        match token {
            Keyword(Keywords::Static) | Keyword(Keywords::Field) => true,
            _ => false,
        }
    }

    fn is_subroutine_dec_token(token: Token) -> bool {
        match token {
            Keyword(Keywords::Constructor)
            | Keyword(Keywords::Function)
            | Keyword(Keywords::Method) => true,
            _ => false,
        }
    }

    fn has_more_var(&self) -> bool {
        match self.tokenizer.token_type() {
            Symbol(Symbols::Comma(_)) => true,
            _ => false,
        }
    }
}
