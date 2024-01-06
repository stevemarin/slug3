
use crate::{object::{ObjFucntion, Object}, chunk::{BytecodeOp, Op}, value::{Value, is_number}, token::Operator, compiler::Compiler};

use hashbrown::HashMap;

#[derive(Debug, Clone, Copy)]
enum InterpretResult {
    CompileError,
    RuntimeError,
    Ok
}

#[derive(Debug, Clone, Copy)]
struct CallFrame<'vm> {
    function: &'vm ObjFucntion<'vm>,
    instructions: &'vm Vec<BytecodeOp>,
    instruction_index: usize,
    slots: u8
}

impl<'vm> CallFrame<'vm> {
    #[inline]
    fn read_byte(self, frame_index: usize) -> (usize, BytecodeOp) {
        (frame_index + 1, self.instructions[frame_index])
    }

    #[inline]
    fn read_constant(self, frame_index: usize) -> (usize, Value<'vm>) {
        let constant_index: BytecodeOp;
        (_, constant_index) = self.read_byte(frame_index);
        let constant_index: usize = match constant_index {
            BytecodeOp::ConstantIndex(x) => x.into(),
            _ => panic!("expected constant index")
        };
        let constant = self.function.chunk.constants[constant_index];

        (frame_index + 2, constant)
    }
}

#[derive(Debug)]
pub struct VM<'vm> {
    stack: Vec<Value<'vm>>,
    compilers: Vec<Compiler<'vm>>,
    globals: HashMap<&'vm str, Value<'vm>>,
    strings: HashMap<&'vm str, &'vm str>,
    frames: Vec<CallFrame<'vm>>,
    frame_count: usize,
    objects: Vec<Object>,
    init_string: &'static str,
}

impl<'vm> VM<'vm> {
    pub fn new() -> VM<'vm> {
        VM {
            stack: vec![],
            compilers: vec![],
            globals: HashMap::new(),
            strings: HashMap::new(),
            frames: vec![],
            frame_count: 0,
            objects: vec![],
            init_string: "init"
        }
    }

    #[inline]
    fn peek(&'vm self, distance: usize) -> &'vm Value {
        self.stack.get(self.stack.len() - distance).unwrap()
    }

    #[inline]
    fn binary_op(&mut self, op: Operator) {
        if !is_number(self.peek(0)) || !is_number(self.peek(1)) {
            panic!("both operands must be numbers")
        }

        let b: Value = self.stack.pop().unwrap();
        let a: Value = self.stack.pop().unwrap();
        let c: Value = op.doit(a, b);
        self.stack.push(c);
        
    }

    fn call(&'vm mut self, function: &'vm ObjFucntion<'vm>, num_args: u8) -> bool {
        _ = if num_args != function.arity {
            panic!("wrong number of arguments, got {} expected {}", num_args, function.arity);
        };

        _ = if self.frames.len() == u8::MAX.into() {
            panic!("stack overflow")
        };

        let frame: CallFrame = CallFrame { function: &function, instructions: &function.chunk.codes, instruction_index: 0, slots: self.stack.len() as u8 - num_args - 1 };
        self.frames.insert(self.frame_count, frame);
        self.frame_count += 1;

        true
    }


    fn run(&'vm mut self) -> InterpretResult {
        let frame: CallFrame = self.frames.get(self.frame_count - 1).cloned().unwrap();
        let mut frame_index: usize = 0;
        let mut instruction: BytecodeOp;

        loop {
            (frame_index, instruction) = frame.read_byte(frame_index);
            match instruction {
                BytecodeOp::Op(Op::Constant) => {
                    let constant: Value;
                    (frame_index, constant) = frame.read_constant(frame_index);
                    self.stack.push(constant);
                }
                BytecodeOp::Op(Op::True) => self.stack.push(Value::Bool(true)),
                BytecodeOp::Op(Op::False) => self.stack.push(Value::Bool(false)),
                BytecodeOp::Op(Op::Pop) => _ = self.stack.pop(),
                BytecodeOp::Op(Op::ValueEqual) => self.binary_op(Operator::EqualEqual),
                BytecodeOp::Op(Op::NotValueEqual) => self.binary_op(Operator::NotEqual),
                BytecodeOp::Op(Op::Less) => self.binary_op(Operator::Less),
                BytecodeOp::Op(Op::LessEqual) => self.binary_op(Operator::LessEqual),
                BytecodeOp::Op(Op::Greater) => self.binary_op(Operator::Greater),
                BytecodeOp::Op(Op::GreaterEqual) => self.binary_op(Operator::GreaterEqual),
                BytecodeOp::Op(Op::Add) => self.binary_op(Operator::Plus),
                BytecodeOp::Op(Op::Subtract) => self.binary_op(Operator::Minus),
                BytecodeOp::Op(Op::Multiply) => self.binary_op(Operator::Star),
                BytecodeOp::Op(Op::Exponent) => self.binary_op(Operator::StarStar),
                BytecodeOp::Op(Op::Divide) => self.binary_op(Operator::Slash),
                BytecodeOp::Op(Op::IntDivide) => self.binary_op(Operator::SlashSlash),
                BytecodeOp::Op(Op::Return) => {
                    self.frame_count -= 1;

                    if self.frame_count == 0 {
                        self.stack.pop(); // result
                        self.stack.pop();
                        return InterpretResult::Ok;
                    }

                    _ = self.frames.pop();

                }
                _ => panic!("got jump or constant index")
            }
                
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::{Tokenizer, parser::{Parser, Precedence}};

    #[test]
    fn test_arithmetic() {
    //     let tokens = Tokenizer::new(r"1 + 2").tokenize();
    //     let mut parser = &Parser::<>::new(tokens);
    //     parser.parse_precedence(Precedence::Assignment)

    }
}