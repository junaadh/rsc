use crate::{
    ast::{
        Block, Decleration, Expression, Function, Ident, Literal, LiteralKind, Parameter,
        PrimaryExpression, Primitives, PrimitveType, Program, Statement, Type,
    },
    error::RscError,
    lexer::Lexer,
    tokens::{Token, TokenKind},
    Res,
};

pub struct Parser<'p> {
    source: &'p str,
    tokens: Vec<Token>,
    pos: usize,
}

impl<'p> Parser<'p> {
    pub fn new(source: &'p str) -> Self {
        let tokens = Lexer::new(source).tokenize().collect::<Vec<_>>();
        Self {
            source,
            tokens,
            pos: 0,
        }
    }

    fn bump(&mut self) -> Token {
        let tok = *self.tokens.get(self.pos).unwrap_or(&Token::default());
        self.pos += 1;
        tok
    }

    fn peek(&self) -> Token {
        *self.tokens.get(self.pos).unwrap_or(&Token::default())
    }

    // fn current(&self) -> Token {
    //     *self
    //         .tokens
    //         .get(self.pos.wrapping_sub(1))
    //         .unwrap_or(&Token::default())
    // }

    fn slice(&self, token: &Token) -> &str {
        &self.source[token.span.0..token.span.1]
    }

    fn expect(&mut self, expected: TokenKind) -> Res<Token> {
        let next = self.bump();
        match next.kind {
            x if x == expected => Ok(next),
            _ => Err(RscError::UnexpectedToken(
                Box::new(self.slice(&next).to_owned()),
                next,
            )),
        }
    }

    fn literal(&mut self, bit32: bool) -> Res<PrimaryExpression> {
        let token = self.bump();

        match token.kind {
            TokenKind::LiteralInt if bit32 => {
                let int = self.slice(&token).parse::<i32>()?;
                Ok(PrimaryExpression::Literal(Literal::new(
                    LiteralKind::Int(int),
                    token.span,
                )))
            }
            TokenKind::LiteralInt if !bit32 => todo!(),
            TokenKind::LiteralFloat if bit32 => {
                let float = self.slice(&token).parse::<f32>()?;
                Ok(PrimaryExpression::Literal(Literal::new(
                    LiteralKind::Float(float),
                    token.span,
                )))
            }
            TokenKind::LiteralFloat if !bit32 => {
                let double = self.slice(&token).parse::<f64>()?;
                Ok(PrimaryExpression::Literal(Literal::new(
                    LiteralKind::Double(double),
                    token.span,
                )))
            }
            _ => Err(RscError::UnexpectedToken(
                Box::new(self.slice(&token).to_owned()),
                token,
            )),
        }
    }

    fn ident(&mut self) -> Res<Ident> {
        let token = self.bump();

        match token.kind {
            TokenKind::Ident => Ok(Ident::new(self.slice(&token), token.span)),
            _ => Err(RscError::UnexpectedToken(
                Box::new(self.slice(&token).to_owned()),
                token,
            )),
        }
    }

    fn primary(&mut self, bit32: bool) -> Res<Expression> {
        let token = self.peek();

        match token.kind {
            TokenKind::LiteralInt | TokenKind::LiteralFloat | TokenKind::LiteralBool => {
                self.literal(bit32).map(Expression::Primary)
            }
            _ => todo!(),
        }
    }

    fn expression(&mut self) -> Res<Expression> {
        let expr = self.primary(true)?;

        Ok(expr)
    }

    fn return_statement(&mut self) -> Res<Statement> {
        self.expect(TokenKind::Return)?;

        let expr = if self.peek().kind != TokenKind::SemiColon {
            Some(self.expression()?)
        } else {
            None
        };

        self.expect(TokenKind::SemiColon)?;

        Ok(Statement::Return(expr))
    }

    fn statement(&mut self) -> Res<Statement> {
        match self.peek().kind {
            TokenKind::Return => Ok(self.return_statement()?),
            _ => todo!(),
        }
    }

    fn block(&mut self) -> Res<Block> {
        let start_span = self.peek().span;

        self.expect(TokenKind::OpenBrace)?;

        let mut statements = Vec::new();
        while self.peek().kind != TokenKind::CloseBrace {
            statements.push(self.statement()?);
        }

        let end = self.expect(TokenKind::CloseBrace)?.span;

        let span = start_span.join(&end);

        Ok(Block { statements, span })
    }

    fn ty_(&mut self) -> Res<Type> {
        let token = self.bump();

        match token.kind {
            TokenKind::Ident => Ok(Type::Primitive(PrimitveType::new(
                Primitives::Int,
                token.span,
            ))),
            _ => todo!(),
        }
    }

    fn parameters(&mut self) -> Res<Parameter> {
        let start = self.peek();

        if self.slice(&start) == "void" {
            let ident = self.ident()?;
            Ok(Parameter {
                ty: Type::Primitive(PrimitveType {
                    kind: Primitives::Void,
                    span: ident.span,
                }),
                ident,
                span: start.span,
            })
        } else {
            let ty = self.ty_()?;
            let param = self.ident()?;
            let span = start.span.join(&param.span);

            Ok(Parameter {
                ty,
                ident: param,
                span,
            })
        }
    }

    fn function(&mut self) -> Res<Function> {
        let start = self.peek().span;

        let ty = self.ty_()?;
        let ident = self.ident()?;

        self.expect(TokenKind::OpenParen)?;
        let mut paramters = Vec::new();
        while self.peek().kind != TokenKind::CloseParen {
            paramters.push(self.parameters()?);

            if self.peek().kind == TokenKind::Comma {
                self.bump();
            }
        }

        self.expect(TokenKind::CloseParen)?;

        let block = self.block()?;

        let span = start.join(&block.span);

        Ok(Function {
            ty,
            name: ident,
            params: paramters,
            block,
            span,
        })
    }

    fn declerations(&mut self) -> Res<Decleration> {
        let token = self.peek();

        match token.kind {
            TokenKind::Ident => Ok(Decleration::Fuction(self.function()?)),
            _ => todo!(),
        }
    }

    pub fn program(&mut self) -> Res<Program> {
        let mut decl = Vec::new();
        while self.peek().kind != TokenKind::Eof {
            decl.push(self.declerations()?);
        }

        Ok(Program { declarations: decl })
    }
}
