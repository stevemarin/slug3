use crate::{token::Token, parser::Parser, object::ObjFucntion};


#[derive(Debug)]
struct Local {
    name: Token,
    depth: usize,
    captured: bool,
}

#[derive(Debug)]
enum FuncType {
    Function,
    Initializer,
    Method,
    Script,
}

#[derive(Debug)]
pub struct Compiler<'vm> {
    parser: Parser<'vm>,
    functype: FuncType,
    function: ObjFucntion<'vm>,
    scope_depth: usize,
    locals: Vec<Local>
}

impl<'a> Compiler<'a> {
    fn new{}
}