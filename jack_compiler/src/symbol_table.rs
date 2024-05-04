use std::collections::HashMap;

pub enum Kind {
    STATIC,
    FIELED,
    ARG,
    VAR,
}
pub struct Symbol {
    symbol_name: String,
    symbol_type: String,
    symbol_kind: Kind,
    symbol_index: usize,
}

pub struct SymbolTable {
    pub class_table: HashMap,
    pub class_kind_counter: HashMap,
    pub subroutine_table: HashMap,
    pub subroutine_kind_counter: HashMap,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut class_kind_counter = HashMap::new();
        class_kind_counter.insert(Kind::STATIC, 0);
        class_kind_counter.insert(Kind::FIELED, 0);
        let mut subroutine_kind_counter = HashMap::new();
        subroutine_kind_counter.insert(Kind::ARG, 0);
        subroutine_kind_counter.insert(Kind::VAR, 0);
        SymbolTable {
            class_table: HashMap::new(),
            class_kind_counter,
            subroutine_table: HashMap::new(),
            subroutine_kind_counter,
        }
    }

    pub fn start_subroutine(&mut self) {
        self.subroutine_table = HashMap::new();
    }

    pub fn define(&self, name: &str, var_type: &str, kind: Kind) {
        match kind {
            Kind::STATICS | Kind::FIELED => {
                self.class_table.insert(
                    name.to_string(),
                    Symbol(name.to_string(), var_type, kind, var_count(kind)),
                );
                *self.class_kind_counter.entry(kind).or_insert(0) += 1;
            }
            Kind::ARG | Kind::VAR => {
                self.subroutine_kind_counter.insert(
                    name.to_string(),
                    Symbol(name.to_string(), var_type, kind, var_count(kind)),
                );
                *self.subroutine_kind_counter.entry(kind).or_insert(0) += 1
            }
        }
    }

    pub fn var_count(&self, kind: Kind) -> usize {
        match kind {
            Kind::STATICS | Kind::FIELED => self.class_kind_counter.get(&kind).unwrap(),

            Kind::ARG | Kind::VAR => self.subroutine_kind_counter.get(&kind).unwrap(),
        }
    }

    pub fn kind_of(&self, name: &str) -> Option<Kind> {
        let symbol = self.get_symbol(name);
        match symbol {
            Some(_) => symbol.symbol_kind,
            None => None,
        }
    }

    pub fn type_of(&self, name: &str) {
        let symbol = self.get_symbol(name);
        match symbol {
            Some(_) => symbol.symbol_type,
            None => None,
        }
    }
    pub fn index_of(&self, name: &str) {
        let symbol = self.get_symbol(name);
        match symbol {
            Some(_) => symbol.symbol_index,
            None => None,
        }
    }

    fn get_symbol(&self, name: &str) -> Option<Symbol> {
        let subrountine_result = self.subroutine_table.get(name);
        let class_result = self.class_table.get(name);
        match subrountine_result {
            Some(symbol) => symbol,
            None => match class_result {
                Some(symbol) => symbol,
                None => None,
            },
        }
    }
}
