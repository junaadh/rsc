use std::str::Chars;

use crate::tokens::{Token, TokenKind};

pub const EOF_CHAR: char = '\0';

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    token_start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            token_start: 0,
        }
    }

    pub fn advance(&mut self) -> char {
        self.chars.next().unwrap_or(EOF_CHAR)
    }

    pub fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn second(&self) -> char {
        let mut char = self.chars.clone();
        char.next();
        char.next().unwrap_or(EOF_CHAR)
    }

    pub fn pos(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    pub fn content(&self) -> &str {
        let end = self.pos();
        &self.source[self.token_start..end]
    }

    pub fn slice(&self, token: &Token) -> &str {
        &self.source[token.span.0..token.span.1]
    }

    pub fn token(&self, kind: TokenKind) -> Token {
        let end = self.pos();
        Token::new(kind, self.token_start, end)
    }

    pub fn reset_ptr(&mut self) {
        self.token_start = self.pos();
    }

    pub fn advance_while<T>(&mut self, cond: T)
    where
        T: Fn(char) -> bool,
    {
        while self.first() != EOF_CHAR && cond(self.first()) {
            self.advance();
        }
    }

    pub fn kind(&mut self) -> TokenKind {
        self.advance_while(|x| matches!(x, ' ' | '\n' | '\t'));
        self.reset_ptr();

        let char = self.advance();

        match char {
            'A'..='Z' | 'a'..='z' | '_' => {
                self.advance_while(|x| matches!(x, 'A'..='Z' | 'a'..='z' | '_'));
                if matches!(self.content(), "false" | "bool") {
                    TokenKind::LiteralBool
                } else if let Ok(t) = self.content().parse::<TokenKind>() {
                    t
                } else {
                    TokenKind::Ident
                }
            }
            x if x.is_ascii_digit() => {
                self.advance_while(|x| x.is_ascii_digit());
                if self.first() == '.' {
                    self.advance_while(|x| x.is_ascii_digit());
                    TokenKind::LiteralFloat
                } else {
                    TokenKind::LiteralInt
                }
            }
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ';' => TokenKind::SemiColon,
            EOF_CHAR => TokenKind::Eof,
            _ => TokenKind::Error,
        }
    }

    fn lex(&mut self) -> Token {
        let kind = self.kind();
        self.token(kind)
    }

    pub fn tokenize(mut self) -> impl Iterator<Item = Token> + 'a {
        std::iter::from_fn(move || {
            let token = self.lex();
            match token.kind {
                TokenKind::Eof => None,
                TokenKind::Error => panic!(
                    "Unexpected character found: '{}' @ {:?}",
                    self.slice(&token),
                    token.span,
                ),
                _ => Some(token),
            }
        })
    }
}
