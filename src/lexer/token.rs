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
    Pow,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Number {
    Integer(isize),
    Real(f64),
    Nan,
    InfPlus,
    InfMinus,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(a) => write!(f, "{:?}", a),
            Number::Real(a) => write!(f, "{:?}", a),
            Number::InfPlus => write!(f, "InfPlus"),
            Number::Nan => write!(f, "Nan"),
            Number::InfMinus => write!(f, "InfMinus"),
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(a) => write!(f, "{:?}", a),
            Number::Real(a) => write!(f, "{:?}", a),
            Number::InfPlus => write!(f, "Inf"),
            Number::Nan => write!(f, "NaN"),
            Number::InfMinus => write!(f, "-Inf"),
        }
    }
}
