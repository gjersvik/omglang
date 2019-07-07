use std::fmt;

#[derive(Debug)]
pub struct OmgError {
    pub msg: String,
    pub pos: Position,
}

impl OmgError {
    pub fn new(msg: String, pos: Position) -> Self {
        OmgError { msg, pos }
    }
}

impl fmt::Display for OmgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.pos, self.msg)
    }
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub src: String,
    pub line: u64,
    pub column: u64,
}

impl Position {
    pub fn new(src: String, line: u64, column: u64) -> Self {
        Position { src, line, column }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.src, self.line, self.column)
    }
}

pub type Result<T> = std::result::Result<T, OmgError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let pos = Position::new("test.omg".to_owned(), 1, 2);
        let display = format!("{}", pos);
        assert_eq!(display, "test.omg:1:2");
    }

    #[test]
    fn omg_error() {
        let pos = Position::new("test.omg".to_owned(), 1, 2);
        let error = OmgError::new("Test error".to_owned(), pos);
        let display = format!("{}", error);
        assert_eq!(display, "test.omg:1:2: Test error\n");
    }
}
