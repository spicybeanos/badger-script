use crate::expression::Expression;
use crate::tokenizer::TokenType;

pub struct Encoder {
    pub counter: usize,
    pub code: Vec<String>,
}

impl Encoder {
    pub fn new() -> Encoder{
        Encoder{
            counter:0,
            code:Vec::<String>::new()
        }
    }
    pub fn encode(&mut self, expr: Expression, lines: &Vec<usize>) -> Result<String, String> {
        match expr {
            Expression::Literal(val, _index) => {
                return Ok(format!("{:?}", val));
            }
            Expression::Variable(name, _index) => {
                self.code.push(format!("t{} = {}", self.counter, name));
                self.counter = self.counter + 1;

                return Ok(format!("t{}", self.counter - 1));
            }
            Expression::Unary(sign, val) => {
                let texpr = self.encode(*val, lines)?;
                match sign.ttype {
                    TokenType::Bang => {
                        self.code
                            .push(format!("t{} = invert {}", self.counter, texpr));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Minus => {
                        self.code
                            .push(format!("t{} = uneg {}", self.counter, texpr));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    _ => {
                        return error("Illegal unary operation", &sign.index, lines);
                    }
                }
            }
            Expression::Binary(left, sign, right) => {
                let left_expr = self.encode(*left, lines)?;
                let right_expr = self.encode(*right, lines)?;

                match sign.ttype {
                    TokenType::And => {
                        self.code.push(format!(
                            "t{} = {} and {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Or => {
                        self.code.push(format!(
                            "t{} = {} or {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Plus => {
                        self.code.push(format!(
                            "t{} = {} + {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Minus => {
                        self.code.push(format!(
                            "t{} = {} - {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Star => {
                        self.code.push(format!(
                            "t{} = {} * {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Slash => {
                        self.code.push(format!(
                            "t{} = {} / {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Mod => {
                        self.code.push(format!(
                            "t{} = {} % {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Equality => {
                        self.code.push(format!(
                            "t{} = {} eq {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::BangEquals => {
                        self.code.push(format!(
                            "t{} = {} neq {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Greater => {
                        self.code.push(format!(
                            "t{} = {} gt {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::GreaterEquals => {
                        self.code.push(format!(
                            "t{} = {} gte {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::Lesser => {
                        self.code.push(format!(
                            "t{} = {} lt {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    TokenType::LesserEquals => {
                        self.code.push(format!(
                            "t{} = {} lte {}",
                            self.counter, left_expr, right_expr
                        ));
                        self.counter = self.counter + 1;
                        return Ok(format!("t{}", self.counter - 1));
                    }
                    _ => return error("Illegal binary operation", &sign.index, lines),
                }
            }
            Expression::Assignment(name,val ,_index ) => {
                let tval = self.encode(*val, lines)?;
                self.code.push(format!("{} = {}",name,tval));
                return Ok(tval);
            }
            Expression::Group(val) => {
                let tval = self.encode(*val, lines)?;
                return Ok(tval);
            }
            _ => error("Not implemented", &0, lines)
        }
    }
}

fn get_line_from_index(lines: &Vec<usize>, index: &usize) -> usize {
    // Find the first newline index greater than the given index
    match lines.binary_search(index) {
        // If the index matches exactly a newline index, return the corresponding line number
        Ok(pos) => pos + 1,
        // If the index is within a range, return the line number of the closest previous newline
        Err(pos) => pos + 1,
    }
}

fn error(msg: &str, index: &usize, lines: &Vec<usize>) -> Result<String, String> {
    let l = get_line_from_index(lines, index);
    // let c = get_col(index, lines);
    return Result::Err(format!("{} at line {}", msg, l));
}
