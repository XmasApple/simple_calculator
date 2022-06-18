use crate::{
    lexer::token::{Number, Op},
    parser::nodes::Node,
};

pub fn visit(node: &Node) -> Number {
    match node {
        Node::Number(val) => val.clone(),
        Node::BinaryOp(_, _, _) => visit_binary(node),
        Node::UnaryOp(_, _) => visit_unary(node),
    }
}

fn visit_binary(node: &Node) -> Number {
    if let Node::BinaryOp(left, op, right) = node {
        let left = visit(left.to_owned());
        let right = visit(right.to_owned());
        match (left, op, right) {
            (Number::Integer(a), op, Number::Integer(b)) => match op {
                Op::Plus => Number::Integer(a + b),
                Op::Minus => Number::Integer(a + b),
                Op::Multiply => Number::Integer(a * b),
                Op::Devide => Number::Real(a as f64 / b as f64),
            },
            (Number::Integer(a), op, Number::Real(b)) => match op {
                Op::Plus => Number::Real(a as f64 + b),
                Op::Minus => Number::Real(a as f64 + b),
                Op::Multiply => Number::Real(a as f64 * b),
                Op::Devide => Number::Real(a as f64 / b),
            },
            (Number::Real(a), op, Number::Integer(b)) => match op {
                Op::Plus => Number::Real(a + b as f64),
                Op::Minus => Number::Real(a + b as f64),
                Op::Multiply => Number::Real(a * b as f64),
                Op::Devide => Number::Real(a / b as f64),
            },
            (Number::Real(a), op, Number::Real(b)) => match op {
                Op::Plus => Number::Real(a + b),
                Op::Minus => Number::Real(a + b),
                Op::Multiply => Number::Real(a * b),
                Op::Devide => Number::Real(a / b),
            },
        }
    } else {
        panic!("is not binary")
    }
}

fn visit_unary(node: &Node) -> Number {
    if let Node::UnaryOp(op, val) = node {
        let val = visit(val.to_owned());
        match (op, val) {
            (Op::Plus, Number::Integer(val)) => Number::Integer(val.to_owned()),
            (Op::Minus, Number::Integer(val)) => Number::Integer(-val.to_owned()),
            (Op::Plus, Number::Real(val)) => Number::Real(val.to_owned()),
            (Op::Minus, Number::Real(val)) => Number::Real(-val.to_owned()),
            _ => panic!("not covered {op:?} {val:?}"),
        }
    } else {
        panic!("is not unary")
    }
}
