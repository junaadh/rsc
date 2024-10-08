use super::{Block, Ident, Type};
use crate::tokens::Span;

#[derive(Debug, Clone)]
pub struct Function {
    pub ty: Type,
    pub name: Ident,
    pub params: Vec<Parameter>,
    pub block: Block,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub ty: Type,
    pub ident: Ident,
    pub span: Span,
}
