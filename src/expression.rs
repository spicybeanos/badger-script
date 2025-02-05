use crate::{
    symbol_table::SymbolTable,
    tokenizer::{Token, TokenType},
};
use std::fmt;


#[derive(Clone)]
pub enum Expression {
    Symbol(String),
    Literal(Value),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Group(Box<Expression>),
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(v) => write!(f, "{:?} ", v),
            Expression::Unary(sgn, exp) => write!(f, "{:?}{:?} ", sgn, exp),
            Expression::Symbol(s) => write!(f, "{:?} ", s),
            Expression::Group(ex) => write!(f, "({:?}) ", ex),
            Expression::Binary(l, s, r) => write!(f, "{:?} {:?} {:?} ", l, s, r),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    StringVal(String),
    Boolean(bool),
}

impl Expression {
    pub fn evaluate(&self, table: &SymbolTable) -> Result<Value, String> {
        match self {
            Expression::Symbol(sym) => table.get_from_symbol(&sym),
            Expression::Literal(val_) => Result::Ok(val_.clone()),
            Expression::Unary(sign, expr) => {
                let r_: Result<Value, String> = expr.evaluate(table);
                match r_ {
                    Ok(_v_) => unary_signing(&_v_, &sign),
                    _ => r_,
                }
            }
            Expression::Binary(left, sign, right) => {
                let l_r: Result<Value, String> = left.evaluate(table);
                let r_r: Result<Value, String> = right.evaluate(table);

                match l_r {
                    Ok(_l_) => match r_r {
                        Ok(_r_) => binary_operation(&_l_, &sign, &_r_),
                        _ => r_r,
                    },
                    _ => l_r,
                }
            }
            Expression::Group(g) => g.evaluate(table),
        }
    }
}

fn binary_operation(left: &Value, operator: &Token, right: &Value) -> Result<Value, String> {
    match operator.ttype {
        TokenType::Plus => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(rn + ln));
                }
                Value::StringVal(rs) => {
                    return Result::Ok(Value::StringVal(ln.to_string() + rs));
                }
                _ => {
                    return Result::Err(
                        "operation is not defined! at ".to_string() + &operator.index.to_string(),
                    )
                }
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
            _ => {
                return Result::Err(
                    "operation is not defined! at ".to_string() + &operator.index.to_string(),
                )
            }
        },
        TokenType::Minus => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln - rn));
                }
                _ => {
                    return Result::Err(
                        "operation is not defined! at ".to_string() + &operator.index.to_string(),
                    )
                }
            },
            _ => {
                return Result::Err(
                    "operation is not defined! at ".to_string() + &operator.index.to_string(),
                )
            }
        },
        TokenType::Star => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln * rn));
                }
                _ => return opp_undef(operator),
            },
            _ => return opp_undef(operator),
        },
        TokenType::Slash => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln / rn));
                }
                _ => return opp_undef(operator),
            },
            _ => return opp_undef(operator),
        },
        TokenType::Mod => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln % rn));
                }
                _ => return opp_undef(operator),
            },
            _ => return opp_undef(operator),
        },

        TokenType::And => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb && *rb)),
                _ => opp_undef(operator),
            },
            _ => opp_undef(operator),
        },
        TokenType::Or => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb || *rb)),
                _ => opp_undef(operator),
            },
            _ => opp_undef(operator),
        },

        TokenType::Equality => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb == *rb)),
                _ => opp_undef(operator),
            },
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln == *rn)),
                _ => opp_undef(operator),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls == *rs)),
                _ => opp_undef(operator),
            },
        },
        TokenType::BangEquals => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb != *rb)),
                _ => opp_undef(operator),
            },
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln != *rn)),
                _ => opp_undef(operator),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls != *rs)),
                _ => opp_undef(operator),
            },
        },

        TokenType::Greater => match left {
            Value::Boolean(_lb) => opp_undef(operator),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln > *rn)),
                _ => opp_undef(operator),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls > *rs)),
                _ => opp_undef(operator),
            },
        },
        TokenType::GreaterEquals => match left {
            Value::Boolean(_lb) => opp_undef(operator),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln >= *rn)),
                _ => opp_undef(operator),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls >= *rs)),
                _ => opp_undef(operator),
            },
        },
        TokenType::Lesser => match left {
            Value::Boolean(_lb) => opp_undef(operator),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln < *rn)),
                _ => opp_undef(operator),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls < *rs)),
                _ => opp_undef(operator),
            },
        },
        TokenType::LesserEquals => match left {
            Value::Boolean(_lb) => opp_undef(operator),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln <= *rn)),
                _ => opp_undef(operator),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls <= *rs)),
                _ => opp_undef(operator),
            },
        },

        _ => opp_undef(operator),
    }
}
fn opp_undef(operator: &Token) -> Result<Value, String> {
    return Result::Err("operation is not defined! at ".to_string() + &operator.index.to_string());
}
fn boolify(val: &Value) -> bool {
    match val {
        Value::Boolean(b) => *b,
        Value::Number(n) => *n > 0.0,
        Value::StringVal(s) => !s.is_empty()
    }
}
fn unary_signing(val: &Value, sign: &Token) -> Result<Value, String> {
    match sign.ttype {
        TokenType::Bang => Ok(Value::Boolean(!boolify(val))),
        TokenType::Minus => match val {
            Value::Number(n) => Result::Ok(Value::Number(-n)),
            _ => Result::Err("Cannot use '-' on anything other than a 'num'".to_owned()),
        },
        _ => Result::Err("Cannot use this operator in a unary expression!".to_owned()),
    }
}
