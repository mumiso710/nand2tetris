use crate::jack_tokenizer::JackTokenizer;
use crate::jack_tokenizer::Token::{Identifier, IntegerConstant, Keyword, StringConstant, Symbol};
use crate::jack_tokenizer::{Keywords, Symbols, Token};

use std::fs::File;
use std::io;
use std::io::Write;

pub struct CompilationEngine {
    pub file: File,
    pub tokenizer: JackTokenizer,
}

impl CompilationEngine {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let tokenizer = JackTokenizer::new(file_name)?;
        let file_name = file_name.replace(".jack", "") + "_compile.xml";
        Ok(CompilationEngine {
            file: File::create(file_name)?,
            tokenizer,
        })
    }

    pub fn compile_class(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<class>\n".as_bytes())?;
        self.write_token_and_advance()?;

        // write class name
        self.write_token_and_advance()?;
        // write "{"
        self.write_token_and_advance()?;

        while Self::is_class_var_dec_token(self.tokenizer.token_type()) {
            Self::compile_class_var_dec(self)?;
        }

        while Self::is_subroutine_dec_token(self.tokenizer.token_type()) {
            Self::compile_subroutine(self)?;
        }

        // write "}"
        self.write_token_and_advance()?;

        self.file.write_all("</class>\n".as_bytes())?;

        Ok(())
    }

    fn compile_class_var_dec(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<classVarDec>\n".as_bytes())?;

        while !Self::is_semicolon(self.tokenizer.token_type()) {
            self.write_token_and_advance()?;
        }

        // write ";"
        self.write_token_and_advance()?;

        self.file.write_all("</classVarDec>\n".as_bytes())?;
        Ok(())

        //        // write ('static' | 'field')
        //        self.tokenizer.write_current_token(&mut self.file)?;
        //        // write type
        //        self.tokenizer.write_current_token(&mut self.file)?;
        //        // write varName
        //        self.tokenizer.write_current_token(&mut self.file)?;
        //
        //        while self.has_more_var() {
        //            // write ","
        //            self.write_token_and_advance()?;
        //            // write var name
        //            self.write_token_and_advance()?;
        //        }
    }
    fn compile_subroutine(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<subroutineDec>\n".as_bytes())?;
        // write ('constrctor' | 'function' | 'method')
        self.write_token_and_advance()?;
        // write ('void' | type)
        self.write_token_and_advance()?;
        // write subroutine name
        self.write_token_and_advance()?;
        // write '('
        self.write_token_and_advance()?;

        self.compile_parameter_list()?;

        // write ')'
        self.write_token_and_advance()?;

        self.file.write_all("<subroutineBody>\n".as_bytes())?;
        // write '{'
        self.write_token_and_advance()?;

        while !Self::is_right_curly(self.tokenizer.token_type()) {
            match self.tokenizer.token_type() {
                Keyword(Keywords::Var) => self.compile_var_dec()?,
                _ => self.compile_statements()?,
            };
        }

        // write '}'
        self.write_token_and_advance()?;
        self.file.write_all("</subroutineBody>\n".as_bytes())?;
        self.file.write_all("</subroutineDec>\n".as_bytes())?;
        Ok(())
    }
    fn compile_parameter_list(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<parameterList>\n".as_bytes())?;
        while !Self::is_right_paran(self.tokenizer.token_type()) {
            self.write_token_and_advance()?;
        }
        // write ')'
        self.file.write_all("</parameterList>\n".as_bytes())?;
        Ok(())
    }
    fn compile_var_dec(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<varDec>\n".as_bytes())?;
        while !Self::is_semicolon(self.tokenizer.token_type()) {
            self.write_token_and_advance()?;
        }
        // write ')'
        self.write_token_and_advance()?;
        self.file.write_all("</varDec>\n".as_bytes())?;
        Ok(())
    }
    fn compile_statements(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<statements>\n".as_bytes())?;
        while Self::is_statement(self.tokenizer.token_type()) {
            match self.tokenizer.token_type() {
                Keyword(Keywords::Let) => self.compile_let()?,
                Keyword(Keywords::If) => self.compile_if()?,
                Keyword(Keywords::While) => self.compile_while()?,
                Keyword(Keywords::Do) => self.compile_do()?,
                Keyword(Keywords::Return) => self.compile_return()?,
                _ => (),
            };
        }
        self.file.write_all("</statements>\n".as_bytes())?;
        Ok(())
    }

    fn compile_let(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<letStatement>\n".as_bytes())?;

        // write let
        self.write_token_and_advance()?;
        // write var name
        self.write_token_and_advance()?;

        if Self::is_left_square(self.tokenizer.token_type()) {
            // write '('
            self.write_token_and_advance()?;
            self.compile_expression()?;
            // write ')'
            self.write_token_and_advance()?;
        }
        // write '='
        self.write_token_and_advance()?;
        self.compile_expression()?;

        // write ";"
        self.write_token_and_advance()?;
        self.file.write_all("</letStatement>\n".as_bytes())?;
        Ok(())
    }

    fn compile_if(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<ifStatement>\n".as_bytes())?;

        // write if
        self.write_token_and_advance()?;
        // write '('
        self.write_token_and_advance()?;
        self.compile_expression()?;
        // write ')'
        self.write_token_and_advance()?;
        // write '{'
        self.write_token_and_advance()?;
        self.compile_statements()?;
        // write '}'
        self.write_token_and_advance()?;

        if !Self::is_semicolon(self.tokenizer.token_type()) {
            // write else
            self.write_token_and_advance()?;
            // write '{'
            self.write_token_and_advance()?;
            self.compile_statements()?;
            // write '}'
            self.write_token_and_advance()?;
        }
        self.file.write_all("</ifStatement>\n".as_bytes())?;
        Ok(())
    }
    fn compile_do(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<doStatement>\n".as_bytes())?;
        // write "do"
        self.write_token_and_advance()?;

        // write subroutine name or (className | varName)
        self.write_token_and_advance()?;

        if Self::is_left_paran(self.tokenizer.token_type()) {
            // write '('
            self.write_token_and_advance()?;
            self.compile_expression_list()?;
            // write ')'
            self.write_token_and_advance()?;
        } else {
            // write '.'
            self.write_token_and_advance()?;
            // write subroutine name
            self.write_token_and_advance()?;
            // write '('
            self.write_token_and_advance()?;
            self.compile_expression_list()?;
            // write ')'
            self.write_token_and_advance()?;
        }

        // write ';'
        self.write_token_and_advance()?;
        self.file.write_all("</doStatement>\n".as_bytes())?;
        Ok(())
    }
    fn compile_while(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<whileStatement>\n".as_bytes())?;

        // write "("
        self.write_token_and_advance()?;
        self.compile_expression()?;
        // write ")"
        self.write_token_and_advance()?;

        // write "{"
        self.write_token_and_advance()?;
        self.compile_statements()?;
        // write "}"
        self.write_token_and_advance()?;
        self.file.write_all("</whileStatement>\n".as_bytes())?;
        Ok(())
    }
    fn compile_return(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<returnStatement>\n".as_bytes())?;
        // write return
        self.write_token_and_advance()?;
        if !Self::is_semicolon(self.tokenizer.token_type()) {
            self.compile_expression()?;
        }
        // write ";"
        self.write_token_and_advance()?;
        self.file.write_all("</returnStatement>\n".as_bytes())?;
        Ok(())
    }
    fn compile_expression(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<expression>\n".as_bytes())?;
        self.compile_term()?;
        while Self::is_op_token(self.tokenizer.token_type()) {
            self.write_token_and_advance()?;
            self.compile_term()?;
        }
        self.file.write_all("</expression>\n".as_bytes())?;

        Ok(())
    }
    fn compile_term(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<term>\n".as_bytes())?;
        self.write_token_and_advance()?;
        self.file.write_all("</term>\n".as_bytes())?;

        Ok(())
    }
    fn compile_expression_list(&mut self) -> Result<(), io::Error> {
        self.file.write_all("<expressionList>\n".as_bytes())?;
        self.file.write_all("</expressionList>\n".as_bytes())?;
        Ok(())
    }

    fn write_token_and_advance(&mut self) -> Result<(), io::Error> {
        self.tokenizer.write_current_token(&mut self.file)?;
        self.tokenizer.advance();
        Ok(())
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

    fn is_return_token(token: Token) -> bool {
        match token {
            Keyword(Keywords::Return) => true,
            _ => false,
        }
    }

    fn is_left_paran(token: Token) -> bool {
        match token {
            Symbol(Symbols::LParen(_)) => true,
            _ => false,
        }
    }

    fn is_right_paran(token: Token) -> bool {
        match token {
            Symbol(Symbols::RParen(_)) => true,
            _ => false,
        }
    }

    fn is_semicolon(token: Token) -> bool {
        match token {
            Symbol(Symbols::Semicolon(_)) => true,
            _ => false,
        }
    }

    fn is_statement(token: Token) -> bool {
        match token {
            Keyword(Keywords::Let)
            | Keyword(Keywords::If)
            | Keyword(Keywords::While)
            | Keyword(Keywords::Do)
            | Keyword(Keywords::Return) => true,
            _ => false,
        }
    }

    fn is_right_curly(token: Token) -> bool {
        match token {
            Symbol(Symbols::RCurly(_)) => true,
            _ => false,
        }
    }

    fn is_left_square(token: Token) -> bool {
        match token {
            Symbol(Symbols::LSquare(_)) => true,
            _ => false,
        }
    }

    fn is_op_token(token: Token) -> bool {
        match token {
            Symbol(Symbols::Plus(_))
            | Symbol(Symbols::Minus(_))
            | Symbol(Symbols::Mult(_))
            | Symbol(Symbols::Div(_))
            | Symbol(Symbols::And(_))
            | Symbol(Symbols::Or(_))
            | Symbol(Symbols::Greater(_))
            | Symbol(Symbols::Less(_))
            | Symbol(Symbols::Eq(_)) => true,
            _ => false,
        }
    }
}
