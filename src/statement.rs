use crate::{
    badger_debug::error,
    expression::{Expression, Value},
    symbol_table::SymbolTable,
    tokenizer::TokenType,
};

#[derive(Clone)]
pub enum Statement {
    Expr(Expression),
    Return(Expression),
    Block(Vec<Statement>),
    VarDecl(String, Expression, TokenType, usize),
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
            Self::VarDecl(name, init, vtype, index) => {
                Self::visit_var_decl(name, vtype, init, table, index, debug_lines)
            },
            Self::Block(statments) => Self::execute_block(statments,SymbolTable::new(Some(&table)),debug_lines)
        }
    }
    fn execute_block(
        statements:&Vec<Statement>,
        table:SymbolTable,
        debug_lines: &Vec<usize>
    ) -> Result<Value, String> {
        
        let mut local_table = table;
        for stmt in statements {
            stmt.accept(&mut local_table,debug_lines)?;
        }

        return Ok(Value::Boolean(true));
    }
    fn visit_var_decl(
        name: &String,
        vtype: &TokenType,
        init: &Expression,
        table: &mut SymbolTable,
        index: &usize,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let value = init.evaluate(table, debug_lines)?;

        match value {
            Value::Number(_) => match vtype {
                TokenType::Num => {}
                TokenType::Var => {}
                _ => {
                    return error(
                        "Expression is not of expected type (num)",
                        index,
                        debug_lines,
                    );
                }
            },
            Value::Boolean(_) => match vtype {
                TokenType::Bool => {}
                TokenType::Var => {}
                _ => {
                    return error(
                        "Expression is not of expected type (bool)",
                        index,
                        debug_lines,
                    );
                }
            },
            Value::StringVal(_) => match vtype {
                TokenType::Str => {}
                TokenType::Var => {}
                _ => {
                    return error(
                        "Expression is not of expected type (str)",
                        index,
                        debug_lines,
                    );
                }
            },
        }

        table.add_symbol(name, value, index, debug_lines)?;

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
