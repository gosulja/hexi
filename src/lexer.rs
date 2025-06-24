use std::collections::{HashMap};
use std::ptr::null;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident,
    Number,
    String,
    LParen,
    RParen,
    Comma,
    DblEquals,  // ==
    Lt,         // <
    Gt,         // >
    Gte,        // >=
    Lte,        // <=
    Neq,        // !=
    Equals,
    Semi,
    Val,        // variable declaration
    DblColon,   // ::
    LBrace,     // {
    RBrace,     // }
    Colon,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    If,
    Else,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

fn make_token(token_type: TokenType, lexeme: String) -> Token {
    Token { token_type, lexeme }
}

pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        let mut keywords = HashMap::new();

        keywords.insert("val", TokenType::Val);
        keywords.insert("if", TokenType::If);
        keywords.insert("else", TokenType::Else);

        Lexer { source, pos: 0, keywords }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.pos + 1)
    }

    pub fn next(self: &mut Lexer<'a>) -> Option<Token> {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break
            }
        }

        let c = self.current()?;
        if c.is_alphabetic() || c == '_' {
            Some(self.process_identifier())
        } else if c.is_numeric() {
            Some(self.process_number())
        } else if c == '"' || c == '\'' {
            Some(self.process_string())
        } else if c == ';' {
            self.advance();
            Some(make_token(TokenType::Semi, ";".to_string()))
        } else if c == '(' {
            self.advance();
            Some(make_token(TokenType::LParen, "(".to_string()))
        } else if c == ')' {
            self.advance();
            Some(make_token(TokenType::RParen, ")".to_string()))
        } else if c == '{' {
            self.advance();
            Some(make_token(TokenType::LBrace, "{".to_string()))
        } else if c == '}' {
            self.advance();
            Some(make_token(TokenType::RBrace, "}".to_string()))
        } else if c == ',' {
            self.advance();
            Some(make_token(TokenType::Comma, ",".to_string()))
        // } else if c == '=' {
        //     self.advance();
        //     Some(make_token(TokenType::Equals, "=".to_string()))
        } else if c == '+' {
            self.advance();
            Some(make_token(TokenType::Add, "+".to_string()))
        } else if c == '-' {
            self.advance();
            Some(make_token(TokenType::Sub, "-".to_string()))
        } else if c == '*' {
            self.advance();
            Some(make_token(TokenType::Mul, "*".to_string()))
        } else if c == '/' {
            self.advance();
            Some(make_token(TokenType::Div, "/".to_string()))
        } else if c == '%' {
            self.advance();
            Some(make_token(TokenType::Mod, "%".to_string()))
        } else if c == ':' {
            self.advance();
            if self.source.chars().nth(self.pos) == Some(':') {
                self.advance();
                Some(make_token(TokenType::DblColon, "::".to_string()))
            } else {
                Some(make_token(TokenType::Colon, ":".to_string()))
            }
        } else if c == '=' {
            self.advance();
            if self.source.chars().nth(self.pos) == Some('=') {
                self.advance();
                Some(make_token(TokenType::DblEquals, "==".to_string()))
            } else {
                Some(make_token(TokenType::Equals, "=".to_string()))
            }
        } else if c == '<' {
            self.advance();
            if self.source.chars().nth(self.pos) == Some('=') {
                self.advance();
                Some(make_token(TokenType::Lte, "<=".to_string()))
            } else {
                Some(make_token(TokenType::Lt, "<".to_string()))
            }
        } else if c == '>' {
            self.advance();
            if self.source.chars().nth(self.pos) == Some('=') {
                self.advance();
                Some(make_token(TokenType::Gte, ">=".to_string()))
            } else {
                Some(make_token(TokenType::Gt, ">".to_string()))
            }
        } else if c == '!' {
            self.advance();
            if self.source.chars().nth(self.pos) == Some('=') {
                self.advance();
                Some(make_token(TokenType::Neq, "!=".to_string()))
            } else {
                self.next()
            }
        // } else if c == '=' {
        //     self.advance();
        //     if self.peek() == Some('=') {
        //         self.advance();
        //         Some(make_token(TokenType::DblEquals, "==".to_string()))
        //     } else {
        //         Some(make_token(TokenType::Equals, "=".to_string()))
        //     }
        // } else if c == ':' {
        //     self.advance();
        //     if self.peek() == Some(':') {
        //         self.advance();
        //         Some(make_token(TokenType::DblColon, "::".to_string()))
        //     } else {
        //         Some(make_token(TokenType::Colon, ":".to_string()))
        //     }
        } else {
            self.advance();
            self.next()
        }
    }

    fn current(&self) -> Option<char> {
        self.source.chars().nth(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn process_string(&mut self) -> Token {
        let opening = self.current().unwrap();
        self.advance();

        let start = self.pos;
        // self.advance();

        while let Some(c) = self.current() {
            if c == opening {
                break;
            }

            self.advance();
        }

        let strval = (start <= self.pos)
            .then(|| self.source[start..self.pos].to_string())
            .unwrap_or_default();

        self.current()
            .filter(|&c| c == opening)
            .map(|_| self.advance());

        make_token(TokenType::String, strval)
    }
    
    fn process_number(&mut self) -> Token {
        let start = self.pos;
        let mut float = false;  // flag for processing floating point numbers

        // while let Some(c) = self.current() {
        //     if c.is_numeric() {
        //         self.advance();
        //     } else {
        //         break;
        //     }
        // }

        // replaced previous processor with a more concise and simple one, this supports floating point numbers
        while let Some(c) = self.current() {
            match c {
                // is a number? advance if so
                f if f.is_numeric() => self.advance(),
                // if we encounter dot, and after it is a number, then process float
                '.' if !float && self.peek().map_or(false, |n| n.is_numeric()) => {
                    float = true;
                    self.advance();
                }
                _ => break,
            }
        }
        
        make_token(TokenType::Number, self.source[start..self.pos].to_string())
    }

    fn process_identifier(&mut self) -> Token {
        let start = self.pos;

        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let ident = self.source[start..self.pos].to_string();

        if self.keywords.contains_key(ident.as_str()) {
            let tok_type = self.keywords.get(ident.as_str()).unwrap().clone();
            make_token(tok_type, ident)
        } else {
            make_token(TokenType::Ident, self.source[start..self.pos].to_string())
        }
    }
}
