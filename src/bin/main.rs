use kotoba::parser::Parser;
use kotoba::eval::*;

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
    let env = Env::new();

    loop {
        print!("::<> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let ast = Parser::new(&input).parse();

        // println!("{:#?}\n", &ast);
        println!("{}", Env::eval(env.clone(), &ast));
        // println!("{:#?}", env);
    }
}

fn interpret_file(path: &str) {
    let mut source = String::new();
    File::open(path)
        .expect("couldn't open file")
        .read_to_string(&mut source)
        .expect("couldn't read file");

    let ast = Parser::new(&source).parse();
    println!("{:#?}", &ast);
}
