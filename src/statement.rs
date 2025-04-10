use crate::{
    badger_debug::error,
    expression::{boolify, Expression, Value},
    symbol_table::SymbolTable,
    tokenizer::TokenType,
};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub enum Statement {
    Expr(Expression),
    Return(Expression),
    Block(Vec<Statement>),
    IfStmt(Expression, Box<Statement>, Box<Option<Statement>>),
    WhileStmt(Expression, Box<Statement>),
    VarDecl(String, Expression, TokenType, usize),
}

impl Statement {
    pub fn accept(
        &self,
        table: Rc<RefCell<SymbolTable>>,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        match self {
            Self::Return(rexpr) => {
                Self::visit_return(rexpr, Rc::clone(&table), debug_lines)
            }
            Self::Expr(expr) => {
                Self::visit_expr(expr, Rc::clone(&table), debug_lines)
            }
            Self::VarDecl(name, init, vtype, index) => {
                Self::visit_var_decl(name, vtype, init, Rc::clone(&table), index, debug_lines)
            }
            Self::Block(statements) => {
                let new_table = SymbolTable::new(Some(Rc::clone(&table)));
                Self::execute_block(statements, new_table, debug_lines)
            }
            Self::IfStmt(condition, then, else_branch) => {
                Self::execute_if(condition, then, else_branch, Rc::clone(&table), debug_lines)
            }
            Self::WhileStmt(condition, body) => {
                Self::execute_while(condition, body, Rc::clone(&table), debug_lines)
            }
        }
    }
    fn execute_if(
        condition: &Expression,
        then: &Statement,
        else_branch: &Option<Statement>,
        table: Rc<RefCell<SymbolTable>>,
        lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let value = condition.evaluate(Rc::clone(&table), lines)?;
        let truthy = boolify(&value);
        if truthy {
            then.accept(table, lines)?;
            return Ok(Value::Boolean(true));
        } else {
            match else_branch {
                Some(stmt) => {
                    stmt.accept(table, lines)?;
                }
                _ => {}
            }
        }

        return Ok(Value::Boolean(false));
    }

    fn execute_while(
        condition: &Expression,
        then: &Statement,
        table: Rc<RefCell<SymbolTable>>,
        lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let mut value = condition.evaluate(Rc::clone(&table), lines)?;
        let mut truthy = boolify(&value);

        while truthy {
            then.accept(Rc::clone(&table), lines)?;
            value = condition.evaluate(Rc::clone(&table), lines)?;
            truthy = boolify(&value);
        }

        Ok(Value::Boolean(false))
    }

    fn execute_block(
        statements: &Vec<Statement>,
        table: SymbolTable,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let local_table = Rc::<RefCell<SymbolTable>>::new(RefCell::new(table));
        for stmt in statements {
            stmt.accept(Rc::clone(&local_table), debug_lines)?;
        }

        return Ok(Value::Boolean(true));
    }
    fn visit_var_decl(
        name: &String,
        vtype: &TokenType,
        init: &Expression,
        table: Rc<RefCell<SymbolTable>>,
        index: &usize,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        let value = init.evaluate(Rc::clone(&table), debug_lines)?;

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

        table.borrow_mut().add_symbol(name, value, index, debug_lines)?;

        return Ok(Value::Boolean(true));
    }
    fn visit_expr(
        expr: &Expression,
        table: Rc<RefCell<SymbolTable>>,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        return expr.evaluate(table, debug_lines);
    }
    fn visit_return(
        expr: &Expression,
        table: Rc<RefCell<SymbolTable>>,
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
