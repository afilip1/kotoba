use std;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp = std::collections::HashMap::new();
        $(tmp.insert($key, $value);)*
        tmp
    });
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Number(f64),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    Nil,
    Equal,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    OpenParen,
    CloseParen,
}

pub struct Lexer<'a> {
    source: &'a [u8],
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.as_bytes(),
            index: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<TokenKind> {
        let mut tokens = vec![];

        let lookahead_map = hashmap! {
            b'=' => (TokenKind::EqualEqual, TokenKind::Equal),
            b'!' => (TokenKind::BangEqual, TokenKind::Bang),
            b'>' => (TokenKind::GreaterEqual, TokenKind::Greater),
            b'<' => (TokenKind::LessEqual, TokenKind::Less)
        };

        while let Some(c) = self.current() {
            match c {
                b'0'...b'9' => tokens.push(TokenKind::Number(self.consume_number())),
                b'a'...b'z' | b'A'...b'Z' | b'_' => tokens.push(match self.consume_identifier() {
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    "nil" => TokenKind::Nil,
                    other => TokenKind::Identifier(other.to_owned()),
                }),
                b'"' => {
                    self.index += 1;
                    let string_contents = self.consume_string();
                    if let Some(b'"') = self.current() {
                        tokens.push(TokenKind::StringLiteral(string_contents.to_owned()));
                        self.index += 1;
                    } else {
                        println!("Unclosed string literal");
                        std::process::exit(1);
                    }
                }
                c @ b'=' | c @ b'!' | c @ b'>' | c @ b'<' => match self.peek() {
                    Some(b'=') => {
                        tokens.push(lookahead_map[c].0.clone());
                        self.index += 2;
                    }
                    Some(_) => {
                        tokens.push(lookahead_map[c].1.clone());
                        self.index += 1;
                    }
                    _ => self.index += 1,
                },
                size_1 => {
                    match size_1 {
                        b' ' | b'\t' | b'\n' => { /* skip whitespace */ }
                        b'+' => tokens.push(TokenKind::Plus),
                        b'-' => tokens.push(TokenKind::Minus),
                        b'*' => tokens.push(TokenKind::Star),
                        b'/' => tokens.push(TokenKind::Slash),
                        b'(' => tokens.push(TokenKind::OpenParen),
                        b')' => tokens.push(TokenKind::CloseParen),
                        other => println!(
                            "Unrecognized byte: '{}' (0x{:x}), skipping...",
                            *other as char, *other
                        ),
                    }
                    self.index += 1;
                }
            }
        }
        tokens
    }

    fn current(&self) -> Option<&u8> {
        self.source.get(self.index)
    }

    fn peek(&self) -> Option<&u8> {
        self.source.get(self.index + 1)
    }

    fn consume_string(&mut self) -> &'a str {
        let start = self.index;
        while let Some(c) = self.current() {
            if *c == b'"' {
                break;
            }
            self.index += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn consume_identifier(&mut self) -> &str {
        let start = self.index;
        while let Some(c) = self.current() {
            if !c.is_ascii_alphanumeric() && *c != b'_' {
                break;
            }
            self.index += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn consume_number(&mut self) -> f64 {
        let start = self.index;
        while let Some(b'0'...b'9') = self.current() {
            self.index += 1;
        }
        if let Some(b'.') = self.current() {
            if let Some(b'0'...b'9') = self.peek() {
                self.index += 2;
                while let Some(b'0'...b'9') = self.current() {
                    self.index += 1;
                }
            }
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end])
            .unwrap()
            .parse::<f64>()
            .unwrap()
    }
}
