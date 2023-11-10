
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
    IntDivide,
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

#[derive(Debug, Clone, Copy)]
pub enum BytecodeOp {
    Op(Op),
    ConstantIndex(u8),
    JumpDistance(u8),
}

#[derive(Debug)]
pub struct Chunk<'vm> {
    pub codes: Vec<BytecodeOp>,
    pub lines: Vec<u16>,
    pub constants: Vec<Value<'vm>>
}