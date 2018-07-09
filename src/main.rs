#![feature(rust_2018_preview, box_patterns)]

mod eval;
mod lexer;
mod parser;
mod source_stream;

use crate::eval::*;
use crate::parser::*;
use std::io::prelude::*;

fn main() {
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let ast = Parser::new(&input).parse();

        println!("Parsed AST:\n{:#?}\n", ast);
        println!("Expression evaluates to:\n{:?}", eval(&ast));
    }
}
