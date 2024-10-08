use super::{Ident, Literal};

#[derive(Debug, Clone)]
pub enum Expression {
    Primary(PrimaryExpression),
}

#[derive(Debug, Clone)]
pub enum PrimaryExpression {
    Literal(Literal),
    Ident(Ident),
}
