use crate::parser::Token;

#[derive(Debug)]
pub enum Node {
    BinaryOp(Box<Node>, Token, Box<Node>),
    UnaryOp(Token, Box<Node>),
    Number(Token),
    Void
}