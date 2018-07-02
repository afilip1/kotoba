#![feature(range_contains)]

#[derive(Debug)]
enum TokenKind {
    Number(f64),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    Nil,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    OpenParen,
    CloseParen,
}

struct Lexer<'a> {
    source: &'a [u8],
    index: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            source: source.as_bytes(),
            index: 0,
        }
    }

    fn tokenize(&mut self) -> Vec<TokenKind> {
        let mut tokens = vec![];

        while let Some(c) = self.source.get(self.index) {
            match c {
                b' ' | b'\t' | b'\n' => self.index += 1,
                b'0'...b'9' => tokens.push(TokenKind::Number(self.number())),
                b'a'...b'z' | b'A'...b'Z' | b'_' => tokens.push(match self.identifier() {
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    "nil" => TokenKind::Nil,
                    other => TokenKind::Identifier(other.to_owned()),
                }),
                b'"' => {
                    self.index += 1;
                    let string_contents = self.string().to_owned();
                    if let Some(b'"') = self.source.get(self.index) {
                        self.index += 1;
                        tokens.push(TokenKind::StringLiteral(string_contents));
                    } else {
                        println!("Unclosed string literal");
                        std::process::exit(1);
                    }
                }
                b'=' => if let Some(b'=') = self.source.get(self.index + 1) {
                    self.index += 2;
                    tokens.push(TokenKind::Equal);
                } else {
                    println!("Assignment is not supported at this time");
                    std::process::exit(2);
                },
                b'!' => match self.source.get(self.index + 1) {
                    Some(b'=') => {
                        self.index += 2;
                        tokens.push(TokenKind::NotEqual);
                    }
                    _ => {
                        self.index += 1;
                        tokens.push(TokenKind::Bang);
                    }
                },
                b'>' => match self.source.get(self.index + 1) {
                    Some(b'=') => {
                        self.index += 2;
                        tokens.push(TokenKind::GreaterOrEqual);
                    }
                    _ => {
                        self.index += 1;
                        tokens.push(TokenKind::Greater);
                    }
                },
                b'<' => match self.source.get(self.index + 1) {
                    Some(b'=') => {
                        self.index += 2;
                        tokens.push(TokenKind::LessOrEqual);
                    }
                    _ => {
                        self.index += 1;
                        tokens.push(TokenKind::Less);
                    }
                },
                b'+' => {
                    tokens.push(TokenKind::Plus);
                    self.index += 1;
                }
                b'-' => {
                    tokens.push(TokenKind::Minus);
                    self.index += 1;
                }
                b'*' => {
                    tokens.push(TokenKind::Star);
                    self.index += 1;
                }
                b'/' => {
                    tokens.push(TokenKind::Slash);
                    self.index += 1;
                }
                b'(' => {
                    tokens.push(TokenKind::OpenParen);
                    self.index += 1;
                }
                b')' => {
                    tokens.push(TokenKind::CloseParen);
                    self.index += 1;
                }
                other => {
                    println!("Unrecognized byte: '{}' (0x{:x})", *other as char, *other);
                    self.index += 1;
                }
            }
        }
        tokens
    }

    fn string(&mut self) -> &str {
        let start = self.index;
        while let Some(c) = self.source.get(self.index) {
            if *c == b'"' {
                break;
            }
            self.index += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn identifier(&mut self) -> &str {
        let start = self.index;
        while let Some(c) = self.source.get(self.index) {
            if !c.is_ascii_alphanumeric() && *c != b'_' {
                break;
            }
            self.index += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn number(&mut self) -> f64 {
        let start = self.index;
        while let Some(b'0'...b'9') = self.source.get(self.index) {
            self.index += 1;
        }
        if let Some(b'.') = self.source.get(self.index) {
            if let Some(b'0'...b'9') = self.source.get(self.index + 1) {
                self.index += 2;
                while let Some(b'0'...b'9') = self.source.get(self.index) {
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

fn main() {
    // let source = "123.434 true false nil \"test \nstring\" + - * / == != >= <= > < ! ()";
    let source = "(1 + 345.67) / some_var >= function(arg)";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize();

    println!("{:#?}", tokens);
}
