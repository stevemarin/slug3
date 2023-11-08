
mod token;
mod parser;
mod chunk;
mod value;
mod object;
mod vm;

use crate::token::Tokenizer;

fn main() {
    let tokens = Tokenizer::new(r"=").tokenize();
    for t in tokens.into_iter() {
        println!("\t{:?}", t.tokentype);
    }
}
