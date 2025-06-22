use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident,
    Number,
    String,
    LParen,
    RParen,
    Comma,
    Equals,
    Semi,
    Val,        // variable declaration
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

        Lexer { source, pos: 0, keywords }
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
        } else if c == ',' {
            self.advance();
            Some(make_token(TokenType::Comma, ",".to_string()))
        } else if c == '=' {
            self.advance();
            Some(make_token(TokenType::Equals, "=".to_string()))
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
        
        while let Some(c) = self.current() {
            if c.is_numeric() {
                self.advance();
            } else {
                break;
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
