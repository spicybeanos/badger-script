use std::collections::HashMap;

use crate::expression::Value;

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
    pub fn add_symbol(&mut self,name:&str,value:Value) -> Result<u64,String> {
        
        if self.front.contains_key(name) {
            return  Err("Identifier already decleared!".to_owned());
        }

        self.storage.insert(self.global_counter,Entry::Val(value));
        self.front.insert(name.to_owned(),self.global_counter);
        self.global_counter = self.global_counter + 1;

        return Ok(self.global_counter-1);
    }
    pub fn new() -> SymbolTable{
        SymbolTable { global_counter: 1000,
            storage: HashMap::<u64,Entry>::new(),
            front: HashMap::<String,u64>::new()
        }
    }
    pub fn get_from_addr(&self, addr: u64, level: u16) -> Result<Value, String> {
        if level < 128 {
            if addr == 0 {
                return Result::Err("Null pointer dereference!".to_owned());
            }

            if self.storage.contains_key(&addr) {
                let o_entry: Option<&Entry> = self.storage.get(&addr);

                match o_entry {
                    Option::Some(ent) => match ent {
                        Entry::Val(vale) => return Result::Ok(vale.clone()),
                        Entry::Pointer(addr2) => self.get_from_addr(*addr2, level + 1),
                    },
                    _ => return Result::Err("Adress does not exist!".to_owned()),
                }
            } else {
                return Result::Err("Adress does not exist!".to_owned());
            }
        } else {
            return Result::Err("Nested dereferencing hit hard limit of 128!".to_owned());
        }
    }

    pub fn get_from_symbol(&self, symbol: &str) -> Result<Value, String> {
        if self.front.contains_key(symbol) {
            let r_addr: Option<&u64> = self.front.get(symbol);
            match r_addr {
                Option::Some(addr) => {
                    return self.get_from_addr(*addr, 0);
                }
                _ => {
                    return Result::Err("Symbol does not point to anything!".to_owned());
                }
            }
        } else {
            return Result::Err("Symbol does not exist!".to_owned());
        }
    }
}
