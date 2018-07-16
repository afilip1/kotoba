use crate::source_stream::*;
use std::collections::HashMap;

macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => ({
        let mut tmp = std::collections::HashMap::new();
        $(tmp.insert($key, $value);)*
        tmp
    });
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
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
    And,
    Or,
    Semicolon,
}

pub struct Lexer<'source> {
    source: SourceStream<'source>,
    lookahead_map: HashMap<u8, (TokenKind, TokenKind)>,
}

impl Iterator for Lexer<'source> {
    type Item = Token;

    /// Consumes some source code, yielding an appropriate `Token`.
    /// Returns `None` only when source stream is empty.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.source.peek() {
            match c {
                b'0'...b'9' => return Some(self.handle_number()),
                b'a'...b'z' | b'A'...b'Z' | b'_' => return Some(self.handle_identifier()),
                b'"' => return Some(self.handle_string()),
                b'=' | b'!' | b'>' | b'<' => return Some(self.handle_size_2_operator()),
                size_1 => {
                    let position = self.source.current_position();
                    self.source.next();
                    let kind = match size_1 {
                        b' ' | b'\t' | b'\r' | b'\n' => continue, // skip whitespace
                        b'+' => TokenKind::Plus,
                        b'-' => TokenKind::Minus,
                        b'*' => TokenKind::Star,
                        b'/' => TokenKind::Slash,
                        b'(' => TokenKind::OpenParen,
                        b')' => TokenKind::CloseParen,
                        b';' => TokenKind::Semicolon,
                        other => {
                            println!(
                                "Unrecognized byte '{}' (0x{:x}) at position {}, skipping...",
                                other as char, other, position
                            );
                            continue;
                        }
                    };
                    return Some(Token { kind, position });
                }
            }
        }
        None
    }
}

impl Lexer<'source> {
    /// Initializes a new `Lexer` with the given source code `&str`.
    /// `source` must be a valid ASCII string.
    pub fn new(source: &'source str) -> Self {
        Self {
            source: SourceStream::new(source),
            lookahead_map: hashmap! {
                b'=' => (TokenKind::EqualEqual, TokenKind::Equal),
                b'!' => (TokenKind::BangEqual, TokenKind::Bang),
                b'>' => (TokenKind::GreaterEqual, TokenKind::Greater),
                b'<' => (TokenKind::LessEqual, TokenKind::Less)
            },
        }
    }

    /// Consumes the bytes that make a number literal, yielding a `Number` token.
    fn handle_number(&mut self) -> Token {
        let position = self.source.current_position();
        let start = self.source.index;

        // read whole part
        while let Some(b'0'...b'9') = self.source.peek() {
            self.source.next();
        }

        // use second char lookahead to ensure . means float, not member access
        if let Some(b'.') = self.source.peek() {
            if let Some(b'0'...b'9') = self.source.peek_second() {
                // ok, read fractional part
                self.source.expect(b'.');
                while let Some(b'0'...b'9') = self.source.peek() {
                    self.source.next();
                }
            }
        }

        let end = self.source.index;
        let number = std::str::from_utf8(&self.source.source[start..end])
            .unwrap()
            .parse()
            .unwrap();

        Token {
            position,
            kind: TokenKind::Number(number),
        }
    }

    /// Consumes the bytes that make an identifier, yielding an appropriate token.
    fn handle_identifier(&mut self) -> Token {
        Token {
            position: self.source.current_position(),
            kind: match self
                .source
                .consume_while(|c| c.is_ascii_alphanumeric() || c == b'_')
            {
                "true" => TokenKind::Boolean(true),
                "false" => TokenKind::Boolean(false),
                "nil" => TokenKind::Nil,
                "and" => TokenKind::And,
                "or" => TokenKind::Or,
                other => TokenKind::Identifier(other.to_owned()),
            },
        }
    }

    /// Consumes the bytes that make a string literal, yielding a `StringLiteral` token.
    /// Panics if no closing quote was found.
    fn handle_string(&mut self) -> Token {
        let position = self.source.current_position();

        self.source.expect(b'"');
        let string_contents = self.source.consume_while(|c| c != b'"');
        if self.source.expect(b'"') {
            Token {
                kind: TokenKind::StringLiteral(string_contents.to_owned()),
                position,
            }
        } else {
            println!("Unclosed string literal");
            std::process::exit(1);
        }
    }

    /// Consumes a one-byte or a two-byte operator, yielding an appropriate token.
    fn handle_size_2_operator(&mut self) -> Token {
        let c = self.source.peek().unwrap();
        match self.source.peek_second() {
            Some(b'=') => {
                let ret = self.token_at_cur_pos(self.lookahead_map[&c].clone().0);
                self.source.next(); //TODO: adjust position code for this
                self.source.next();
                ret
            }
            _ => {
                let ret = self.token_at_cur_pos(self.lookahead_map[&c].clone().1);
                self.source.next();
                ret
            }
        }
    }

    /// Constructs a token with `position` set to the current position in source.
    fn token_at_cur_pos(&self, token_kind: TokenKind) -> Token {
        Token {
            kind: token_kind,
            position: self.source.current_position(),
        }
    }
}
