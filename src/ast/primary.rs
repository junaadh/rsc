use crate::tokens::Span;

use super::{Primitives, PrimitveType, Type};

#[derive(Debug, Clone)]
pub struct Ident {
    pub ident: Box<str>,
    pub span: Span,
}

impl Ident {
    pub fn new(ident: &str, span: Span) -> Self {
        Self {
            ident: ident.to_string().into_boxed_str(),
            span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

impl Literal {
    pub fn new(kind: LiteralKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Int(i32),
    Float(f32),
    Double(f64),
    Char(char),
    Bool(bool),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
    }
}

impl Eq for Ident {}

impl From<Literal> for Type {
    fn from(value: Literal) -> Self {
        match value.kind {
            LiteralKind::Int(_) => Type::Primitive(PrimitveType {
                kind: Primitives::Int,
                span: value.span,
            }),
            LiteralKind::Float(_) => Type::Primitive(PrimitveType {
                kind: Primitives::Float,
                span: value.span,
            }),
            LiteralKind::Double(_) => Type::Primitive(PrimitveType {
                kind: Primitives::Double,
                span: value.span,
            }),
            LiteralKind::Char(_) => Type::Primitive(PrimitveType {
                kind: Primitives::Char,
                span: value.span,
            }),
            LiteralKind::Bool(_) => Type::Primitive(PrimitveType {
                kind: Primitives::Bool,
                span: value.span,
            }),
        }
    }
}
