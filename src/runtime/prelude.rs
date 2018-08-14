use super::*;

crate fn print(args: Vec<Type>) -> Type {
    for a in args {
        print!(
            "{}",
            match a {
                Type::String(s) => s,
                other => other.to_string(),
            }
        );
    }
    Type::Nil
}

crate fn println(args: Vec<Type>) -> Type {
    print(args);
    print!("\n");
    Type::Nil
}

crate fn add_two(args: Vec<Type>) -> Type {
    match args.as_slice() {
        [Type::Number(left), Type::Number(right)] => Type::Number(left + right),
        _ => panic!("not numbers"),
    }
}

crate fn div(args: Vec<Type>) -> Type {
    match args.as_slice() {
        [Type::Number(q), Type::Number(n)] => Type::Boolean(n % q == 0.0),
        _ => panic!(""),
    }
}
