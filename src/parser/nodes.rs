use crate::lexer::token::{Number, Op};

#[derive(Debug)]
pub enum Node {
    BinaryOp(Box<Node>, Op, Box<Node>),
    UnaryOp(Op, Box<Node>),
    Number(Number),
}
