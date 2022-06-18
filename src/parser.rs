use crate::lexer::token::*;
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
            Token::ParenL => {
                let expr = self.expr();
                assert!(self.current() == Token::ParenR, "Syntax Error");
                self.advance();
                expr
            }
            Token::Number(number) => Node::Number(number),
            Token::Terminator => panic!("Syntax Error"),
            _ => self.expr(),
        }
    }

    fn factor(&mut self) -> Node {
        let current = self.current();
        if let Token::Op(op @ Op::Plus | op @ Op::Minus) = current {
            self.advance();
            return Node::UnaryOp(op, Box::new(self.factor()));
        }
        self.atom()
    }

    fn mul(&mut self) -> Node {
        let mut res = self.factor();
        while let Token::Op(op @ Op::Multiply | op @ Op::Devide) = self.current() {
            self.advance();
            res = Node::BinaryOp(Box::new(res), op, Box::new(self.factor()));
        }
        res
    }

    fn sum(&mut self) -> Node {
        let mut res = self.mul();
        while let Token::Op(op @ Op::Plus | op @ Op::Minus) = self.current() {
            self.advance();
            res = Node::BinaryOp(Box::new(res), op, Box::new(self.mul()));
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
