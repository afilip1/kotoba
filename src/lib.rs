#![feature(rust_2018_preview, map_get_key_value)]

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp = std::collections::HashMap::new();
        $(tmp.insert($key, $value);)*
        tmp
    });
}

pub mod lexer;
pub mod parser;
pub mod runtime;
mod source_stream;
