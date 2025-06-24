use crate::ast::{Assignment, VarDecl, Expr, Call, BinaryOp, UnaryOp, If, Block};
use crate::lexer::{Lexer, Token, TokenType};

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

    // we can to have operations such as adding and subbing lower precedence than to mul and div, and mod.
    fn precedence(&self, token_type: TokenType) -> u8 {
        match token_type {
            TokenType::DblEquals | TokenType::Lt |
            TokenType::Gt | TokenType::Lte |
            TokenType::Gte | TokenType::Neq => 1,

            TokenType::Add | TokenType::Sub => 2,

            TokenType::Mul | TokenType::Div | TokenType::Mod => 3,
            _ => 0,
        }
    }

    // is this going to be a binar operation??
    fn is_binop(&self, token_type: TokenType) -> bool {
        matches!(token_type, TokenType::Add | TokenType::Sub |
            TokenType::Mul | TokenType::Div |
            TokenType::Mod | TokenType::DblEquals | TokenType::Lt |
            TokenType::Gt | TokenType::Lte |
            TokenType::Gte | TokenType::Neq)
    }

    fn current_lex(&self) -> Option<&String> {
        self.current.as_ref().map(|t| &t.lexeme)
    }

    pub fn parse(&mut self) -> Result<Vec<Expr>, String> {
        let mut exprs = Vec::new();

        while !self.check(&TokenType::Eof) {
            exprs.push(self.parse_expr()?);

            // optional semis
            if self.check(&TokenType::Semi) {
                self.advance();
            }
        }

        Ok(exprs)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        // match &self.current {
        //     Some(t) => match t.token_type {
        //         TokenType::Val => self.parse_var_decl(),
        //         TokenType::Ident => self.parse_identifier(),    // is this a function call or reference to identifier
        //         TokenType::String => self.parse_string(),
        //         TokenType::Number => self.parse_number(),
        //         _ => Err(format!("unexpected token {:?}", t))
        //     }
        //     None => Err("unexpected eof".to_string())
        // }

        self.parse_bin_expr(0)
    }

    fn parse_bin_expr(&mut self, precedence: u8) -> Result<Expr, String> {
        let mut left = self.parse_prim()?;

        while let Some(ref t) = self.current {
            if !self.is_binop(t.clone().token_type) {
                break;
            }

            let prec = self.precedence(t.clone().token_type);
            if prec < precedence {
                break;
            }

            let op = t.token_type.clone();
            self.advance();

            let right = self.parse_bin_expr(prec + 1)?;
            left = Expr::BinaryOp(BinaryOp::new(left, right, op));
        }

        Ok(left)
    }

    // move to parse_prim, parsing exprs "atoms"
    fn parse_prim(&mut self) -> Result<Expr, String> {
        match &self.current {
            Some(t) => match t.token_type {
                TokenType::Sub => self.parse_unary(),
                TokenType::Val => self.parse_var_decl(),
                TokenType::Ident => self.parse_identifier(),
                TokenType::String => self.parse_string(),
                TokenType::Number => self.parse_number(),
                TokenType::LParen => self.parse_grouped(),
                TokenType::LBrace => Ok(Expr::Block(self.parse_block()?)),
                TokenType::If => self.parse_if(),
                _ => Err(format!("unexpected token {:?}", t))
            }
            None => Err("unexpected eof".to_string())
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        let op = self.current.clone().unwrap().token_type.clone();
        self.advance();
        let operand = self.parse_prim()?;
        Ok(Expr::UnaryOp(UnaryOp::new(operand, op)))
    }

    fn parse_grouped(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::LParen)?;
        let expr = self.parse_bin_expr(0)?;
        self.consume(TokenType::RParen)?;
        Ok(expr)
    }

    fn parse_if(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::If)?;

        let cond = self.parse_expr()?;
        // dont gotta consume L brace, because blocks are started with {
        let block = self.parse_block()?;
        let else_block = if self.check(&TokenType::Else) {
            self.consume(TokenType::Else)?;
            // Some(self.parse_block()?)
            // to add support for else if
            // check if the next token is an if token,
            // if it is then recursively parse it
            if self.check(&TokenType::If) {
                Some(Block::new(vec![self.parse_if()?]))
            } else {
                Some(self.parse_block()?)
            }
        } else {
            None
        };

        Ok(Expr::If(If::new(cond, block, else_block)))
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.consume(TokenType::LBrace)?;

        // these really should be statements, but whatever
        let mut exprs = Vec::new();
        // parse until we reach the } or EOF
        while !self.check(&TokenType::RBrace) && !self.check(&TokenType::Eof) {
            exprs.push(self.parse_expr()?);

            // optional semi
            if self.check(&TokenType::Semi) {
                self.advance();
            }
        }

        self.consume(TokenType::RBrace)?;
        Ok(Block::new(exprs))
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

        // check if we encounter a double colon '::' for module access, do this first
        if self.check(&TokenType::DblColon) {
            self.consume(TokenType::DblColon)?;

            // ident after ::
            let fn_name = self.consume(TokenType::Ident)?.lexeme;

            // function call?
            return if self.check(&TokenType::LParen) {
                self.parse_mod_call(name, fn_name)
            } else {
                // no module call but a reference to const perhaps?
                // math::PI for example
                Ok(Expr::Identifier(format!("{}::{}", name, fn_name)))
            }
        }

        // if the next token is a '(' then treat it as a function call
        if self.check(&TokenType::LParen) {
            // pass the name of the function
            self.parse_call(name)
            // // if the next token is a '=' then treat it as a variable declaration
            // } else if self.check(&TokenType::Equals) {
            //     self.parse_var_decl(name)

        // we are now expecting this: `ident = ...` , assignment
        } else if self.check(&TokenType::Equals) {
            self.parse_assignment(name)
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
        // self.consume(TokenType::Semi)?;

        Ok(Expr::Call(Call::new( name, args )))
    }

    fn parse_mod_call(&mut self, module: String, name: String) -> Result<Expr, String> {
        self.consume(TokenType::LParen)?;
        let args = if self.check(&TokenType::RParen) { Vec::new() } else { self.parse_args()? };
        self.consume(TokenType::RParen)?;
        // self.consume(TokenType::Semi)?;

        Ok(Expr::Call(Call::new_from_module(module, name, args)))
    }

    fn parse_assignment(&mut self, name: String) -> Result<Expr, String> {
        self.consume(TokenType::Equals)?;

        let assignee = self.parse_bin_expr(0)?;

        // self.consume(TokenType::Semi)?;

        Ok(Expr::Assignment(Assignment::new(name, assignee)))
    }

    fn parse_var_decl(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::Val)?;

        let name = self.consume(TokenType::Ident)?.lexeme;

        self.consume(TokenType::Equals)?;

        let value = self.parse_expr()?;

        // self.consume(TokenType::Semi)?;

        Ok(Expr::VarDecl(VarDecl::new(name, value)))
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();

        // first arg be pused
        args.push(self.parse_bin_expr(0)?);

        // parse arg after comma, until it hits a )
        while self.check(&TokenType::Comma) {
            self.consume(TokenType::Comma)?; // go past comma
            // now we're expecting an expression after the comma
            // (1, 2, 3)
            //   ^ _
            // at right paren? stop parsing args, we're done
            if self.check(&TokenType::RParen) { break; }

            // then push the parsed expr as an arg
            args.push(self.parse_bin_expr(0)?);
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