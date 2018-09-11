use crate::source_stream::*;

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
    Nonlocal, // such hack much bodge wow
}

pub struct Lexer<'source> {
    source: SourceStream<'source>,
    peek_cache: Option<Token>,
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    /// Consumes some source code, yielding an appropriate `Token`.
    /// Returns `None` only when source stream is empty.
    fn next(&mut self) -> Option<Self::Item> {
        if self.peek_cache.is_some() {
            return self.peek_cache.take();
        }

        self.source.take_while(u8::is_ascii_whitespace);
        let position = self.source.current_position();

        self.source.peek().map(|c| match c {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.handle_identifier(position),
            b'=' | b'!' | b'>' | b'<' => self.handle_size_2_operator(position),
            b'0'..=b'9' => self.handle_number(position),
            b'"' => self.handle_string(position),
            _ => self.handle_size_1_token(position),
        })
    }
}

impl<'s> Lexer<'s> {
    /// Initializes a new `Lexer` with the given source code `&str`.
    /// `source` must be a valid ASCII string.
    pub fn new(source: &'s str) -> Self {
        Self {
            source: SourceStream::new(source),
            peek_cache: None,
        }
    }

    /// Peeks next token in the stream without consuming it.
    ///
    /// Peeking a certain token the first time advances the iterator, all
    /// subsequent calls to `peek()` and first call to `next()` will return
    /// the cached value instead.
    pub fn peek(&mut self) -> Option<Token> {
        if self.peek_cache.is_none() {
            self.peek_cache = self.next();
        }

        self.peek_cache.clone()
    }

    pub fn expect(&mut self, expected: &TokenKind) -> Option<Token> {
        self.peek()
            .filter(|t| t.kind == *expected)
            .and_then(|_| self.next())
    }

    pub fn expect_any(&mut self, expected: &[TokenKind]) -> Option<Token> {
        self.peek()
            .filter(|t| expected.iter().any(|e| &t.kind == e))
            .and_then(|_| self.next())
    }

    #[rustfmt::skip]
    pub fn expect_identifier(&mut self) -> Option<String> {
        if let Some(Token { kind: TokenKind::Identifier(id), .. }) = self.peek() {
            self.next();
            return Some(id);
        }

        None
    }

    fn handle_size_1_token(&mut self, position: Position) -> Token {
        let kind = match self.source.next().unwrap() {
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
            other => panic!(
                "lexical error: unrecognized byte '{}' (0x{:x}) at position {}",
                other as char, other, position
            ),
        };

        Token { kind, position }
    }

    /// Consumes the bytes that make a number literal, yielding a `Number`
    /// token.
    fn handle_number(&mut self, position: Position) -> Token {
        let mut acc = 0.0;
        
        // read whole part
        while let Some(b'0'...b'9') = self.source.peek() {
            acc *= 10.0;
            acc += f64::from(self.source.next().unwrap() - b'0');
        }

        if self.source.expect(b'.') {
            let mut fraction = 10.0;
            // ok, read fractional part
            while let Some(b'0'...b'9') = self.source.peek() {
                acc += f64::from(self.source.next().unwrap() - b'0') / fraction;
                fraction *= 10.0;
            }
        }

        Token {
            position,
            kind: TokenKind::Number(acc),
        }
    }

    /// Consumes the bytes that make an identifier, yielding an appropriate
    /// token.
    fn handle_identifier(&mut self, position: Position) -> Token {
        let is_ident = |c: &u8| c.is_ascii_alphanumeric() || *c == b'_';
        let kind = match self.source.take_while(is_ident) {
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
        };

        Token { position, kind }
    }

    /// Consumes the bytes that make a string literal, yielding a
    /// `StringLiteral` token. Panics if no closing quote was found.
    fn handle_string(&mut self, position: Position) -> Token {
        self.source.expect(b'"');
        let string_contents = self.source.take_while(|c| *c != b'"');
        if !self.source.expect(b'"') {
            panic!("unclosed string literal at position {}", position);
        }

        Token {
            kind: TokenKind::StringLiteral(string_contents.to_owned()),
            position,
        }
    }

    /// Consumes a one-byte or a two-byte operator, yielding an appropriate
    /// token.
    fn handle_size_2_operator(&mut self, position: Position) -> Token {
        let c = self.source.next().unwrap();
        let kind = match self.source.peek() {
            Some(b'=') => {
                self.source.next();
                match c {
                    b'=' => TokenKind::EqualEqual,
                    b'!' => TokenKind::BangEqual,
                    b'>' => TokenKind::GreaterEqual,
                    b'<' => TokenKind::LessEqual,
                    _ => unreachable!(),
                }
            }
            _ => match c {
                b'=' => TokenKind::Equal,
                b'!' => TokenKind::Bang,
                b'>' => TokenKind::Greater,
                b'<' => TokenKind::Less,
                _ => unreachable!(),
            },
        };

        Token { kind, position }
    }
}
