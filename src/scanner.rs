
use crate::token::*;
use crate::error::*;

use lazy_static::lazy_static;

use std::collections::HashMap;


lazy_static! {
pub static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
    ("and", TokenType::And),
    ("class", TokenType::Class),
    ("else", TokenType::Else),
    ("false", TokenType::False),
    ("for", TokenType::For),
    ("fun", TokenType::Fun),
    ("if", TokenType::If),
    ("nil", TokenType::Nil),
    ("or", TokenType::Or),
    ("print", TokenType::Print),
    ("return", TokenType::Return),
    ("super", TokenType::Super),
    ("this", TokenType::This),
    ("true", TokenType::True),
    ("var", TokenType::Var),
    ("while", TokenType::While)
]);
}



pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_owned(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>,InterpreterError> {
        let mut had_error = false;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {},
                Err(e) => {e.report(); had_error = true}
            };
        }
        self.tokens.push(Token::new(TokenType::EOF, String::new(), Literal::None, self.line));
        if !had_error {return Ok(&self.tokens)}
        else {return Err(InterpreterError::new("Error(s) encountered while scanning."))}
    }

    fn is_at_end(&self) -> bool {
        //.len() might not work for non-ascii stuff; look into fixing later
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), InterpreterError> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => {
                //the book Lox did not include support for numbers of the form '.03' but this will
                if self.peek().is_ascii_digit() { self.number(); }
                else {self.add_token(TokenType::Dot, Literal::None); }
            },
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Plus, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),
            '!' => {
                if self.matches('=') {self.add_token(TokenType::BangEqual, Literal::None);}
                else {self.add_token(TokenType::Bang, Literal::None);}
            },
            '=' => {
                if self.matches('=') {self.add_token(TokenType::EqualEqual, Literal::None);}
                else {self.add_token(TokenType::Equal, Literal::None);}
            },
            '<' => {
                if self.matches('=') {self.add_token(TokenType::LessEqual, Literal::None);}
                else {self.add_token(TokenType::Less, Literal::None);}
            },

            '>' => {
                if self.matches('=') {self.add_token(TokenType::GreaterEqual, Literal::None);}
                else {self.add_token(TokenType::Greater, Literal::None);}
            },
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, Literal::None);
                }
            },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line+=1,
            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number(),
            c if c.is_ascii_alphabetic() || c=='_' => self.identifier(),
            _ => return Err(InterpreterError::new_local(self.line,"",&format!("Unexpected character: {:?}", c)[..]))
        };
        Ok(())
    }
    
    fn advance(&mut self) -> char{
        let next = self.source.chars().nth(self.current).unwrap();
        self.current+=1;
        return next
    }
    
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).expect("Index out of bounds") != expected {return false;}
        self.current+=1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {return '\0'}
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {return '\0'}
        self.source.chars().nth(self.current + 1).unwrap()
    }


    fn string(&mut self) -> Result<(), InterpreterError> {
        let mut peek = self.peek();
        while peek != '"' && !self.is_at_end() {
            if peek == '\n' {self.line+=1;}
            self.advance();
            peek = self.peek();
        }
        
        if self.is_at_end() {
            return Err(InterpreterError::new_local(self.line, "", "Unterminated string."));
        }

        self.advance();

        let value = &self.source[self.start+1..self.current-1];
        self.add_token(TokenType::Str, Literal::Str(value.to_owned()));
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {self.advance();}

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();
            while self.peek().is_ascii_digit() {self.advance();}
        }
        let string = &self.source[self.start..self.current];
        self.add_token(TokenType::Number, Literal::Number(string.parse::<f64>().unwrap()));
    }

    fn identifier(&mut self) {
        let alphanumeric = |c: char| -> bool {c.is_ascii_alphanumeric() || c=='_'};
        while alphanumeric(self.peek()) { self.advance(); }

        let text = &self.source[self.start..self.current];
        let mut token_type: Option<TokenType> = KEYWORDS.get(text).copied();
        if token_type.is_none() { token_type = Some(TokenType::Identifier); }
        self.add_token(token_type.unwrap(), Literal::None);
    }

}
