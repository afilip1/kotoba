#![feature(range_contains)]

#[derive(Debug)]
enum TokenKind {
    Number(f64),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    Nil,
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

        while self.index < self.source.len() {
            match self.source[self.index] {
                b' ' | b'\t' | b'\n' => self.index += 1,
                b'0'...b'9' => {
                    tokens.push(TokenKind::Number(self.number()));
                }
                b'a'...b'z' | b'A'...b'Z' | b'_' => tokens.push(match self.identifier() {
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    "nil" => TokenKind::Nil,
                    other => TokenKind::Identifier(other.to_owned()),
                }),
                b'"' => {
                    self.index += 1;
                    let string_contents = self.string().to_owned();
                    if self.index < self.source.len() && self.source[self.index] == b'"' {
                        self.index += 1;
                        tokens.push(TokenKind::StringLiteral(string_contents));
                    } else {
                        println!("Unclosed string literal");
                        std::process::exit(1);
                    }
                }
                other => {
                    println!("Unrecognized byte: '{}' (0x{:x})", other as char, other);
                    self.index += 1;
                }
            }
        }
        tokens
    }

    fn string(&mut self) -> &str {
        let start = self.index;
        while self.index < self.source.len() && self.source[self.index] != b'"' {
            self.index += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn identifier(&mut self) -> &str {
        let start = self.index;
        while self.index < self.source.len()
            && (self.source[self.index].is_ascii_alphanumeric() || self.source[self.index] == b'_')
        {
            self.index += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn number(&mut self) -> f64 {
        let start = self.index;
        while self.index < self.source.len() && (b'0'..=b'9').contains(&self.source[self.index]) {
            self.index += 1;
        }
        if self.index < self.source.len() && self.source[self.index] == b'.' {
            if self.index + 1 < self.source.len()
                && (b'0'..=b'9').contains(&self.source[self.index + 1])
            {
                self.index += 2;
                while self.index < self.source.len()
                    && (b'0'..=b'9').contains(&self.source[self.index])
                {
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
    let source = "true false nil \"test \nstring\"";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.tokenize();

    println!("{:?}", tokens);
}
