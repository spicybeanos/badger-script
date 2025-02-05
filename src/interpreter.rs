use crate::{expression::Value, statement::Statement, symbol_table::SymbolTable};

pub struct Interpreter<'a>{
    symbol_table:&'a mut SymbolTable,
    statments:&'a Vec<Statement>
}

impl<'a> Interpreter<'a> {
    pub fn new(table:&'a mut SymbolTable,stmt:&'a Vec<Statement>) -> Interpreter<'a>
    {
        Interpreter{
            symbol_table:table,
            statments:stmt
        }
    }
    pub fn interpret(&mut self) -> Result<i32,String> {
        
        for stmt in self.statments.clone() {
            let r = self.execute(&stmt);
            match r {
                Ok(_) => {},
                Err(er) => return Result::Err(er)
            }
        }

        return Ok(0);
    }
    pub fn execute(&mut self,stmt:&Statement) -> Result<Value, String> {
        stmt.accept(&mut self.symbol_table)
    }
}