#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Integer(usize),
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
