use std::collections::HashMap;

use crate::{
    badger_debug::{error, get_col, get_line_from_index},
    expression::Value,
};

pub struct SymbolTable<'a> {
    global_counter: u64,
    map: HashMap<String, Value>,
    encolsing: Box<Option<&'a SymbolTable<'a>>>,
}

impl<'a> SymbolTable<'a> {
    pub fn add_symbol(
        &mut self,
        name: &str,
        value: Value,
        index: &usize,
        lines: &Vec<usize>,
    ) -> Result<u64, String> {
        if self.map.contains_key(name) {
            let l = get_line_from_index(lines, index);
            let c = get_col(index, lines);
            return Result::Err(format!(
                "{} at line {}, {}",
                "Identifier already decleared", l, c
            ));
        }

        self.map.insert(name.to_owned(), value);
        self.global_counter = self.global_counter + 1;

        return Ok(self.global_counter - 1);
    }
    pub fn new(parent_scope: Option<&'a SymbolTable<'_>>) -> SymbolTable<'a> {
        SymbolTable {
            global_counter: 1000,
            map: HashMap::<String, Value>::new(),
            encolsing: Box::new(parent_scope),
        }
    }
    pub fn set_var_val(
        &mut self,
        name: &str,
        val: Value,
        index: &usize,
        lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let og_value = self.get_from_symbol(name, index, lines, 0)?;

        match og_value {
            Value::Boolean(_b) => match val {
                Value::Boolean(_) => {}
                _ => {
                    return error(
                        "Cannot assign different typed value to different types variable",
                        index,
                        lines,
                    )
                }
            },
            Value::Number(_b) => match val {
                Value::Number(_) => {}
                _ => {
                    return error(
                        "Cannot assign different typed value to different types variable",
                        index,
                        lines,
                    )
                }
            },
            Value::StringVal(_b) => match val {
                Value::StringVal(_) => {}
                _ => {
                    return error(
                        "Cannot assign different typed value to different types variable",
                        index,
                        lines,
                    )
                }
            },
        }

        *self.map.get_mut(name).unwrap() = val.clone();
        return Ok(val);
    }

    pub fn get_from_symbol(
        &self,
        var_name: &str,
        index: &usize,
        lines: &Vec<usize>,
        level: usize,
    ) -> Result<Value, String> {
        if self.map.contains_key(var_name) {
            let entry = self.map.get(var_name);

            match entry {
                Some(val) => return Ok(val.clone()),
                _ => {
                    if level < 256 {
                        match *self.encolsing {
                            Some(ref table) => {
                                return table.get_from_symbol(var_name, index, lines, level + 1);
                            }
                            _ => {}
                        }
                    }

                    return error(
                        &format!("Symbol '{}' does not exist!", var_name),
                        index,
                        lines,
                    );
                }
            }
        } else {
            return error(
                &format!("Symbol '{}' does not exist!", var_name),
                index,
                lines,
            );
        }
    }
}
