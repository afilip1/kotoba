use kotoba::{parser::Parser, eval::Environment};
use std::io::{stdout, stdin, Write};

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
