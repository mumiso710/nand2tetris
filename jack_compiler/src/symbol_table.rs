use std::{collections::HashMap, fmt};

use crate::jack_tokenizer::{Keywords, Token};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Kind {
    STATIC,
    FIELD,
    ARG,
    VAR,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::STATIC => write!(f, "static"),
            Self::FIELD => write!(f, "field"),
            Self::VAR => write!(f, "var"),
            Self::ARG => write!(f, "arg"),
        }
    }
}

pub struct Symbol {
    symbol_name: String,
    symbol_type: String,
    symbol_kind: Kind,
    symbol_index: usize,
}

pub struct SymbolTable {
    class_table: HashMap<String, Symbol>,
    class_kind_counter: HashMap<Kind, usize>,
    subroutine_table: HashMap<String, Symbol>,
    subroutine_kind_counter: HashMap<Kind, usize>,
    current_symbol_type: String,
    current_symbol_kind: Kind,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut class_kind_counter = HashMap::new();
        class_kind_counter.insert(Kind::STATIC, 0);
        class_kind_counter.insert(Kind::FIELD, 0);
        let mut subroutine_kind_counter = HashMap::new();
        subroutine_kind_counter.insert(Kind::ARG, 0);
        subroutine_kind_counter.insert(Kind::VAR, 0);
        SymbolTable {
            class_table: HashMap::new(),
            class_kind_counter,
            subroutine_table: HashMap::new(),
            subroutine_kind_counter,
            current_symbol_type: "".to_string(),
            current_symbol_kind: Kind::STATIC,
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_table = HashMap::new();
    }

    pub fn define(&mut self, name: String, var_type: String, kind: Kind) {
        match kind {
            Kind::STATIC | Kind::FIELD => {
                self.class_table.insert(
                    name.to_string(),
                    Symbol {
                        symbol_name: name,
                        symbol_type: var_type,
                        symbol_kind: kind,
                        symbol_index: self.var_count(kind),
                    },
                );
                *self.class_kind_counter.entry(kind).or_insert(0) += 1;
            }
            Kind::ARG | Kind::VAR => {
                self.subroutine_table.insert(
                    name.to_string(),
                    Symbol {
                        symbol_name: name,
                        symbol_type: var_type,
                        symbol_kind: kind,
                        symbol_index: self.var_count(kind),
                    },
                );
                *self.subroutine_kind_counter.entry(kind).or_insert(0) += 1;
            }
        }
    }

    pub fn var_count(&self, kind: Kind) -> usize {
        match kind {
            Kind::STATIC | Kind::FIELD => *self.class_kind_counter.get(&kind).unwrap(),

            Kind::ARG | Kind::VAR => *self.subroutine_kind_counter.get(&kind).unwrap(),
        }
    }

    pub fn kind_of(&self, name: &str) -> Option<&Kind> {
        let symbol = self.get_symbol(name);
        match symbol {
            Some(_) => Some(&symbol.unwrap().symbol_kind),
            None => None,
        }
    }

    pub fn type_of(&self, name: &str) -> Option<&String> {
        let symbol = self.get_symbol(name);
        match symbol {
            Some(_) => Some(&symbol.unwrap().symbol_type),
            None => None,
        }
    }
    pub fn index_of(&self, name: &str) -> Option<&usize> {
        let symbol = self.get_symbol(name);
        match symbol {
            Some(_) => Some(&symbol.unwrap().symbol_index),
            None => None,
        }
    }

    fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        let subrountine_result = self.subroutine_table.get(name);
        let class_result = self.class_table.get(name);
        match subrountine_result {
            Some(_) => subrountine_result,
            None => match class_result {
                Some(_) => class_result,
                None => None,
            },
        }
    }

    pub fn set_token_type(&mut self, token: &Token) {
        match token {
            Token::Keyword(Keywords::Int) => self.current_symbol_type = "int".to_string(),
            Token::Keyword(Keywords::Char) => self.current_symbol_type = "char".to_string(),
            Token::Keyword(Keywords::Boolean) => self.current_symbol_type = "boolean".to_string(),
            Token::Identifier(name) => self.current_symbol_type = name.to_string(),
            _ => (),
        }
    }
    pub fn set_token_kind(&mut self, token: &Token) {
        match token {
            Token::Keyword(Keywords::Static) => self.current_symbol_kind = Kind::STATIC,
            Token::Keyword(Keywords::Field) => self.current_symbol_kind = Kind::FIELD,
            Token::Keyword(Keywords::Var) => self.current_symbol_kind = Kind::VAR,
            _ => self.current_symbol_kind = Kind::ARG,
        }
    }

    pub fn get_current_token_type(&self) -> String {
        self.current_symbol_type.clone()
    }

    pub fn get_current_token_kind(&self) -> Kind {
        self.current_symbol_kind.clone()
    }

    pub fn contains(&self, key: &String) -> bool {
        self.class_table.contains_key(key) || self.subroutine_table.contains_key(key)
    }
}
