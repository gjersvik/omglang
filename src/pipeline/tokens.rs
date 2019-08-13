use crate::error::Position;
use im::Vector;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Identifier,
    Number,
    True,
    False,
    ParenthesesOpen,
    ParenthesesClose,
    Comma,
    Semicolon,
    Assignment,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpEqual,
    OpGreaterThan,
    OpLessThan,
    EndOfFile,
}

#[derive(Debug, Clone)]
struct MetaData {
    pub slice: String,
    pub line: u64,
    pub column: u64,
}

#[derive(Debug)]
pub struct Tokens {
    path: String,
    tokens: Vector<Token>,
    metadata: Vector<MetaData>,
    index: usize,
}

impl Tokens {
    pub fn new(path: String) -> Self {
        Tokens {
            path,
            tokens: Vector::new(),
            metadata: Vector::new(),
            index: 0,
        }
    }

    pub fn push(&mut self, token: Token, slice: String, line: u64, column: u64){
        self.tokens.push_back(token);
        self.metadata.push_back(MetaData {slice, line, column});
    } 

    pub fn next(&mut self) {
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.index = self.tokens.len() - 1;
        }
    }

    pub fn expect(&mut self, token: Token) -> bool {
        if self.get(self.index + 1) == token {
            self.next();
            return true;
        }
        false
    }

    pub fn current(&self) -> Token {
        self.tokens[self.index]
    }

    pub fn peek(&self) -> Token {
        self.get(self.index + 1)
    }

    pub fn position(&self) -> Position {
        let meta = self.get_meta(self.index);
        Position::new(&self.path).with_pos(meta.line, meta.column)
    }

    pub fn slice(&self) -> &str {
        &self.get_meta(self.index).slice
    }

    fn get(&self, index: usize) -> Token {
        if index >= self.tokens.len() {
            return self.tokens[self.tokens.len() - 1];
        }
        self.tokens[index]
    }

    fn get_meta(&self, index: usize) -> &MetaData {
        if index >= self.metadata.len() {
            return &self.metadata[self.metadata.len() - 1];
        }
        &self.metadata[index]
    }
}
