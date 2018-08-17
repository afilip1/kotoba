macro_rules! prelude {
    ($($name:ident($args:ident) $body:block)*) => {
        use std::collections::HashMap;
        use super::{Type, Callable};

        pub(super) fn init() -> HashMap<String, Callable> {
            hashmap!{
                $(stringify!($name).to_owned() => Callable::Builtin($name)),*
            }
        }

        $(pub(super) fn $name($args: Vec<Type>) -> Type $body)*
    };
}

prelude!{
    print(args) {
        for a in args {
            let out = match a {
                Type::String(s) => s,
                _ => a.to_string(),
            };
            print!("{}", out);
        }

        Type::Nil
    }

    println(args) {
        print(args);
        println!();

        Type::Nil
    }

    add_two(args) {
        match args.as_slice() {
            [Type::Number(left), Type::Number(right)] => Type::Number(left + right),
            _ => panic!(),
        }
    }

    div(args) {
        match args.as_slice() {
            [Type::Number(q), Type::Number(n)] => Type::Boolean(n % q == 0.0),
            _ => panic!(),
        }
    }
}
