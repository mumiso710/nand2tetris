use std::{fs, io};

use jack_tokenizer::JackTokenizer;

pub struct CompilationEngine {
    pub tokenizer: JackTokenizer,
}

impl CompilationEngine {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let tokenizer = JackTokenizer::new(file_name)?;
        Ok(CompilationEngine { tokenizer })
    }

    pub fn complie(&self) {
        while self.tokenizer.has_more_tokens() {
            match self.tokenizer.token_type() {
                Token::Keyword() => (),
                JackTokenizer::Symbol => (),
                JackTokenizer::IntegerConstant => (),
                JackTokenizer::StringConstant => (),
                JackTokenizer::Identifier => (),
            }
        }
    }

    fn compile_class(&self) {}
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
