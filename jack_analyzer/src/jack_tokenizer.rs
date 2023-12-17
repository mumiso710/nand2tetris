use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub struct JackTokenizer {
    pub tokens: Vec<String>,
    pub token_index: usize,
}

pub enum Keywords {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

pub enum Symbols {
    LCurly,
    RCurly,
    LParen,
    RParen,
    LSquare,
    RSquare,
    Period,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Mult,
    Div,
    And,
    Or,
    Less,
    Greater,
    Eq,
    Not,
}

pub enum Token {
    Keyword(Keywords),
    Symbol(Symbols),
    IntegerConstant(usize),
    StringConstant(String),
    Identifier(String),
}

impl JackTokenizer {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);
        let tokens: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap().trim().to_string();
            let line_removed_inline_comment =
                line.split("//").next().unwrap_or("").trim().to_string();
            tokens.push(line);
        }
        Ok(JackTokenizer {
            tokens: Vec::new(),
            token_index: 0,
        })
    }

    pub fn has_more_tokens(&self) -> bool {
        true
    }

    pub fn advance(&self) {}

    pub fn token_type(&self) -> Token {}

    pub fn keyword(&self) -> Keywords {}

    pub fn symbol(&self) -> Symbols {}

    pub fn identifier(&self) -> Identifier {}

    pub fn int_val(&self) -> usize {}

    pub fn string_val(&self) -> String {}
}
