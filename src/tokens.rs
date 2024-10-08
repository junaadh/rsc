use std::{fmt, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, offset: usize) -> Self {
        Self {
            span: Span(start, offset),
            kind,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(pub usize, pub usize);

impl Span {
    pub fn join(&self, rhs: &Span) -> Span {
        Span(self.0, rhs.1)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.0, self.1)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    // KW
    Return,

    Ident,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    LiteralInt,
    LiteralFloat,
    LiteralBool,
    SemiColon,
    Comma,
    #[default]
    Eof,
    Error,
}

impl FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "return" => Ok(Self::Return),
            _ => Err(()),
        }
    }
}
