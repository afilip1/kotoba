#![feature(rust_2018_preview)]

mod lexer;
mod parser;
mod source_stream;
use crate::lexer::*;

fn main() {
    // let source = "123.434 true false nil \"test \nstring\" + - * / == != >= <= > < ! ()";
    let source = "(1 + 345.67) / some_var >= function(\"some string\")";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize();

    println!("{:#?}", tokens);
}
