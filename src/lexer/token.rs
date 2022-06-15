#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Integer(usize),
    Real(f64),
    OpPlus,
    OpMinus,
    OpMultiply,
    OpDevide,
    ParenL,
    ParenR,
    Terminator,
}
