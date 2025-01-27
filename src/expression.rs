use crate::tokenizer::TokenType;

pub enum Expression <T : std::cmp::PartialOrd>{
    Literal(T),
    Unary(TokenType,Box<Expression<T>>),
    Binary(Box<Expression<T>>,TokenType,Box<Expression<T>>),
    Group(Box<Expression<T>>)
}

pub fn accept<T : std::cmp::PartialOrd>(expr:Expression<T>) -> T {
    match expr {
        Expression::Literal(data) => data,
        
    }
}
