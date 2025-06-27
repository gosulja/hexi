use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident,
    Number,
    String,
    LParen,
    RParen,
    Comma,
    DblEquals, // ==
    Lt,        // <
    Gt,        // >
    Gte,       // >=
    Lte,       // <=
    Neq,       // !=
    Equals,
    Semi,
    Val,      // variable declaration
    DblColon, // ::
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Dot,
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

        Lexer {
            source,
            pos: 0,
            keywords,
        }
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    // single token, =
    fn stoken(&mut self, token_type: TokenType) -> Option<Token> {
        let c = self.current()?.to_string();
        self.advance();
        Some(make_token(token_type, c))
    }
    
    // double tokens, so like ==
    fn dtoken(&mut self, second_char: char, double_type: TokenType, single_type: TokenType) -> Option<Token> {
        let first_char = self.current()?;
        self.advance();
        
        if self.current() == Some(second_char) {
            self.advance();
            Some(make_token(double_type, format!("{}{}", first_char, second_char)))
        } else {
            Some(make_token(single_type, first_char.to_string()))
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.pos + 1)
    }

    pub fn next(self: &mut Lexer<'a>) -> Option<Token> {
        self.skip_ws();

        let c = self.current()?;

        match c {
            c if c.is_alphabetic() || c == '_' => Some(self.process_identifier()),
            c if c.is_numeric() => Some(self.process_number()),
            '"' | '\'' => Some(self.process_string()),

            ';' => self.stoken(TokenType::Semi),
            '(' => self.stoken(TokenType::LParen),
            ')' => self.stoken(TokenType::RParen),
            '{' => self.stoken(TokenType::LBrace),
            '}' => self.stoken(TokenType::RBrace),
            '[' => self.stoken(TokenType::LBracket),
            ']' => self.stoken(TokenType::RBracket),
            '.' => self.stoken(TokenType::Dot),
            ',' => self.stoken(TokenType::Comma),
            '+' => self.stoken(TokenType::Add),
            '-' => self.stoken(TokenType::Sub),
            '*' => self.stoken(TokenType::Mul),
            '/' => self.stoken(TokenType::Div),
            '%' => self.stoken(TokenType::Mod),

            ':' => self.dtoken(':', TokenType::DblColon, TokenType::Colon),
            '=' => self.dtoken('=', TokenType::DblEquals, TokenType::Equals),
            '<' => self.dtoken('=', TokenType::Lte, TokenType::Lt),
            '>' => self.dtoken('=', TokenType::Gte, TokenType::Gt),
            // for double tokens which have two different chars in them, but there is no character ! by it
            // self, so make sure to skip the illegal character if it's by itself
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    Some(make_token(TokenType::Neq, "!=".to_string()))
                } else {
                    self.advance();
                    self.next()
                }
            }

            _ => {
                self.advance();
                self.next()
            }
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
        let mut float = false; // flag for processing floating point numbers

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
