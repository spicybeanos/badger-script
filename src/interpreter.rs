use crate::{expression::Value, statement::Statement, symbol_table::SymbolTable};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter<'a> {
    symbol_table: Rc<RefCell<SymbolTable>>,
    statments: &'a Vec<Option<Statement>>,
    debug_lines: &'a Vec<usize>,
}

impl<'a> Interpreter<'a> {
    pub fn new(
        table: Rc<RefCell<SymbolTable>>,
        stmt: &'a Vec<Option<Statement>>,
        debug_lines: &'a Vec<usize>,
    ) -> Interpreter<'a> {
        Interpreter {
            symbol_table: table,
            statments: stmt,
            debug_lines: &debug_lines,
        }
    }
    pub fn interpret(&mut self) -> Result<i32, String> {
        for stmt in self.statments.clone() {
            match stmt {
                Some(s) => {
                    let r = self.execute(&s);
                    match r {
                        Ok(_) => {}
                        Err(er) => return Result::Err(er),
                    }
                },
                None => {
                    
                }
            }
        }

        return Ok(0);
    }
    pub fn execute(&mut self, stmt: &Statement) -> Result<Value, String> {
        stmt.accept(Rc::clone(&self.symbol_table), &self.debug_lines)
    }
}
