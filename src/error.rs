use std::path::PathBuf;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub struct OmgError {
    pub msg: String,
    pub pos: String,
}

impl OmgError {
    pub fn new<S>(msg: S, pos: Position) -> Self
    where
        S: Into<String>,
    {
        OmgError {
            msg: msg.into(),
            pos: pos.to_string(),
        }
    }
}

impl fmt::Display for OmgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.pos, self.msg)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub src: Arc<String>,
    pub line: u64,
    pub column: u64,
}

impl Position {
    pub fn new<S>(src: S) -> Self
    where
        S: Into<String>,
    {
        Position {
            src: Arc::new(src.into()),
            line: 1,
            column: 1,
        }
    }

    pub fn with_pos(&self, line: u64, column: u64) -> Self {
        Position {
            src: Arc::clone(&self.src),
            line,
            column,
        }
    }

    pub fn add(&self, count: u64) -> Self {
        Position {
            src: Arc::clone(&self.src),
            line: self.line,
            column: self.column + count,
        }
    }

    pub fn newline(&self) -> Self {
        Position {
            src: Arc::clone(&self.src),
            line: self.line + 1,
            column: 1,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.src, self.line, self.column)
    }
}

impl From<String> for Position {
    fn from(path: String) -> Self {
        Position::new(path)
    }
}

impl From<PathBuf> for Position {
    fn from(path: PathBuf) -> Self {
        Position::new(path.to_string_lossy())
    }
}

pub type Result<T> = std::result::Result<T, OmgError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let pos = Position::new("test.omg").with_pos(1, 2);
        let display = format!("{}", pos);
        assert_eq!(display, "test.omg:1:2");
    }

    #[test]
    fn omg_error() {
        let pos = Position::new("test.omg").with_pos(1, 2);
        let error = OmgError::new("Test error".to_owned(), pos);
        let display = format!("{}", error);
        assert_eq!(display, "test.omg:1:2: Test error\n");
    }
}
