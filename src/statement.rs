use crate::{expression::{Expression, Value}, symbol_table::SymbolTable};

#[derive(Clone)]
pub enum Statement{
    Expr(Expression),
    Return(Expression)
}

impl Statement {
    pub fn accept(&self,table:&mut SymbolTable) -> Result<Value,String>{
        match self {
            Self::Return(rexpr) => Self::visit_return(&rexpr,table),
            Self::Expr(expr) => Self::visit_expr(&expr, table)
        }
    }
    fn visit_expr(expr:&Expression,table:&mut SymbolTable) -> Result<Value,String> {
        return expr.evaluate(table);
    }
    fn visit_return(expr:&Expression,table:&mut SymbolTable) -> Result<Value,String>{
        let val:Value = expr.evaluate(table)?;
        match &val {
            Value::Boolean(b)=>println!("{}",b),
            Value::StringVal(s)=>println!("{}",s),
            Value::Number(n)=>println!("{}",n),
        }
        return Ok(val);
    }
}