use std::collections::HashMap;

use crate::expression::Value;

pub enum Entry {
    Pointer(u64),
    Val(Value),
}

pub struct SymbolTable {
    global_counter: usize,
    storage: HashMap<u64, Entry>,
    front: HashMap<String, u64>,
}

impl SymbolTable {
    fn get_from_addr(&self, addr: u64, level: u16) -> Result<Value, &'static str> {
        if level < 128 {
            if addr == 0 {
                return Result::Err("Null pointer dereference!");
            }

            if self.storage.contains_key(&addr) {
                let o_entry: Option<&Entry> = self.storage.get(&addr);

                match o_entry {
                    Option::Some(ent) => match ent {
                        Entry::Val(vale) => return Result::Ok(vale.clone()),
                        Entry::Pointer(addr2) => self.get_from_addr(*addr2, level + 1),
                    },
                    _ => return Result::Err("Adress does not exist!"),
                }
            } else {
                return Result::Err("Adress does not exist!");
            }
        } else {
            return Result::Err("Nested dereferencing hit hard limit of 128!");
        }
    }

    fn get_from_symbol(&self, symbol: &str) -> Result<Value, &'static str> {
        if self.front.contains_key(symbol) {
            let r_addr: Option<&u64> = self.front.get(symbol);
            match r_addr {
                Option::Some(addr) => {
                    return self.get_from_addr(*addr, 0);
                }
                _ => {
                    return Result::Err("Symbol does not point to anything!");
                }
            }
        } else {
            return Result::Err("Symbol does not exist!");
        }
    }
}
