use crate::lexer::token::Token;
pub mod nodes;
use nodes::Node;

pub struct Parser {
    pub tokens: Vec<Token>,
    ptr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, ptr: 0 }
    }

    pub fn parse(mut self) -> Node {
        self.expr()
    }

    fn atom(&mut self) -> Node {
        let current = self.current();
        self.advance();
        match current {
            Token::Integer(_) | Token::Real(_) => {
                self.advance();
                Node::Number(current)
            }
            Token::ParenL => {
                self.advance();
                let expr = self.expr();
                assert!(self.current() == Token::ParenR, "Syntax Error");
                expr
            }
            Token::Terminator => panic!("Syntax Error"),
            _ => self.expr(),
        }
    }

    fn factor(&mut self) -> Node {
        let current = self.current();
        match current {
            Token::OpPlus | Token::OpMinus => {
                self.advance();
                Node::UnaryOp(current, Box::new(self.atom()))
            }
            _ => self.atom(),
        }
    }

    fn mul(&mut self) -> Node {
        let mut res = self.factor();
        while self.current() == Token::OpMultiply || self.current() == Token::OpDevide {
            let tmp = self.current();
            self.advance();
            res = Node::BinaryOp(Box::new(res), tmp, Box::new(self.factor()))
        }
        res
    }

    fn sum(&mut self) -> Node {
        let mut res = self.mul();
        while self.current() == Token::OpPlus || self.current() == Token::OpMinus {
            let tmp = self.current();
            self.advance();
            res = Node::BinaryOp(Box::new(res), tmp, Box::new(self.mul()))
        }
        res
    }

    fn expr(&mut self) -> Node {
        self.sum()
    }

    fn advance(&mut self) {
        self.ptr += 1;
    }

    fn current(&self) -> Token {
        if self.ptr < self.tokens.len() {
            self.tokens[self.ptr]
        } else {
            Token::Terminator
        }
    }
}
