#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Integer(isize),
    Real(f64),
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
