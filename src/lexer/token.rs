use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Op(Op),
    ParenL,
    ParenR,
    Terminator,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Devide,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Number {
    Integer(isize),
    Real(f64),
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(a) => write!(f, "{:?}", a),
            Number::Real(a) => write!(f, "{:?}", a),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(a) => write!(f, "{:?}", a),
            Number::Real(a) => write!(f, "{:?}", a),
        }
    }
}
