macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp = std::collections::HashMap::new();
        $(tmp.insert($key, $value);)*
        tmp
    });
}

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub position: Position,
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind<'a> {
    Number(f64),
    Boolean(bool),
    Identifier(&'a str),
    StringLiteral(&'a str),
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
    cur_line: usize,
    cur_char: usize,
}

impl Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.as_bytes(),
            index: 0,
            cur_line: 0,
            cur_char: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        let lookahead_map = hashmap! {
            b'=' => (TokenKind::EqualEqual, TokenKind::Equal),
            b'!' => (TokenKind::BangEqual, TokenKind::Bang),
            b'>' => (TokenKind::GreaterEqual, TokenKind::Greater),
            b'<' => (TokenKind::LessEqual, TokenKind::Less)
        };

        while let Some(c) = self.current() {
            match c {
                b'0'...b'9' => tokens.push(Token {
                    position: Position {
                        line: self.cur_line,
                        character: self.cur_char,
                    },
                    kind: TokenKind::Number(self.consume_number()),
                }),
                b'a'...b'z' | b'A'...b'Z' | b'_' => tokens.push(Token {
                    position: Position {
                        line: self.cur_line,
                        character: self.cur_char,
                    },
                    kind: match self.consume_identifier() {
                        "true" => TokenKind::Boolean(true),
                        "false" => TokenKind::Boolean(false),
                        "nil" => TokenKind::Nil,
                        other => TokenKind::Identifier(other),
                    },
                }),
                b'"' => {
                    let line = self.cur_line;
                    let ch = self.cur_char;
                    self.index += 1;
                    self.cur_char += 1;
                    let string_contents = self.consume_string();
                    if let Some(b'"') = self.current() {
                        tokens.push(Token {
                            kind: TokenKind::StringLiteral(string_contents),
                            position: Position {
                                line: line,
                                character: ch,
                            },
                        });
                        self.index += 1;
                        self.cur_char += 1;
                    } else {
                        println!("Unclosed string literal");
                        std::process::exit(1);
                    }
                }
                c @ b'=' | c @ b'!' | c @ b'>' | c @ b'<' => match self.peek() {
                    Some(b'=') => {
                        tokens.push(Token {
                            kind: lookahead_map[c].0,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        });
                        self.index += 2;
                        self.cur_char += 2;
                    }
                    Some(_) => {
                        tokens.push(Token {
                            kind: lookahead_map[c].1,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        });
                        self.index += 1;
                        self.cur_char += 1;
                    }
                    _ => {
                        self.index += 1;
                        self.cur_char += 1;
                    }
                },
                size_1 => {
                    match size_1 {
                        b' ' | b'\t' => { /* skip whitespace */ }
                        b'\n' => {
                            self.cur_line += 1;
                            self.cur_char = 0;
                        }
                        b'+' => tokens.push(Token {
                            kind: TokenKind::Plus,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        }),
                        b'-' => tokens.push(Token {
                            kind: TokenKind::Minus,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        }),
                        b'*' => tokens.push(Token {
                            kind: TokenKind::Star,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        }),
                        b'/' => tokens.push(Token {
                            kind: TokenKind::Slash,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        }),
                        b'(' => tokens.push(Token {
                            kind: TokenKind::OpenParen,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        }),
                        b')' => tokens.push(Token {
                            kind: TokenKind::CloseParen,
                            position: Position {
                                line: self.cur_line,
                                character: self.cur_char,
                            },
                        }),
                        other => println!(
                            "Unrecognized byte: '{}' (0x{:x}), skipping...",
                            *other as char, *other
                        ),
                    }
                    self.index += 1;
                    self.cur_char += 1;
                }
            }
        }
        tokens
    }

    fn current(&self) -> Option<&'a u8> {
        self.source.get(self.index)
    }

    fn peek(&self) -> Option<&'a u8> {
        self.source.get(self.index + 1)
    }

    fn consume_string(&mut self) -> &'a str {
        let start = self.index;
        while let Some(c) = self.current() {
            if *c == b'"' {
                break;
            }
            self.index += 1;
            self.cur_char += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn consume_identifier(&mut self) -> &'a str {
        let start = self.index;
        while let Some(c) = self.current() {
            if !c.is_ascii_alphanumeric() && *c != b'_' {
                break;
            }
            self.index += 1;
            self.cur_char += 1;
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    fn consume_number(&mut self) -> f64 {
        let start = self.index;
        while let Some(b'0'...b'9') = self.current() {
            self.index += 1;
            self.cur_char += 1;
        }
        if let Some(b'.') = self.current() {
            if let Some(b'0'...b'9') = self.peek() {
                self.index += 2;
                self.cur_char += 2;
                while let Some(b'0'...b'9') = self.current() {
                    self.index += 1;
                    self.cur_char += 1;
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
