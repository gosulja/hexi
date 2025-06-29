use crate::ast::{Assignment, BinaryOp, Block, Call, Expr, If, IndexAccess, MethodCall, UnaryOp, VarDecl, Include, FieldAccess, Collection, CEntry};
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
        let mut left = self.parse_postfix()?;

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
                TokenType::Include => self.parse_include(),
                TokenType::Sub => self.parse_unary(),
                TokenType::Val => self.parse_var_decl(),
                TokenType::Ident => self.parse_identifier(),
                TokenType::String => self.parse_string(),
                TokenType::Number => self.parse_number(),
                TokenType::LParen => self.parse_grouped(),
                TokenType::LBracket => self.parse_collection(),
                TokenType::LBrace => Ok(Expr::Block(self.parse_block()?)),
                TokenType::If => self.parse_if(),
                _ => Err(format!("unexpected token {:?}", t))
            }
            None => Err("unexpected eof".to_string())
        }
    }

    fn parse_include(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::Include)?;  // consume 'include' keyword

        // expect identifier
        let module_name = if self.check(&TokenType::Ident) {
            let name = self.current_lex().unwrap().clone();
            self.advance();
            name
        } else {
            return Err("expected identifier after 'include'".to_string());
        };

        Ok(Expr::Include(Include::new(module_name)))
    }

    // postfix => some_array[0] or some_array.empty()
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut e = self.parse_prim()?;

        loop {
            match &self.current {
                Some(t) => match t.token_type {
                    TokenType::LBracket => {
                        // some_array[idx]
                        self.consume(TokenType::LBracket)?; // get past [
                        let idx = self.parse_expr()?;
                        self.consume(TokenType::RBracket)?; // get pas ]
                        // at this post we've parsed [idx]
                        // so set the current expr to this index access
                        e = Expr::IndexAccess(IndexAccess::new(e, idx));
                    },
                    TokenType::Dot => {
                        // some_obj.func(args...)
                        self.consume(TokenType::Dot)?;  // get past .
                        // now get method from obj
                        let meth = self.consume(TokenType::Ident)?.lexeme;
                        if self.check(&TokenType::LParen) { // we calling it?
                            self.consume(TokenType::LParen)?;   // get past (
                            // if we're not an empty () call parse_args, if we are empty, just create an empty vec
                            let args = if self.check(&TokenType::RParen) { Vec::new() } else { self.parse_args()? };
                            self.consume(TokenType::RParen)?;   // get past )
                            e = Expr::MethodCall(MethodCall::new(e, meth, args));
                        } else {
                            // for shit like, person.name
                            e = Expr::FieldAccess(FieldAccess::new(e, meth))
                        }
                    },
                    _ => break,
                },
                None => break
            }
        }

        Ok(e)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        let op = self.current.clone().unwrap().token_type.clone();
        self.advance();
        let operand = self.parse_postfix()?;
        Ok(Expr::UnaryOp(UnaryOp::new(operand, op)))
    }

    fn parse_grouped(&mut self) -> Result<Expr, String> {
        self.consume(TokenType::LParen)?;
        let expr = self.parse_bin_expr(0)?;
        self.consume(TokenType::RParen)?;
        Ok(expr)
    }

    // fn parse_array(&mut self) -> Result<Expr, String> {
    //     self.consume(TokenType::LBracket)?; // get passt [
    //     // let mut values = Vec::new(); // create a vec for the values within the array
    //     // empty array? val some_array = []
    //     if self.check(&TokenType::RBracket) {
    //         self.consume(TokenType::RBracket)?;
    //         return Ok(Expr::Array(Array::new(Vec::new())))
    //     }
    //
    //     // push first
    //     // val some_vec = [1, 2, 3, 4]
    //     //                 ^
    //     let first = self.parse_expr()?;
    //
    //     // here we would want to check if an '=' is after, first
    //     // only if first is an identifier (for key)
    //     // basically, check for: [key = value, ...]
    //     if let Expr::Identifier(k) = first {
    //         // is an equals? if there isnt
    //         // just parse a normal array
    //         // instead of an objecy
    //         if self.check(&TokenType::Equals) {
    //             return self.parse_obj(k);
    //         }
    //     }
    //
    //     // we can od it like this now
    //     let mut values = vec![first];
    //
    //     // parse next value when we at a comma
    //     while self.check(&TokenType::Comma) {
    //         self.consume(TokenType::Comma)?;    // we at a comma? eat it.
    //         // but lets also allow for a trailing comma
    //         // val some = [1, 2, 3,]
    //         //                    ^
    //         if self.check(&TokenType::RBracket) { break; }
    //
    //         // and then parse the value
    //         values.push(self.parse_expr()?);
    //     }
    //
    //     self.consume(TokenType::RBracket)?; // end it of with ]
    //     Ok(Expr::Array(Array::new(values)))
    // }

    fn parse_collection(&mut self) -> Result<Expr, String> {
        // since a collection is an array and object
        // in one, we need to keep this in mind
        // so we need to conditionally parse this structure
        self.consume(TokenType::LBracket)?;

        // allow empty
        // collection definitions
        // e.g val people = []
        // initially set to empty
        if self.check(&TokenType::RBracket) {
            self.consume(TokenType::RBracket)?;
            return Ok(Expr::Collection(Collection::empty()));   // use the handy empty constructor
        }

        let mut entries = vec![];
        let mut idx = 0;    // index in the collection, default to 0

        loop {
            // we need to specially handle the case where
            // an identifier is followed by an equals
            // carefully do this so we dont invoke a var decl
            if self.check(&TokenType::Ident) {
                let key = self.current_lex().unwrap().clone();
                self.advance(); // eat identifier

                if self.check(&TokenType::Equals) {
                    // so now we're parsing a key pair with value
                    // e.g [a = 1]
                    self.consume(TokenType::Equals)?;
                    let val = self.parse_expr()?;
                    entries.push(CEntry::Keyed(key, val));
                } else {
                    // this is an ordinary identifier reference which needs to be evaluated
                    // so just go back and parse it as a normal expr
                    let first = Expr::Identifier(key);  // since we've lost the previous identifier by eating it, simply create a new one
                    entries.push(CEntry::Indexed(first));
                    idx += 1;
                }
            } else {
                // parse the first entry in the collection
                let first = self.parse_expr()?;

                // is it a key -> value ??
                if self.check(&TokenType::Equals) {
                    self.consume(TokenType::Equals)?;
                    let value = self.parse_expr()?;

                    // because we have different types of entries:
                    // pub enum CEntry {
                    //     Indexed(Expr),                      // [1, 2, 3]
                    //     Keyed(String, Expr),                // [name = "value"] -> like a map, so key -> value
                    //     NumKeyed(f64, Expr),                // [1 = "first", 2 = "second"] - num -> value
                    // }
                    // we need to differentriate the key
                    match first {
                        // map, key -> value
                        // Expr::Identifier(k) => {
                        //     println!("{}", k);
                        //     entries.push(CEntry::Keyed(k, value))
                        // },
                        // lets also allow for string literals to be keys
                        // map, string -> value
                        Expr::String(s) => entries.push(CEntry::Keyed(s, value)),
                        // indexed map (i guess lol?), num -> value
                        Expr::Number(n) => entries.push(CEntry::NumKeyed(n, value)),
                        // to be safe
                        _ => return Err("invalid key usage type for collection structure entry.".to_string()),
                    }
                } else {
                    // okay this is an ordinary indexed entry,
                    // so automatically increment the index
                    entries.push(CEntry::Indexed(first));
                    idx += 1;
                }
            }

            // so we want to continue if we encounter a comma
            if self.check(&TokenType::Comma) {
                self.consume(TokenType::Comma)?;    // eat the comma

                // lets be nice and allow for trailing commas
                // so shit like this: [1, 2, 3,]
                //                            ^
                if self.check(&TokenType::RBracket) {
                    break;
                }
            } else if self.check(&TokenType::RBracket) {
                break;
            } else {
                return Err("expected ',' or ']' to terminate collection definition.".to_string())
            }
        }

        self.consume(TokenType::RBracket)?;
        Ok(Expr::Collection(Collection::new(entries)))
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