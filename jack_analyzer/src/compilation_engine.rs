use std::{fs, io};

pub struct CompilationEngine {
    pub tokens: Vec<String>,
    pub code_index: usize,
}

impl CompilationEngine {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let token_strings = fs::read_to_string(file_name)?;
        let tokens = token_strings
            .split("\n")
            .map(|token| token.to_string())
            .collect();
        Ok(CompilationEngine {
            tokens,
            code_index: 0,
        })
    }

    pub fn complie(&self) {
        for token in &self.tokens {
            match token.as_str() {
                "<token>" => (),
                "<keyword>" => (),
                "<symbol>" => (),
                "<integerConstant>" => (),
                "<stringConstant>" => (),
                "<identifier>" => (),
                _ => (),
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
