use core::fmt;

use crate::tokens::Span;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Primitive(PrimitveType),
}

#[derive(Clone, Eq)]
pub struct PrimitveType {
    pub kind: Primitives,
    pub span: Span,
}

impl PrimitveType {
    pub fn new(kind: Primitives, span: Span) -> Self {
        Self { kind, span }
    }
}

impl PartialEq for PrimitveType {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitives {
    Void,
    Int,
    Float,
    Double,
    Char,
    Bool,
}

impl fmt::Debug for PrimitveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Primitive type: {} [{}]", self.kind, self.span)
    }
}

impl fmt::Display for PrimitveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl fmt::Display for Primitives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Void => "void",
                Self::Int => "int",
                Self::Float => "float",
                Self::Double => "double",
                Self::Char => "char",
                Self::Bool => "bool",
            }
        )
    }
}
