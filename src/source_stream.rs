use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.character)
    }
}

pub struct SourceStream<'source> {
    source: &'source [u8],
    index: usize,
    cur_line: usize,
    cur_char: usize,
}

impl<'s> SourceStream<'s> {
    /// Initializes a new `SourceStream` with the given source code `&str`.
    /// `source` must be a valid ASCII string.
    pub fn new(source: &'s str) -> Self {
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
        self.source.get(self.index).cloned()
    }

    /// Returns the next byte in the stream, consuming it,
    /// or `None` if the stream is empty.
    pub fn next(&mut self) -> Option<u8> {
        self.source.get(self.index).map(|&c| {
            self.index += 1;
            if c == b'\n' {
                self.cur_line += 1;
                self.cur_char = 1;
            } else {
                self.cur_char += 1;
            }
            c
        })
    }

    /// If the next byte in the stream is equal to `expected`,
    /// consumes it and return `true`, otherwise returns `false`.
    pub fn expect(&mut self, expected: u8) -> bool {
        self.peek()
            .filter(|&c| c == expected)
            .map(|_| self.next())
            .is_some()
    }

    /// Consumes the bytes in the stream while `predicate` is true,
    /// and returns them all as `&str`. Does not consume the first byte that
    /// fails the `predicate` check (cf. `Iterator::take_while`).
    pub fn take_while(&mut self, predicate: impl Fn(&u8) -> bool) -> &'s str {
        let start = self.index;
        while self.peek().filter(&predicate).is_some() {
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
