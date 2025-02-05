use crate::{
    expression::{Expression, Value},
    statement::Statement,
    tokenizer::{Token, TokenType},
};

pub struct ExprStmtParser<'a> {
    current: usize,
    lines: &'a Vec<usize>,
    tokens: &'a Vec<Token>,
}

impl<'a> ExprStmtParser<'a> {
    pub fn new(
        tokens_: &'a Vec<Token>,
        lines_: &'a Vec<usize>,
        start: usize,
    ) -> ExprStmtParser<'a> {
        ExprStmtParser {
            tokens: tokens_,
            current: start,
            lines: lines_,
        }
    }

    pub fn parse_statement(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmt: Vec<Statement> = Vec::<Statement>::new();
        while !self.is_at_end() {
            stmt.push(self.statement()?);
        }

        return Ok(stmt);
    }

    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_tokentype(&[TokenType::Return]) {
            return self.return_statement();
        }

        return self.expr_statement();
    }
    fn return_statement(&mut self) -> Result<Statement,String>{
        let value = self.expression()?;
        self.consume(&TokenType::EoStmt, "Expected ';' after value.")?;
        return Ok(Statement::Return(value));
    }
    fn expr_statement(&mut self) -> Result<Statement,String>{
        let value = self.expression()?;
        self.consume(&TokenType::EoStmt, "Expected ';' after value.")?;
        return Ok(Statement::Expr(value));
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.expression()
    }
    pub fn expression(&mut self) -> Result<Expression, String> {
        self.boolean_logic()
    }
    fn boolean_logic(&mut self) -> Result<Expression, String> {
        let mut expr: Expression = self.equality()?;
        while self.match_tokentype(&[TokenType::And, TokenType::Or]) {
            let op: Token = self.previous().clone();
            let right: Expression = self.equality()?;
            let temp = expr;
            expr = Expression::Binary(Box::new(temp), op, Box::new(right));
        }

        return Ok(expr);
    }
    fn equality(&mut self) -> Result<Expression, String> {
        let mut expr: Expression = self.comparison()?;
        while self.match_tokentype(&[TokenType::Equality, TokenType::BangEquals]) {
            let op: Token = self.previous().clone();
            let right: Expression = self.comparison()?;
            let temp = expr;
            expr = Expression::Binary(Box::new(temp), op, Box::new(right));
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expression, String> {
        let mut expr: Expression = self.term()?;
        while self.match_tokentype(&[
            TokenType::Greater,
            TokenType::GreaterEquals,
            TokenType::Lesser,
            TokenType::LesserEquals,
        ]) {
            let op: Token = self.previous().clone();
            let right: Expression = self.term()?;
            let temp = expr;
            expr = Expression::Binary(Box::new(temp), op, Box::new(right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expression, String> {
        let mut expr: Expression = self.factor()?;
        while self.match_tokentype(&[TokenType::Plus, TokenType::Minus]) {
            let op: Token = self.previous().clone();
            let right: Expression = self.factor()?;
            let temp = expr;
            expr = Expression::Binary(Box::new(temp), op, Box::new(right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr: Expression = self.unary()?;
        while self.match_tokentype(&[TokenType::Slash, TokenType::Star, TokenType::Mod]) {
            let op: Token = self.previous().clone();
            let right: Expression = self.unary()?;
            let temp = expr;
            expr = Expression::Binary(Box::new(temp), op, Box::new(right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokentype(&[TokenType::Bang, TokenType::Minus]) {
            let opr: Token = self.previous().clone();
            let right: Expression = self.unary()?;
            return Ok(Expression::Unary(opr, Box::new(right)));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_tokentype(&[TokenType::False]) {
            return Ok(Expression::Literal(Value::Boolean(false)));
        }
        if self.match_tokentype(&[TokenType::True]) {
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

        if self.match_tokentype(&[TokenType::OpenParent]) {
            let expr: Expression = self.expression()?;
            self.consume(&TokenType::CloseParent, "Expected ')' after expression")?;
            return Ok(Expression::Group(Box::new(expr)));
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
    fn match_tokentype(&mut self, ttypes: &[TokenType]) -> bool {
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
