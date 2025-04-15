use crate::{
    badger_debug::{error, get_col, get_line_from_index},
    symbol_table::SymbolTable,
    tokenizer::{Token, TokenType},
};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub enum Expression {
    SpecialSymbol(String, usize),
    Literal(Value, usize),
    Variable(String, usize),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Group(Box<Expression>),
    Assignment(String, Box<Expression>, usize),
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Literal(v, _lindx) => write!(f, "{:?} ", v),
            Expression::Unary(sgn, exp) => write!(f, "{:?}{:?} ", sgn, exp),
            Expression::SpecialSymbol(s, _) => write!(f, "{:?} ", s),
            Expression::Group(ex) => write!(f, "({:?}) ", ex),
            Expression::Binary(l, s, r) => write!(f, "({:?} {:?} {:?}) ", l, s, r),
            Expression::Variable(name, _iindex) => write!(f, "{:?}", name),
            Expression::Assignment(lhs, rhs, _) => write!(f, "{:?} = {:?}", lhs,rhs),
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Number(f64),
    StringVal(String),
    Boolean(bool),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Number(n) => write!(f, "{}", n),
            Self::StringVal(s) => write!(f, "'{}'", s),
        }
    }
}

impl Expression {
    pub fn evaluate(
        &self,
        table: Rc<RefCell<SymbolTable>>,
        debug_lines: &Vec<usize>,
    ) -> Result<Value, String> {
        match self {
            Expression::SpecialSymbol(sym, _sindx) => {
                table
                    .borrow_mut()
                    .get_from_symbol(&sym, _sindx, debug_lines, 0)
            }
            Expression::Literal(val_, _lindx) => Result::Ok(val_.clone()),
            Expression::Unary(sign, expr) => {
                let r_: Result<Value, String> = expr.evaluate(table, debug_lines);
                match r_ {
                    Ok(_v_) => unary_signing(&_v_, &sign, debug_lines),
                    _ => r_,
                }
            }
            Expression::Binary(left, sign, right) => {
                let l_r: Result<Value, String> = left.evaluate(Rc::clone(&table), debug_lines);
                let r_r: Result<Value, String> = right.evaluate(table, debug_lines);

                match l_r {
                    Ok(_l_) => match r_r {
                        Ok(_r_) => binary_operation(&_l_, &sign, &_r_, debug_lines),
                        _ => r_r,
                    },
                    _ => l_r,
                }
            }
            Expression::Group(g) => g.evaluate(table, debug_lines),
            Expression::Variable(name, index) => {
                table
                    .borrow_mut()
                    .get_from_symbol(name, index, debug_lines, 0)
            }
            Expression::Assignment(name, rhs, s_idx) => {
                let val = rhs.evaluate(Rc::clone(&table), debug_lines)?;
                table
                    .borrow_mut()
                    .set_var_val(name, val, s_idx, debug_lines, 0)
            }
        }
    }
}

fn binary_operation(
    left: &Value,
    operator: &Token,
    right: &Value,
    lines: &Vec<usize>,
) -> Result<Value, String> {
    match operator.ttype {
        TokenType::Plus => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(rn + ln));
                }
                Value::StringVal(rs) => {
                    return Result::Ok(Value::StringVal(ln.to_string() + rs));
                }
                _ => return opp_undef(operator, lines),
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
            _ => return opp_undef(operator, lines),
        },
        TokenType::Minus => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln - rn));
                }
                _ => return opp_undef(operator, lines),
            },
            _ => return opp_undef(operator, lines),
        },
        TokenType::Star => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln * rn));
                }
                _ => return opp_undef(operator, lines),
            },
            Value::StringVal(sl) => match right {
                Value::Number(rn) => {
                    let mut rep = *rn;
                    let mut mstr = "".to_owned();
                    while rep > 0.0 {
                        mstr.push_str(sl);
                        rep = rep - 1.0;
                    }
                    return Result::Ok(Value::StringVal(mstr));
                }
                _ => return opp_undef(operator, lines),
            },
            _ => return opp_undef(operator, lines),
        },
        TokenType::Slash => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln / rn));
                }
                _ => return opp_undef(operator, lines),
            },
            _ => return opp_undef(operator, lines),
        },
        TokenType::Mod => match left {
            Value::Number(ln) => match right {
                Value::Number(rn) => {
                    return Result::Ok(Value::Number(ln % rn));
                }
                _ => return opp_undef(operator, lines),
            },
            _ => return opp_undef(operator, lines),
        },

        TokenType::And => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb && *rb)),
                _ => opp_undef(operator, lines),
            },
            _ => opp_undef(operator, lines),
        },
        TokenType::Or => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb || *rb)),
                _ => opp_undef(operator, lines),
            },
            _ => opp_undef(operator, lines),
        },

        TokenType::Equality => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb == *rb)),
                _ => opp_undef(operator, lines),
            },
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln == *rn)),
                _ => opp_undef(operator, lines),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls == *rs)),
                _ => opp_undef(operator, lines),
            },
        },
        TokenType::BangEquals => match left {
            Value::Boolean(lb) => match right {
                Value::Boolean(rb) => Result::Ok(Value::Boolean(*lb != *rb)),
                _ => opp_undef(operator, lines),
            },
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln != *rn)),
                _ => opp_undef(operator, lines),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls != *rs)),
                _ => opp_undef(operator, lines),
            },
        },

        TokenType::Greater => match left {
            Value::Boolean(_lb) => opp_undef(operator, lines),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln > *rn)),
                _ => opp_undef(operator, lines),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls > *rs)),
                _ => opp_undef(operator, lines),
            },
        },
        TokenType::GreaterEquals => match left {
            Value::Boolean(_lb) => opp_undef(operator, lines),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln >= *rn)),
                _ => opp_undef(operator, lines),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls >= *rs)),
                _ => opp_undef(operator, lines),
            },
        },
        TokenType::Lesser => match left {
            Value::Boolean(_lb) => opp_undef(operator, lines),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln < *rn)),
                _ => opp_undef(operator, lines),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls < *rs)),
                _ => opp_undef(operator, lines),
            },
        },
        TokenType::LesserEquals => match left {
            Value::Boolean(_lb) => opp_undef(operator, lines),
            Value::Number(ln) => match right {
                Value::Number(rn) => Result::Ok(Value::Boolean(*ln <= *rn)),
                _ => opp_undef(operator, lines),
            },
            Value::StringVal(ls) => match right {
                Value::StringVal(rs) => Result::Ok(Value::Boolean(*ls <= *rs)),
                _ => opp_undef(operator, lines),
            },
        },

        _ => opp_undef(operator, lines),
    }
}
fn opp_undef(operator: &Token, lines: &Vec<usize>) -> Result<Value, String> {
    let l = get_line_from_index(lines, &operator.index);
    let c = get_col(&operator.index, lines);
    return Result::Err(format!("operation is not defined! at line {}, {}", l, c));
}
pub fn boolify(val: &Value) -> bool {
    match val {
        Value::Boolean(b) => *b,
        Value::Number(n) => *n > 0.0,
        Value::StringVal(s) => !s.is_empty(),
    }
}
fn unary_signing(val: &Value, sign: &Token, lines: &Vec<usize>) -> Result<Value, String> {
    match sign.ttype {
        TokenType::Bang => Ok(Value::Boolean(!boolify(val))),
        TokenType::Minus => match val {
            Value::Number(n) => Result::Ok(Value::Number(-n)),
            _ => error(
                "Cannot use '-' on anything other than a 'num'",
                &sign.index,
                lines,
            ),
        },
        _ => error(
            "Cannot use this operator in a unary expression!",
            &sign.index,
            lines,
        ),
    }
}
