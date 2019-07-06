use std::fmt;

#[derive(Debug)]
pub struct OmgError {
    pub msg: String,
    pub pos: Position,
}

impl fmt::Display for OmgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.pos, self.msg)
    }
}

#[derive(Debug)]
pub struct Position {
    pub src: String,
    pub line: u64,
    pub column: u64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.src, self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let pos = Position {
            src: "test.omg".to_owned(),
            line: 1,
            column: 2,
        };
        let display = format!("{}", pos);
        assert_eq!(display, "test.omg:1:2");
    }

    #[test]
    fn omg_error() {
        let pos = Position {
            src: "test.omg".to_owned(),
            line: 1,
            column: 2,
        };
        let error = OmgError {
            msg: "Test error".to_owned(),
            pos,
        };
        let display = format!("{}", error);
        assert_eq!(display, "test.omg:1:2: Test error\n");
    }

}
