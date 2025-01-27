use crate::{
    symbol_table::SymbolTable,
    tokenizer::{Token, TokenType},
};

pub enum Expression {
    Symbol(String),
    Literal(Value),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Group(Box<Expression>),
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    StringVal(String),
    Boolean(bool),
}

pub fn accept(expr: &Expression, table: &SymbolTable) -> Result<Value, &'static str> {
    match expr {
        Expression::Literal(val_) => Result::Ok(val_.clone()),
        Expression::Unary(sign, expr) => {
            let r_: Result<Value, &'static str> = accept(&expr, table);
            match r_ {
                Ok(_v_) => unary_signing(&_v_, sign),
                _ => r_,
            }
        }
        Expression::Binary(left, sign, right) => {
            let l_r: Result<Value, &'static str> = accept(left, table);
            let r_r: Result<Value, &'static str> = accept(right, table);

            match l_r {
                Ok(_l_) => match r_r {
                    Ok(_r_) => binary_operation(&_l_, &sign, &_r_),
                    _ => r_r,
                },
                _ => l_r,
            }
        }
        Expression::Group(g) => accept(g),
    }
}

fn binary_operation(left: &Value, operator: &Token, right: &Value) -> Result<Value, &'static str> {
    match operator.ttype {
        TokenType::Plus => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(rn + ln));
                }
                Value::StringVal(rs) => {
                    return Result::Ok(Value::StringVal(ln.to_string() + rs));
                }
                _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string())),
            },
            Value::StringVal(ls) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::StringVal(ls.to_owned() + &rn.to_string()));
                }
                Value::StringVal(rs) => {
                    return Result::Ok(Value::StringVal(ls.to_string() + rs));
                }
                Value::Boolean(rb) => {
                    return Result::Ok(Value::StringVal(ls.to_owned() + &rb.to_string()));
                }
            },
            _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string()))
        },
        TokenType::Minus => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln - rn));
                }
                _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string())),
            },
            _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string()))
        },
        TokenType::Star => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln * rn));
                }
                _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string())),
            },
            _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string()))
        },
        TokenType::Slash => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln / rn));
                }
                _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string())),
            },
            _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string()))
        },
        TokenType::Mod => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln % rn));
                }
                _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string())),
            },
            _ => return Result::Err(&("operation is not defined! at ".to_string() + &operator.index.to_string()))
        },

        
    }
}

fn unary_signing(val: &Value, sign: &Token) -> Result<Value, &'static str> {
    match sign.ttype {
        TokenType::Bang => match val {
            Value::Boolean(b) => Result::Ok(Value::Boolean(!b)),
            Value::Number(n) => Result::Ok(Value::Boolean(*n != 0.0)),
            Value::StringVal(s) => Result::Ok(Value::Boolean(!s.is_empty())),
        },
        TokenType::Minus => match val {
            Value::Number(n) => Result::Ok(Value::Number(-n)),
            _ => Result::Err("Cannot use '-' on anything other than a 'num'"),
        },
        _ => Result::Err("Cannot use this operator in a unary expression!"),
    }
}
