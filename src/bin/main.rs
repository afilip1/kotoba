use kotoba::{eval::Environment, parser::Parser, lexer::Lexer};
use std::{
    fs::File,
    io::{stdin, stdout, Read, Write},
};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        start_repl()
    } else {
        interpret_file(&args[1])
    }
}

fn start_repl() -> ! {
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

fn interpret_file(path: &str) {
    let mut source = String::new();
    File::open(path)
        .expect("couldn't open file")
        .read_to_string(&mut source)
        .expect("couldn't read file");

    println!("{:#?}", Lexer::new(&source).map(|t| t.kind).collect::<Vec<_>>());
}
