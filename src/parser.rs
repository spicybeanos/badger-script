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

    pub fn parse_statement(&mut self) -> Result<Vec<Option<Statement>>, String> {
        let mut stmt: Vec<Option<Statement>> = Vec::<Option<Statement>>::new();
        while !self.is_at_end() {
            stmt.push(self.declaration());
        }

        return Ok(stmt);
    }
    fn var_declearation(&mut self, vtype: TokenType) -> Result<Statement, String> {
        let mut name: String = "".to_string();

        self.consume_identifier(&mut name, "Expected variable name")?;
        let mut init: Expression;

        let idx = self.peek().index - 1;

        match vtype {
            TokenType::Num => init = Expression::Literal(Value::Number(0.0), idx),
            TokenType::Bool => init = Expression::Literal(Value::Boolean(false), idx),
            TokenType::Str => init = Expression::Literal(Value::StringVal("".to_string()), idx),
            TokenType::Var => init = Expression::Literal(Value::Number(0.0), idx),
            _ => return Err("Illegal type for a variable".to_string()),
        }

        if self.match_tokentype(&[TokenType::Asign]) {
            init = self.parse_expression()?;
        }

        self.consume(&TokenType::EoStmt, "Expect ';' after variable declaration.")?;

        return Ok(Statement::VarDecl(name, init, vtype, idx));
    }
    fn declaration(&mut self) -> Option<Statement> {
        let mut vtype: Option<TokenType> = Option::None;

        if self.match_tokentype(&[TokenType::Num]) {
            vtype = Some(TokenType::Num);
        } else if self.match_tokentype(&[TokenType::Bool]) {
            vtype = Some(TokenType::Bool);
        } else if self.match_tokentype(&[TokenType::Str]) {
            vtype = Some(TokenType::Str);
        } else if self.match_tokentype(&[TokenType::Var]) {
            vtype = Some(TokenType::Var);
        }

        match vtype {
            Some(typ) => {
                let vd = self.var_declearation(typ);
                match vd {
                    Result::Ok(dec) => return Some(dec),
                    Result::Err(ex) => {
                        println!("Error: {}", ex);
                        self.synchronize();
                        return None;
                    }
                }
            }
            _ => {}
        }

        let st = self.statement();

        match st {
            Ok(stmt) => return Some(stmt),
            Err(_) => {
                self.synchronize();
                return None;
            }
        }
    }
    fn block(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements: Vec<Statement> = Vec::<Statement>::new();
        while !self.check(&TokenType::CloseBrace) && !self.is_at_end() {
            let dec = self.declaration();
            match dec {
                Some(st) => statements.push(st),
                _ => {}
            }
        }

        self.consume(&TokenType::CloseBrace, "Expected '}' after block")?;

        return Ok(statements);
    }
    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_tokentype(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_tokentype(&[TokenType::OpenBrace]) {
            return Ok(Statement::Block(self.block()?));
        }
        if self.match_tokentype(&[TokenType::If]) {
            return self.if_statement();
        }

        return self.expr_statement();
    }
    fn if_statement(&mut self) -> Result<Statement, String> {
        self.consume(&TokenType::OpenParent, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::CloseParent, "Expect ')' after condition")?;
        let then = self.statement()?;
        let mut else_branch : Option<Statement> = None;

        if self.match_tokentype(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }
        return Ok(Statement::IfStmt(condition, Box::new(then), Box::new(else_branch)));
    }
    fn return_statement(&mut self) -> Result<Statement, String> {
        let value = self.expression()?;
        self.consume(&TokenType::EoStmt, "Expected ';' after value.")?;
        return Ok(Statement::Return(value));
    }
    fn expr_statement(&mut self) -> Result<Statement, String> {
        let value = self.expression()?;
        self.consume(&TokenType::EoStmt, "Expected ';' after value.")?;
        return Ok(Statement::Expr(value));
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        self.expression()
    }
    pub fn expression(&mut self) -> Result<Expression, String> {
        self.assignment()
    }
    fn assignment(&mut self) -> Result<Expression, String> {
        let expr = self.boolean_logic()?;

        if self.match_tokentype(&[TokenType::Asign]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                Expression::Variable(name, idx) => {
                    return Ok(Expression::Assignment(name, Box::new(value), idx));
                }
                _ => {}
            }

            return self.error_ex(&equals, "Invalid assignment target");
        }

        return Ok(expr);
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
        let mut index = 0;
        if self.match_tokentype_index(&[TokenType::False], &mut index) {
            return Ok(Expression::Literal(Value::Boolean(false), index));
        }
        if self.match_tokentype_index(&[TokenType::True], &mut index) {
            return Ok(Expression::Literal(Value::Boolean(true), index));
        }

        if let Some((num, id)) = self.match_number_literal() {
            return Ok(Expression::Literal(Value::Number(num), id));
        }
        if let Some((b, id)) = self.match_boolean_literal() {
            return Ok(Expression::Literal(Value::Boolean(b), id));
        }
        if let Some((string_value, id)) = self.match_string_literal() {
            return Ok(Expression::Literal(Value::StringVal(string_value), id));
        }
        if let Some((identifer, id)) = self.match_identifier() {
            return Ok(Expression::Variable(identifer, id));
        }

        if let Some((symbol, id)) = self.match_symbol() {
            return Ok(Expression::SpecialSymbol(symbol, id));
        }

        if self.match_tokentype(&[TokenType::OpenParent]) {
            let expr: Expression = self.expression()?;
            self.consume(&TokenType::CloseParent, "Expected ')' after expression")?;
            return Ok(Expression::Group(Box::new(expr)));
        }

        // if self.match_tokentype(&[TokenType::OpenSquare]) {
        //     let expr: Expression = self.expression()?;
        //     self.consume(&TokenType::CloseSquare, "Expected ')' after expression")?;
        //     return Ok(Expression::Index(Box::new(expr)));
        // }

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
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().ttype == TokenType::EoStmt {
                return;
            }

            match self.peek().ttype {
                TokenType::Fxn => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Var => return,
                TokenType::Num => return,
                TokenType::Str => return,
                TokenType::Bool => return,
                TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
    fn consume(&mut self, ttype: &TokenType, err_msg: &str) -> Result<&Token, String> {
        if self.check(ttype) {
            return Ok(self.advance());
        }
        return self.error(self.peek(), err_msg);
    }

    fn consume_identifier(&mut self, id: &mut String, err_msg: &str) -> Result<&Token, String> {
        if self.check_identifier(id) {
            return Ok(self.advance());
        }
        return self.error(self.peek(), err_msg);
    }

    fn check_identifier(&self, id: &mut String) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let TokenType::Identifier(s) = &self.peek().ttype {
            *id = s.to_string();
            return true;
        }

        return false;
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
    fn match_tokentype_index(&mut self, ttypes: &[TokenType], index: &mut usize) -> bool {
        *index = self.peek().index;
        for tt in ttypes {
            if self.check(tt) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn match_number_literal(&mut self) -> Option<(f64, usize)> {
        let index = self.peek().index;
        if let TokenType::NumberLiteral(num) = self.peek().ttype {
            self.advance(); // Move past the number
            Some((num, index)) // Return the extracted number
        } else {
            None
        }
    }
    fn match_boolean_literal(&mut self) -> Option<(bool, usize)> {
        let index = self.peek().index;
        if let TokenType::BooleanLiteral(b) = self.peek().ttype {
            self.advance(); // Move past the number
            Some((b, index)) // Return the extracted number
        } else {
            None
        }
    }
    fn match_string_literal(&mut self) -> Option<(String, usize)> {
        let token_type = self.peek().ttype.clone(); // Clone the token type to avoid borrowing issues
        let index = self.peek().index;
        if let TokenType::StringLiteral(s) = token_type {
            self.advance(); // Now it's safe to advance
            Some((s, index)) // Return the extracted string
        } else {
            None
        }
    }
    fn match_identifier(&mut self) -> Option<(String, usize)> {
        let token_type = self.peek().ttype.clone(); // Clone the token type to avoid borrowing issues
        let index = self.peek().index;
        if let TokenType::Identifier(s) = token_type {
            self.advance(); // Now it's safe to advance
            Some((s, index)) // Return the extracted string
        } else {
            None
        }
    }
    fn match_symbol(&mut self) -> Option<(String, usize)> {
        let token_type = self.peek().ttype.clone(); // Clone the token type to avoid borrowing issues
        let index = self.peek().index;
        if let TokenType::Identifier(s) = token_type {
            self.advance(); // Now it's safe to advance
            Some((s, index)) // Return the extracted string
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
