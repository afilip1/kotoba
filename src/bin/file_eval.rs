use kotoba::{parser::Parser, runtime::*};
use std::{env, fs};

fn main() {
    let path = env::args()
        .nth(1)
        .expect("No file provided. Did you mean to run in REPL mode instead?");

    let source = fs::read_to_string(path).unwrap();

    let ast = Parser::new(&source).parse();
    Env::eval(Env::new(), &ast);
}
