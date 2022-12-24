
use crate::token::*;
use crate::error::*;

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
        self.tokens.push(Token::new(TokenType::EOF, String::new(), None, self.line));
        if !had_error {return Ok(&self.tokens)}
        else {return Err(InterpreterError::new("Error(s) encountered while scanning."))}
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), InterpreterError> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                if self.matches('=') {self.add_token(TokenType::BangEqual, None);}
                else {self.add_token(TokenType::Bang, None);}
            },
            '=' => {
                if self.matches('=') {self.add_token(TokenType::EqualEqual, None);}
                else {self.add_token(TokenType::Equal, None);}
            },
            '<' => {
                if self.matches('=') {self.add_token(TokenType::LessEqual, None);}
                else {self.add_token(TokenType::Less, None);}
            },

            '>' => {
                if self.matches('=') {self.add_token(TokenType::GreaterEqual, None);}
                else {self.add_token(TokenType::Greater, None);}
            },
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, None);
                }
            },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line+=1,
            _ => return Err(InterpreterError::new_local(self.line,"",&format!("Unexpected character: {:?}", c)[..]))
        };
        Ok(())
    }
    
    fn advance(&mut self) -> char{
        let next = self.source.chars().nth(self.current).unwrap();
        self.current+=1;
        return next
    }
    
    fn add_token(&mut self, token_type: TokenType, literal: Option<()>) {
        let text: String = self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).expect("Index out of bounds") != expected {return false;}
        self.current+=1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {return '\0'}
        self.source.chars().nth(self.current).unwrap()
    }
}
