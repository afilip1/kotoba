#![feature(rust_2018_preview)]

mod lexer;
mod parser;
mod source_stream;
use crate::lexer::*;

fn main() {
    // let source = "123.434 true false nil \"test \nstring\" + - * / == != >= <= > < ! ()";
    let source = r#"(1 + 345.67) / some_var >= function("some string")"#;
    let tokens = Lexer::new(source).collect::<Vec<_>>();

    println!("{:#?}", tokens);
}
