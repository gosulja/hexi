use crate::lexer::{Lexer, Token, TokenType};
use crate::ast::{Expr, Call};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Option<Token>
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next();
        Parser { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next();
    }

    fn check(&self, target_type: &TokenType) -> bool {
        if let Some(ref token) = self.current {
            token.token_type == *target_type
        } else {
            *target_type == TokenType::Eof
        }
    }

    fn consume(&mut self, expect: TokenType) -> Result<Token, String> {
        if self.check(&expect) {
            let t = self.current.clone();
            self.advance();
            t.ok_or_else(|| "unexpected eof".to_string())
        } else {
            Err(format!("expected {:?} but found {:?}", expect, self.current.as_ref().map(|t| &t.token_type)))
        }
    }

    fn current_lex(&self) -> Option<&String> {
        self.current.as_ref().map(|t| &t.lexeme)
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut exprs = Vec::new();

        while !self.check(&TokenType::Eof) {
            exprs.push(self.parse_expr()?);
        }

        Ok(exprs)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        match &self.current {
            Some(t) => match t.token_type {
                TokenType::Ident => self.parse_identifier(),    // is this a function call or reference to identifier
                TokenType::Number => self.parse_number(),
                _ => Err(format!("unexpected tokennnnn {:?}", t))
            }
            None => Err("unexpected eof".to_string())
        }
    }

    fn parse_identifier(&mut self) -> Result<Expr, String> {
        let name = self.current_lex().unwrap().clone();

        // explicitly check if it's a print call
        if name == "print" {
            self.parse_call()
        } else {
            self.advance();
            Ok(Expr::Identifier(name))
        }
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        let name = self.current_lex().unwrap().clone();
        self.advance();

        let mut args: Vec<Expr> = Vec::new();

        // for now we'll just expect a number
        match &self.current {
            Some(t) => match t.token_type {
                TokenType::Number => args.push(self.parse_expr()?),
                _ => return Err(format!("unexpected token {:?}", t))
            }
            None => return Err(format!("unexpected eof"))
        }

        self.advance();
        Ok(Expr::Call(Call { name, args }))
    }

    fn parse_number(&mut self) -> Result<Expr, String> {
        let num = self.current_lex().unwrap().clone();
        self.advance();

        Ok(Expr::Number(num.parse().unwrap()))
    }
}