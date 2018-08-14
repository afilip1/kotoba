use crate::source_stream::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    Nil,

    OpenParen,
    CloseParen,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Colon,
    Comma,
    Semicolon,

    If,
    Else,
    While,
    Fn,
    Ret,
    Nonlocal,
}

pub struct Lexer<'source> {
    source: SourceStream<'source>,
    lookahead_map: HashMap<u8, (TokenKind, TokenKind)>,
    peek_cache: Option<Token>,
}

impl Iterator for Lexer<'source> {
    type Item = Token;

    /// Consumes some source code, yielding an appropriate `Token`.
    /// Returns `None` only when source stream is empty.
    fn next(&mut self) -> Option<Self::Item> {
        if self.peek_cache.is_some() {
            return self.peek_cache.take();
        }

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
                        b'%' => TokenKind::Percent,
                        b'(' => TokenKind::OpenParen,
                        b')' => TokenKind::CloseParen,
                        b':' => TokenKind::Colon,
                        b',' => TokenKind::Comma,
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
            peek_cache: None,
        }
    }

    /// Peeks next token in the stream without consuming it.
    ///
    /// Peeking a certain token the first time advances the iterator, all subsequent calls
    /// to `peek()` and first call to `next()` will return the cached value instead.
    pub fn peek(&mut self) -> Option<Token> {
        if self.peek_cache.is_some() {
            return self.peek_cache.clone();
        }

        self.next().map(|t| {
            self.peek_cache = Some(t.clone());
            t
        })
    }

    pub fn expect(&mut self, expected: &TokenKind) -> Option<Token> {
        if let Some(t) = self.peek() {
            if t.kind == *expected {
                return self.next();
            }
        }
        None
    }

    pub fn expect_any(&mut self, expected: &[TokenKind]) -> Option<Token> {
        if let Some(t) = self.peek() {
            if expected.iter().any(|e| &t.kind == e) {
                return self.next();
            }
        }
        None
    }

    pub fn expect_identifier(&mut self) -> Option<String> {
        if let Some(t) = self.peek() {
            if let TokenKind::Identifier(id) = t.kind {
                self.next();
                return Some(id);
            }
        }
        None
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
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "while" => TokenKind::While,
                "fn" => TokenKind::Fn,
                "ret" => TokenKind::Ret,
                "nonlocal" => TokenKind::Nonlocal,
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
