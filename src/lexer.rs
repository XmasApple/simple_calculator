mod token;
use std::str::Chars;

use token::Token;

pub struct Lexer {
    pub chars: Vec<char>,
    ptr: usize,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            chars: text.chars().collect(),
            ptr: 0,
        }
    }

    pub fn parse(mut self: Self) -> Vec<Token> {
        let mut result = Vec::new();

        while self.ptr < self.chars.len() {
            let current = self.current();
            if current.is_whitespace() {
                self.advance()
            } else if current.is_digit(10) {
                result.push(self.parse_digit());
            } else {
                result.push(match current {
                    '+' => Token::OpPlus,
                    '-' => Token::OpMinus,
                    '*' => Token::OpMultiply,
                    '/' => Token::OpDevide,
                    '(' => Token::ParenL,
                    ')' => Token::ParenR,
                    _ => {
                        self.advance();
                        continue;
                    }
                });
                self.advance();
            }
        }

        result
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn current(&self) -> char {
        if self.ptr < self.chars.len() {
            self.chars[self.ptr]
        } else {
            '\0'
        }
    }

    fn parse_digit(&mut self) -> Token {
        let start = self.ptr.clone();
        while self.current().is_digit(10) {
            self.advance();
        }
        if self.current() == '.' {
            while self.current().is_digit(10) {
                self.advance();
            }
            return Token::Real(
                String::from_iter(self.chars[start..self.ptr].iter())
                    .parse()
                    .unwrap(),
            );
        }
        Token::Integer(
            String::from_iter(self.chars[start..self.ptr].iter())
                .parse()
                .unwrap(),
        )
    }
}
