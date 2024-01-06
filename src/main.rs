
mod token;
mod parser;
mod chunk;
mod value;
mod object;
mod compiler;
mod vm;

use vm::VM;
use parser::Parser;
use token::Tokenizer;
use compiler::Compiler;

fn main() {

    let vm: VM = VM::new();
    let compiler: Compiler::new()

    // let tokens = Tokenizer::new(r"1 + 3 == 4").tokenize();
    // for t in tokens.iter() {
    //     println!("\t{:?}", t.tokentype);
    // }
    // let parser = &mut Parser::new(tokens);
    // parser.declaration()
}
