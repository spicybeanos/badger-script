use crate::{expression::{Expression, Value}, symbol_table::SymbolTable};

#[derive(Clone)]
pub enum Statement{
    Expr(Expression),
    Return(Expression)
}

impl Statement {
    pub fn accept(&self,table:&mut SymbolTable,debug_lines:&Vec<usize>) -> Result<Value,String>{
        match self {
            Self::Return(rexpr) => Self::visit_return(&rexpr,table,debug_lines),
            Self::Expr(expr) => Self::visit_expr(&expr, table,debug_lines)
        }
    }
    fn visit_expr(expr:&Expression,table:&mut SymbolTable,debug_lines:&Vec<usize>) -> Result<Value,String> {
        return expr.evaluate(table,debug_lines);
    }
    fn visit_return(expr:&Expression,table:&mut SymbolTable,debug_lines:&Vec<usize>) -> Result<Value,String>{
        let val:Value = expr.evaluate(table,debug_lines)?;
        match &val {
            Value::Boolean(b)=>println!("{}",b),
            Value::StringVal(s)=>println!("{}",s),
            Value::Number(n)=>println!("{}",n),
        }
        return Ok(val);
    }
}