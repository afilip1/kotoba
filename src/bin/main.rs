use kotoba::{eval::*, parser::Parser};
use std::{
    env, fs,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        start_repl()?
    } else {
        interpret_file(&args[1])?
    }

    Ok(())
}

fn start_repl() -> io::Result<()> {
    let env = Env::new();

    loop {
        print!("::<> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let ast = Parser::new(&input).parse();
        let res = Env::eval(env.clone(), &ast);
        println!("{}", res);
    }
}

fn interpret_file(path: &str) -> io::Result<()> {
    let source = fs::read_to_string(path)?;

    let ast = Parser::new(&source).parse();
    println!("{:#?}", &ast);

    Ok(())
}
