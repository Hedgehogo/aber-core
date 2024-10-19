use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub byte: usize,
    pub line: usize,
    pub collumn: usize,
}

impl Position {
    pub fn new(byte: usize, line: usize, collumn: usize) -> Self {
        Self {
            byte,
            line,
            collumn,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.collumn, self.byte)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span<'input> {
    pub slice: &'input str,
    pub begin: Position,
    pub end: Position,
}

impl<'input> Span<'input> {
    pub fn new(slice: &'input str, begin: Position, end: Position) -> Self {
        Self { slice, begin, end }
    }
}

impl<'input> fmt::Debug for Span<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}..{:?}", self.begin, self.end)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Spanned<'input, T>(Span<'input>, T);

impl<'input, T: fmt::Debug> fmt::Debug for Spanned<'input, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} @ {:?}", self.1, self.0)
    }
}
