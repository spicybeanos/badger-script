use crate::{
    expression::{Expression, Value},
    tokenizer::{KeyWords, Token, TokenType},
};

//struct ExprParser<'a> {
//    index: usize,
//    tokens: &'a Vec<Token>,
//    lines: &'a Vec<usize>,
//}

fn match_token(ttypes: &[TokenType], tokens: &Vec<Token>, index: &mut usize) -> bool {
    for t in ttypes {
        if check(t, tokens, index) {
            if !is_end(tokens, index) {
                *index = *index + 1;
            }
            return true;
        }
    }
    false
}
fn match_keyword(kwords: &[KeyWords], token: &Token, index: &mut usize) -> bool {
    match token.ttype {
        TokenType::Keyword(k) => {
            for ktest in kwords {
                if *ktest == k {
                    *index = *index + 1;
                    return true;
                }
            }
        }
        _ => return false,
    }

    return false;
}
fn check(ttype: &TokenType, tokens: &Vec<Token>, index: &mut usize) -> bool {
    if is_end(tokens, index) {
        false
    } else {
        tokens[*index].ttype == *ttype
    }
}

fn consume(
    ttype: &TokenType,
    tokens: &Vec<Token>,
    index: &mut usize,
    error: &str,
) -> Result<usize, String> {
    if check(ttype, tokens, index) {
        if !is_end(tokens, index) {
            *index = *index + 1;
        }
        return Ok(*index - 1);
    }

    return Err(error.to_owned());
}

fn is_end(tokens: &Vec<Token>, index: &mut usize) -> bool {
    tokens[*index].ttype == TokenType::Eof
}

pub fn parse(tokens: &Vec<Token>) -> Result<Expression, String> {
    let mut index: usize = 0;
    expression(tokens, &mut index)
}

fn expression(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    equality(tokens, index)
}

fn equality(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut left: Expression = comparision(tokens, index)?;

    while match_token(&[TokenType::Equality, TokenType::BangEquals], tokens, index) {
        let operator: &Token = &tokens[*index - 1];
        let right: Expression = comparision(tokens, index)?;
        let temp = left;
        left = Expression::Binary(Box::new(temp), operator.to_owned(), Box::new(right));
    }

    return Ok(left);
}

fn comparision(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut left: Expression = term(tokens, index)?;

    while match_token(
        &[
            TokenType::Greater,
            TokenType::GreaterEquals,
            TokenType::Lesser,
            TokenType::LesserEquals,
        ],
        tokens,
        index,
    ) {
        let operator: &Token = &tokens[*index - 1];
        let right: Expression = term(tokens, index)?;
        let temp = left;
        left = Expression::Binary(Box::new(temp), operator.to_owned(), Box::new(right));
    }

    return Ok(left);
}
fn term(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut left: Expression = factor(tokens, index)?;

    while match_token(&[TokenType::Minus, TokenType::Plus], tokens, index) {
        let operator: &Token = &tokens[*index - 1];
        let right: Expression = factor(tokens, index)?;
        let temp = left;
        left = Expression::Binary(Box::new(temp), operator.to_owned(), Box::new(right));
    }

    return Ok(left);
}
fn factor(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    let mut left: Expression = unary(tokens, index)?;

    while match_token(
        &[TokenType::Star, TokenType::Slash, TokenType::Mod],
        tokens,
        index,
    ) {
        let operator: &Token = &tokens[*index - 1];
        let right: Expression = unary(tokens, index)?;
        let temp = left;
        left = Expression::Binary(Box::new(temp), operator.to_owned(), Box::new(right));
    }

    return Ok(left);
}

fn unary(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    if match_token(&[TokenType::Minus, TokenType::Bang], tokens, index) {
        let operator: &Token = &tokens[*index - 1];
        let right: Expression = unary(tokens, index)?;
        return Ok(Expression::Unary(operator.to_owned(), Box::new(right)));
    }

    return primary(tokens, index);
}
fn primary(tokens: &Vec<Token>, index: &mut usize) -> Result<Expression, String> {
    if match_keyword(&[KeyWords::False], &tokens[*index], index) {
        return Ok(Expression::Literal(Value::Boolean(false)));
    }
    if match_keyword(&[KeyWords::True], &tokens[*index], index) {
        return Ok(Expression::Literal(Value::Boolean(true)));
    }

    if let TokenType::NumberLiteral(num) = tokens[*index].ttype {
        return Ok(Expression::Literal(Value::Number(num)));
    }
    if let TokenType::StringLiteral(str_) = &tokens[*index].ttype {
        return Ok(Expression::Literal(Value::StringVal(str_.to_owned())));
    }
    if let TokenType::BooleanLiteral(boo_) = tokens[*index].ttype {
        return Ok(Expression::Literal(Value::Boolean(boo_)));
    }

    if match_token(&[TokenType::OpenParent], tokens, index) {
        let expr = expression(tokens, index)?;
        let _con = consume(
            &TokenType::CloseParent,
            tokens,
            index,
            "Expected ')' after expression",
        )?;
        return Ok(Expression::Group(Box::new(expr)));
    }

    return Err("Expected expression at ".to_owned() + &*index.to_string());
}
