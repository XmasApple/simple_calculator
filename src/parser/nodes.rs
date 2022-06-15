use crate::{parser::Token, parser::Op};

#[derive(Debug)]
pub enum Node {
    BinaryOp(Box<Node>, Op, Box<Node>),
    UnaryOp(Op, Box<Node>),
    Number(Token),
    Void
}