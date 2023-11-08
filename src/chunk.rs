
use crate::value::Value;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Constant,
    True,
    False,

    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,

    ValueEqual,
    NotValueEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Not,
    Negative,

    Noop,
    Pop,
    Return,

    Assert,
}

pub enum BytecodeOp {
    Op,
    ConstantIndex(u8),
    JumpDistance(u8),
}

pub struct Chunk {
    codes: Vec<BytecodeOp>,
    lines: Vec<u16>,
    constants: Vec<Value>
}