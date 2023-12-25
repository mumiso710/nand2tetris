use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
};

pub struct JackTokenizer {
    pub tokens: Vec<Token>,
    pub code_index: usize,
}

const SYMBOLS: [char; 19] = [
    '{', '}', '(', ')', '[', ']', '.', ',', ';', '+', '-', '*', '/', '&', '|', '<', '>', '=', '~',
];

const KEYWORDS: [&str; 21] = [
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return",
];

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Token {
    Keyword(Keywords),
    Symbol(Symbols),
    IntegerConstant(usize),
    StringConstant(String),
    Identifier(String),
}

impl JackTokenizer {
    pub fn new(file_name: &str) -> Result<Self, io::Error> {
        let jack_code = fs::read_to_string(file_name)?;
        let jack_code_without_comments = JackTokenizer::remove_comments(&jack_code);

        let tokens = JackTokenizer::split_to_tokens(&jack_code_without_comments);
        println!("{:?}", tokens);
        Ok(JackTokenizer {
            tokens,
            code_index: 0,
        })
    }

    pub fn has_more_tokens(&self) -> bool {
        todo!()
    }

    pub fn advance(&self) {
        todo!()
    }

    fn remove_comments(jack_code: &String) -> String {
        // start with "//" is inline comment
        let mut is_in_inline_comment = false;
        // start with "/**" and end with "*/" is block comment
        let mut is_in_block_comment = false;

        let mut jack_code_without_comments = String::from("");

        let mut jack_code = jack_code.chars().peekable();

        while jack_code.peek() != None {
            let mut c = jack_code.next().unwrap();

            if c == '/' && jack_code.peek() == Some(&'/') {
                is_in_inline_comment = true;
            }

            if c == '/' && jack_code.peek() == Some(&'*') {
                is_in_block_comment = true;
            }

            if c == '\n' {
                is_in_inline_comment = false;
            }

            if c == '*' && jack_code.peek() == Some(&'/') && is_in_block_comment {
                jack_code.next();
                c = jack_code.next().unwrap();
                is_in_block_comment = false;
            }

            if !(is_in_inline_comment || is_in_block_comment) {
                jack_code_without_comments.push(c);
            }
        }

        jack_code_without_comments.trim().to_string()
    }

    fn split_to_tokens(jack_code: &String) -> Vec<Token> {
        let mut in_string = false;

        let mut jack_code = jack_code.chars().peekable();

        let mut tokens = Vec::new();

        let mut token = "".to_string();

        while jack_code.peek() != None {
            let c = jack_code.next().unwrap();

            if (c == ' ' || c == '\n') && !in_string {
                continue;
            }

            if c == '"' {
                in_string = !in_string;
                continue;
            }

            if JackTokenizer::is_symbol(c) && !in_string {
                tokens.push(JackTokenizer::make_symbol_token(c));
                token = "".to_string();
                continue;
            }

            token.push(c);

            if jack_code.peek() == Some(&'"') && in_string {
                in_string = !in_string;
                tokens.push(Token::StringConstant(token));
                jack_code.next();
                token = "".to_string();
                continue;
            }

            if JackTokenizer::is_symbol(*jack_code.peek().unwrap()) && !in_string {
                if let Ok(int_value) = token.parse::<usize>() {
                    tokens.push(Token::IntegerConstant(int_value));
                    token = "".to_string();
                    continue;
                }
                tokens.push(Token::Identifier(token));
                token = "".to_string();
                continue;
            }

            if (jack_code.peek() == Some(&' ') || jack_code.peek() == Some(&'\n')) && !in_string {
                if JackTokenizer::is_keyword(&token) {
                    tokens.push(JackTokenizer::make_keyword_token(&token));
                    token = "".to_string();
                    continue;
                }

                if let Ok(int_value) = token.parse::<usize>() {
                    tokens.push(Token::IntegerConstant(int_value));
                    token = "".to_string();
                    continue;
                }

                tokens.push(Token::Identifier(token));

                token = "".to_string();
            }
        }
        tokens
    }

    fn is_symbol(c: char) -> bool {
        SYMBOLS.contains(&c)
    }

    fn is_keyword(str: &String) -> bool {
        KEYWORDS.contains(&str.as_str())
    }

    fn make_symbol_token(symbol: char) -> Token {
        match symbol {
            '{' => Token::Symbol(Symbols::LCurly),
            '}' => Token::Symbol(Symbols::RCurly),
            '(' => Token::Symbol(Symbols::LParen),
            ')' => Token::Symbol(Symbols::RParen),
            '[' => Token::Symbol(Symbols::LSquare),
            ']' => Token::Symbol(Symbols::RSquare),
            '.' => Token::Symbol(Symbols::Period),
            ',' => Token::Symbol(Symbols::Comma),
            ';' => Token::Symbol(Symbols::Semicolon),
            '+' => Token::Symbol(Symbols::Plus),
            '-' => Token::Symbol(Symbols::Minus),
            '*' => Token::Symbol(Symbols::Mult),
            '/' => Token::Symbol(Symbols::Div),
            '&' => Token::Symbol(Symbols::And),
            '|' => Token::Symbol(Symbols::Or),
            '<' => Token::Symbol(Symbols::Less),
            '>' => Token::Symbol(Symbols::Greater),
            '=' => Token::Symbol(Symbols::Eq),
            '~' => Token::Symbol(Symbols::Not),
            _ => panic!("this is not symbol character"),
        }
    }

    fn make_keyword_token(keyword: &str) -> Token {
        match keyword {
            "class" => Token::Keyword(Keywords::Class),
            "constructor" => Token::Keyword(Keywords::Constructor),
            "function" => Token::Keyword(Keywords::Function),
            "method" => Token::Keyword(Keywords::Method),
            "field" => Token::Keyword(Keywords::Field),
            "static" => Token::Keyword(Keywords::Static),
            "var" => Token::Keyword(Keywords::Var),
            "int" => Token::Keyword(Keywords::Int),
            "char" => Token::Keyword(Keywords::Char),
            "boolean" => Token::Keyword(Keywords::Boolean),
            "void" => Token::Keyword(Keywords::Void),
            "true" => Token::Keyword(Keywords::True),
            "false" => Token::Keyword(Keywords::False),
            "null" => Token::Keyword(Keywords::Null),
            "this" => Token::Keyword(Keywords::This),
            "let" => Token::Keyword(Keywords::Let),
            "do" => Token::Keyword(Keywords::Do),
            "if" => Token::Keyword(Keywords::If),
            "else" => Token::Keyword(Keywords::Else),
            "while" => Token::Keyword(Keywords::While),
            "return" => Token::Keyword(Keywords::Return),
            _ => panic!("this string is not keyword"),
        }
    }

    fn create_token_xml_file() {}

    pub fn token_type(&self) -> Token {
        todo!();
    }

    pub fn keyword(&self) -> Keywords {
        todo!();
    }

    pub fn symbol(&self) -> Symbols {
        todo!();
    }

    pub fn identifier(&self) -> Token {
        todo!();
    }

    pub fn int_val(&self) -> usize {
        todo!();
    }

    pub fn string_val(&self) -> String {
        todo!();
    }
}
