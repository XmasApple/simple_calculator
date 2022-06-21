use std::{isize, panic};

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
    println!("{node:?}");
    if let Node::BinaryOp(left, op, right) = node {
        let left = visit(left.to_owned());
        let right = visit(right.to_owned());
        match op {
            Op::Plus => match left {
                Number::Integer(a) => match right {
                    Number::Integer(b) => match a.checked_add(b) {
                        Some(r) => Number::Integer(r),
                        None => Number::InfPlus,
                    },
                    Number::Real(b) => Number::Real(a as f64 + b),
                    _ => right,
                },
                Number::Real(a) => match right {
                    Number::Integer(b) => Number::Real(a + b as f64),
                    Number::Real(b) => Number::Real(a + b),
                    _ => right,
                },
                _ => left,
            },
            Op::Minus => match left {
                Number::Integer(a) => match right {
                    Number::Integer(b) => match a.checked_sub(b) {
                        Some(r) => Number::Integer(r),
                        None => Number::InfMinus,
                    },
                    Number::Real(b) => Number::Real(a as f64 - b),
                    Number::InfPlus => Number::InfMinus,
                    Number::InfMinus => Number::InfPlus,
                    _ => right,
                },
                Number::Real(a) => match right {
                    Number::Integer(b) => Number::Real(a - b as f64),
                    Number::Real(b) => Number::Real(a - b),
                    Number::InfPlus => Number::InfMinus,
                    Number::InfMinus => Number::InfPlus,
                    _ => right,
                },
                _ => left,
            },
            Op::Multiply => match left {
                Number::Integer(a) => match right {
                    Number::Integer(b) => match a.checked_mul(b) {
                        Some(r) => Number::Integer(r),
                        None => {
                            if a > 0 && b > 0 || a < 0 && b < 0 {
                                Number::InfPlus
                            } else {
                                Number::InfMinus
                            }
                        }
                    },
                    Number::Real(b) => Number::Real(a as f64 * b),
                    Number::InfMinus => match a {
                        a if a < 0 => Number::InfPlus,
                        a if a > 0 => Number::InfMinus,
                        _ => Number::Integer(0),
                    },
                    _ => right,
                },
                Number::Real(a) => match right {
                    Number::Integer(b) => Number::Real(a * b as f64),
                    Number::Real(b) => Number::Real(a * b),
                    Number::InfMinus => match a {
                        a if a < 0.0 => Number::InfPlus,
                        a if a > 0.0 => Number::InfMinus,
                        _ => Number::Integer(0),
                    },
                    _ => right,
                },
                Number::InfMinus if matches!(right, Number::InfMinus) => Number::InfPlus,
                _ => left,
            },
            Op::Devide => match left {
                Number::Integer(a) => match right {
                    Number::Integer(0) => Number::Nan,
                    Number::Real(b) if b == 0.0 => Number::Nan,
                    Number::Integer(b) if a % b == 0 => Number::Integer(a / b),
                    Number::Integer(b) => Number::Real(a as f64 / b as f64),
                    Number::Real(b) => Number::Real(a as f64 / b),
                    Number::InfMinus | Number::InfPlus => Number::Nan,
                    _ => right,
                },
                Number::Real(a) => match right {
                    Number::Integer(0) => Number::Nan,
                    Number::Real(b) if b == 0.0 => Number::Nan,
                    Number::Integer(b) => Number::Real(a / b as f64),
                    Number::Real(b) => Number::Real(a / b),
                    _ => right,
                },
                _ => left,
            },
            Op::Pow => match left {
                Number::Integer(a) => match right {
                    Number::Integer(b) => {
                        let res = f64::powf(a as f64, b as f64) as isize;
                        match res {
                            isize::MAX => Number::InfPlus,
                            isize::MIN => Number::InfMinus,
                            _ => Number::Integer(res),
                        }
                    }
                    Number::Real(b) => Number::Real((a as f64).powf(b)),
                    Number::InfMinus | Number::InfPlus => Number::Nan,
                    _ => right,
                },
                Number::Real(a) => match right {
                    Number::Integer(b) => Number::Real(a.powf(b as f64)),
                    Number::Real(b) => Number::Real(a.powf(b)),
                    Number::InfMinus | Number::InfPlus => Number::Nan,
                    _ => right,
                },
                Number::InfMinus if matches!(right, Number::InfMinus) => Number::InfPlus,
                _ => left,
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
