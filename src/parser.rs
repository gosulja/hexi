use crate::ast::VarDecl;
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
                TokenType::Val => self.parse_var_decl(),
                TokenType::Ident => self.parse_identifier(),    // is this a function call or reference to identifier
                TokenType::String => self.parse_string(),
                TokenType::Number => self.parse_number(),
                _ => Err(format!("unexpected token {:?}", t))
            }
            None => Err("unexpected eof".to_string())
        }
    }

    fn parse_identifier(&mut self) -> Result<Expr, String> {
        let name = self.current_lex().unwrap().clone();

        // explicitly check if it's a print call
        // if name == "print" {
        //     self.parse_call()
        // } else {
        //     self.advance();
        //     Ok(Expr::Identifier(name))
        // }

        // advance to the next token
        self.advance();

        // if the next token is a '(' then treat it as a function call
        if self.check(&TokenType::LParen) {
            // pass the name of the function
            self.parse_call(name)
        // // if the next token is a '=' then treat it as a variable declaration
        // } else if self.check(&TokenType::Equals) {
        //     self.parse_var_decl(name)
        } else {
            Ok(Expr::Identifier(name))
        }
    }

    fn parse_call(&mut self, name: String) -> Result<Expr, String> {
        // self.advance();
        // let mut args: Vec<Expr> = Vec::new();

        // for now i'll just expect a number
        // match &self.current {
        //     Some(t) => match t.token_type {
        //         TokenType::Number => args.push(self.parse_expr()?),
        //         _ => return Err(format!("parse_call::unexpected token {:?}", t))
        //     }
        //     None => return Err(format!("unexpected eof"))
        // }

        // this was a test to see if args are handled well
        // args.push(Number(23f64));

        // cleaned up the args parsing section.
        self.consume(TokenType::LParen)?;

        let args = if self.check(&TokenType::RParen) { Vec::new() } else { self.parse_args()? };

        self.consume(TokenType::RParen)?;
        self.consume(TokenType::Semi)?;

        Ok(Expr::Call(Call::new( name, args )))
    }

    fn parse_var_decl(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::Val)?;

        let name = self.consume(TokenType::Ident)?.lexeme;

        self.consume(TokenType::Equals)?;

        let value = self.parse_expr()?;

        self.consume(TokenType::Semi)?;

        Ok(Expr::VarDecl(VarDecl::new(name, value)))
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        // first arg be pused
        args.push(self.parse_expr()?);

        // parse arg after comma, until it hits a )
        while self.check(&TokenType::Comma) {
            self.consume(TokenType::Comma)?; // go past comma
            // now we're expecting an expression after the comma
            // (1, 2, 3)
            //   ^ _
            // at right paren? stop parsing args, we're done
            if self.check(&TokenType::RParen) { break; }

            // then push the parsed expr as an arg
            args.push(self.parse_expr()?);
        }

        // return args vec as ok
        Ok(args)
    }

    fn parse_number(&mut self) -> Result<Expr, String> {
        let num = self.current_lex().unwrap().clone();
        self.advance();

        Ok(Expr::Number(num.parse().unwrap()))
    }

    fn parse_string(&mut self) -> Result<Expr, String> {
        let strr = self.current_lex().unwrap().clone();
        self.advance();

        Ok(Expr::String(strr))
    }
}