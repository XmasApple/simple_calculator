use crate::{
    lexer::token::{Op, Token},
    parser::nodes::Node,
};

pub fn visit(node: &Node) -> Token {
    match node {
        Node::Number(val) => val.clone(),
        Node::BinaryOp(_, _, _) => visit_binary(node),
        Node::UnaryOp(_, _) => visit_unary(node),
    }
}

fn visit_binary(node: &Node) -> Token {
    if let Node::BinaryOp(left, op, right) = node {
        let left = visit(left.to_owned());
        let right = visit(right.to_owned());
        match (left, op, right) {
            (Token::Integer(a), op, Token::Integer(b)) => match op {
                Op::Plus => Token::Integer(a + b),
                Op::Minus => Token::Integer(a + b),
                Op::Multiply => Token::Integer(a * b),
                Op::Devide => Token::Real(a as f64 / b as f64),
            },
            (Token::Integer(a), op, Token::Real(b)) => match op {
                Op::Plus => Token::Real(a as f64 + b),
                Op::Minus => Token::Real(a as f64 + b),
                Op::Multiply => Token::Real(a as f64 * b),
                Op::Devide => Token::Real(a as f64 / b),
            },
            (Token::Real(a), op, Token::Integer(b)) => match op {
                Op::Plus => Token::Real(a + b as f64),
                Op::Minus => Token::Real(a + b as f64),
                Op::Multiply => Token::Real(a * b as f64),
                Op::Devide => Token::Real(a / b as f64),
            },
            (Token::Real(a), op, Token::Real(b)) => match op {
                Op::Plus => Token::Real(a + b),
                Op::Minus => Token::Real(a + b),
                Op::Multiply => Token::Real(a * b),
                Op::Devide => Token::Real(a / b),
            },
            _ => panic!("not covered {left:?} {op:?} {right:?}"),
        }
    } else {
        panic!("is not binary")
    }
}

fn visit_unary(node: &Node) -> Token {
    if let Node::UnaryOp(op, val) = node {
        let val = visit(val.to_owned());
        match (op, val) {
            (Op::Plus, Token::Integer(val)) => Token::Integer(val.to_owned()),
            (Op::Minus, Token::Integer(val)) => Token::Integer(-val.to_owned()),
            (Op::Plus, Token::Real(val)) => Token::Real(val.to_owned()),
            (Op::Minus, Token::Real(val)) => Token::Real(-val.to_owned()),
            _ => panic!("not covered {op:?} {val:?}"),
        }
    } else {
        panic!("is not unary")
    }
}
