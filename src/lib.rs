macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    });
}

pub mod lexer;
pub mod parser;
pub mod runtime;
mod source_stream;
