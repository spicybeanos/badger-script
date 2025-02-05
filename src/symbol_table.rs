use std::collections::HashMap;

use crate::{
    badger_debug::{get_col, get_line_from_index,error},
    expression::Value,
};

pub enum Entry {
    Pointer(u64),
    Val(Value),
}

pub struct SymbolTable {
    global_counter: u64,
    storage: HashMap<u64, Entry>,
    front: HashMap<String, u64>,
}

impl SymbolTable {
    pub fn add_symbol(
        &mut self,
        name: &str,
        value: Value,
        index: &usize,
        lines: &Vec<usize>,
    ) -> Result<u64, String> {
        if self.front.contains_key(name) {
            let l = get_line_from_index(lines, index);
            let c = get_col(index, lines);
            return Result::Err(format!("{} at line {}, {}", "Identifier already decleared", l, c));
        }

        self.storage.insert(self.global_counter, Entry::Val(value));
        self.front.insert(name.to_owned(), self.global_counter);
        self.global_counter = self.global_counter + 1;

        return Ok(self.global_counter - 1);
    }
    pub fn new() -> SymbolTable {
        SymbolTable {
            global_counter: 1000,
            storage: HashMap::<u64, Entry>::new(),
            front: HashMap::<String, u64>::new(),
        }
    }
    pub fn get_from_addr(
        &self,
        addr: u64,
        level: u16,
        index: &usize,
        lines: &Vec<usize>,
    ) -> Result<Value, String> {
        if level < 128 {
            if addr == 0 {
                return Result::Err("Null pointer dereference!".to_owned());
            }

            if self.storage.contains_key(&addr) {
                let o_entry: Option<&Entry> = self.storage.get(&addr);

                match o_entry {
                    Option::Some(ent) => match ent {
                        Entry::Val(vale) => return Result::Ok(vale.clone()),
                        Entry::Pointer(addr2) => {
                            self.get_from_addr(*addr2, level + 1, index, lines)
                        }
                    },
                    _ => return error("Adress does not exist!", index, lines),
                }
            } else {
                return error("Adress does not exist!", index, lines);
            }
        } else {
            return error("Nested dereferencing hit hard limit!", index, lines);
        }
    }

    pub fn get_from_symbol(
        &self,
        symbol: &str,
        index: &usize,
        lines: &Vec<usize>,
    ) -> Result<Value, String> {
        if self.front.contains_key(symbol) {
            let r_addr: Option<&u64> = self.front.get(symbol);
            match r_addr {
                Option::Some(addr) => {
                    return self.get_from_addr(*addr, 0, index, lines);
                }
                _ => {
                    return error("Symbol does not point to anything!", index, lines);
                }
            }
        } else {
            return error("Symbol does not exist!", index, lines);
        }
    }
}
