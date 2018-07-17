use kotoba::{eval::Environment, parser::Parser};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut env = Environment::new();

    loop {
        print!("::<> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let ast = Parser::new(&input).parse();

        println!("{}", env.eval(&ast));
    }
}
