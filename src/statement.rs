use crate::{
    expression::{Expression, Value},
    symbol_table::SymbolTable,
};

#[derive(Clone)]
pub enum Statement {
    Expr(Expression),
    Return(Expression),
    VarDecl(String, Expression),
}

impl Statement {
    pub fn accept(
        &self,
        table: &mut SymbolTable,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        match self {
            Self::Return(rexpr) => Self::visit_return(&rexpr, table, debug_lines),
            Self::Expr(expr) => Self::visit_expr(&expr, table, debug_lines),
            Self::VarDecl(name, init) => Self::visit_var_decl(name, init, table, debug_lines),
        }
    }
    fn visit_var_decl(
        name: &String,
        init: &Expression,
        table: &mut SymbolTable,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let value = init.evaluate(table, debug_lines)?;
        table.add_symbol(name, value, &0, debug_lines)?;

        return Ok(Value::Boolean(true));
    }
    fn visit_expr(
        expr: &Expression,
        table: &mut SymbolTable,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        return expr.evaluate(table, debug_lines);
    }
    fn visit_return(
        expr: &Expression,
        table: &mut SymbolTable,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let val: Value = expr.evaluate(table, debug_lines)?;
        match &val {
            Value::Boolean(b) => println!("{}", b),
            Value::StringVal(s) => println!("{}", s),
            Value::Number(n) => println!("{}", n),
        }
        return Ok(val);
    }
}
