use crate::{
    expression::{Expression, Value},
    tokenizer::{Token, TokenType},
};

pub struct ExpressionParser<'a> {
    current: usize,
    lines: &'a Vec<usize>,
    tokens: &'a Vec<Token>,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(tokens_: &'a Vec<Token>, lines_: &'a Vec<usize>) -> ExpressionParser<'a> {
        ExpressionParser {
            tokens: tokens_,
            current: 0,
            lines: lines_,
        }
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_type(&[TokenType::Bang,TokenType::Minus]) {
            
        }
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_type(&[TokenType::False]) {
            return Ok(Expression::Literal(Value::Boolean(false)));
        }
        if self.match_type(&[TokenType::True]) {
            return Ok(Expression::Literal(Value::Boolean(true)));
        }

        if let Some(num) = self.match_number_literal() {
            return Ok(Expression::Literal(Value::Number(num)));
        }
        if let Some(b) = self.match_boolean_literal() {
            return Ok(Expression::Literal(Value::Boolean(b)));
        }
        if let Some(string_value) = self.match_string_literal() {
            return Ok(Expression::Literal(Value::StringVal(string_value)));
        }

        if let Some(string_value) = self.match_symbol() {
            return Ok(Expression::Symbol(string_value));
        }

        if self.match_type(&[TokenType::OpenParent]) {
            let expr: Expression = expression()?;
            self.consume(&TokenType::CloseParent, "Expected ')' after expression");
            return Ok(Expression::Group(expr));
        }

        return self.error_ex(self.peek(), "Expected expression");
    }

    //helper functions

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        return self.previous();
    }
    fn error(&self, token: &Token, msg: &str) -> Result<&Token, String> {
        Result::Err(format!(
            "{} at line {}",
            msg,
            self.get_line_from_index(token.index)
        ))
    }
    fn error_ex(&self, token: &Token, msg: &str) -> Result<Expression, String> {
        Result::Err(format!(
            "{} at line {}",
            msg,
            self.get_line_from_index(token.index)
        ))
    }
    fn consume(&mut self, ttype: &TokenType, err_msg: &str) -> Result<&Token, String> {
        if self.check(ttype) {
            return Ok(self.advance());
        }
        return self.error(self.peek(), err_msg);
    }
    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().ttype == *ttype;
    }
    fn match_type(&mut self, ttypes: &[TokenType]) -> bool {
        for tt in ttypes {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn match_number_literal(&mut self) -> Option<f64> {
        if let TokenType::NumberLiteral(num) = self.peek().ttype {
            self.advance(); // Move past the number
            Some(num) // Return the extracted number
        } else {
            None
        }
    }
    fn match_boolean_literal(&mut self) -> Option<bool> {
        if let TokenType::BooleanLiteral(b) = self.peek().ttype {
            self.advance(); // Move past the number
            Some(b) // Return the extracted number
        } else {
            None
        }
    }
    fn match_string_literal(&mut self) -> Option<String> {
        let token_type = self.peek().ttype.clone(); // Clone the token type to avoid borrowing issues

        if let TokenType::StringLiteral(s) = token_type {
            self.advance(); // Now it's safe to advance
            Some(s) // Return the extracted string
        } else {
            None
        }
    }
    fn match_symbol(&mut self) -> Option<String> {
        let token_type = self.peek().ttype.clone(); // Clone the token type to avoid borrowing issues

        if let TokenType::Identifier(s) = token_type {
            self.advance(); // Now it's safe to advance
            Some(s) // Return the extracted string
        } else {
            None
        }
    }
    pub fn get_line_from_index(&self, index: usize) -> usize {
        // Find the first newline index greater than the given index
        match self.lines.binary_search(&index) {
            // If the index matches exactly a newline index, return the corresponding line number
            Ok(pos) => pos + 1,
            // If the index is within a range, return the line number of the closest previous newline
            Err(pos) => pos + 1,
        }
    }
}
