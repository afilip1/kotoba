use kotoba::{parser::Parser, runtime::*};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let env = Env::new();

    loop {
        print!("::<> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let ast = Parser::new(&input).parse();
        let res = Env::eval(env.clone(), &ast);
        println!("=> {}", res);
    }
}
