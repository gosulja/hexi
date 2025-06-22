#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident,
    Number,
    LParen,
    RParen,
    Comma,
    Semi,
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
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer {
        Lexer { source, pos: 0 }
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
        } else if c == ';' {
            Some(make_token(TokenType::Semi, ";".to_string()))
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

        make_token(TokenType::Ident, self.source[start..self.pos].to_string())
    }
}
