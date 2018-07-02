mod lexer;
use lexer::*;

fn main() {
    // let source = "123.434 true false nil \"test \nstring\" + - * / == != >= <= > < ! ()";
    let source = "(1 + 345.67) / some_var >= function(arg)";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize();

    println!("{:#?}", tokens);
}
