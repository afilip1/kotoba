use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}:{}", self.line, self.character)
    }
}

pub struct SourceStream<'a> {
    pub source: &'a [u8],
    pub index: usize,
    cur_line: usize,
    cur_char: usize,
}

impl SourceStream<'a> {
    /// Initializes a new `SourceStream` with the given source code `&str`.
    /// `source` must be a valid ASCII string.
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.as_bytes(),
            index: 0,
            cur_line: 1,
            cur_char: 1,
        }
    }

    /// Returns the next byte in the stream without consuming it,
    /// or `None` if the stream is empty.
    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.index).map(|c| *c)
    }

    /// Returns the second next byte in the stream without consuming it,
    /// or `None` if the stream has less than two bytes left.
    pub fn peek_second(&self) -> Option<u8> {
        self.source.get(self.index + 1).map(|c| *c)
    }

    /// Returns the next byte in the stream, consuming it,
    /// or `None` if the stream is empty.
    pub fn next(&mut self) -> Option<u8> {
        self.source.get(self.index).map(|c| {
            self.index += 1;
            match c {
                it @ b'\n' => {
                    self.cur_line += 1;
                    self.cur_char = 1;
                    *it
                }
                it @ _ => {
                    self.cur_char += 1;
                    *it
                }
            }
        })
    }

    /// If the next byte in the stream is equal to `expected`,
    /// consumes it and return `true`, otherwise returns `false`.
    pub fn expect(&mut self, expected: u8) -> bool {
        if let Some(c) = self.peek() {
            if c == expected {
                self.next();
                return true;
            }
        }
        false
    }

    /// Consumes the bytes in the stream while `predicate` is true,
    /// and returns them all as `&str`. Does not consume the first byte that
    /// fails the `predicate` check (cf. `Iterator::take_while()`).
    pub fn consume_while(&mut self, predicate: impl Fn(u8) -> bool) -> &'a str {
        let start = self.index;
        while let Some(c) = self.peek() {
            if !predicate(c) {
                break;
            }
            self.next();
        }
        let end = self.index;
        std::str::from_utf8(&self.source[start..end]).unwrap()
    }

    /// Returns the current position of the source reader.
    pub fn current_position(&self) -> Position {
        Position {
            line: self.cur_line,
            character: self.cur_char,
        }
    }
}
