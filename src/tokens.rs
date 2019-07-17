use crate::error::{OmgError, Position, Result};

use logos::{Extras, Logos};

#[derive(Default)]
pub struct TokenExtra {
    line: u64,
    column: u64,
}

impl TokenExtra {
    fn update_pos(&mut self, pos: Position) -> Position {
        let mut pos = pos;
        while self.line > 0 {
            pos = pos.newline();
            self.line -= 1;
        }
        pos = pos.add(self.column);
        self.column = 0;

        return pos;
    }
}

impl Extras for TokenExtra {
    fn on_advance(&mut self) {}

    fn on_whitespace(&mut self, byte: u8) {
        if byte == 10 {
            self.line += 1;
            self.column = 0
        } else {
            self.column += 1;
        }
    }
}

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
#[extras = "TokenExtra"]
pub enum TokenType {
    #[end]
    End,

    #[error]
    Error,

    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Identifier,

    #[regex = "\\d+"]
    Number,

    #[token = "true"]
    True,

    #[token = "false"]
    False,

    #[token = "("]
    ParenthesesOpen,

    #[token = ")"]
    ParenthesesClose,

    #[token = ","]
    Comma,

    #[token = ";"]
    Semicolon,

    #[token = "="]
    Assignment,

    #[token = "+"]
    OpAdd,

}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub slice: &'a str,
    pub pos: Position,
}

#[derive(Debug)]
pub struct Tokens<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
}

impl<'a> Tokens<'a> {
    pub fn lex(code: &'a str, file: &'a str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut lexer = TokenType::lexer(code);
        let mut pos = Position::new(file);
        loop {
            if lexer.token == TokenType::Error {
                return Err(OmgError::new(
                    format!("Found unknown character in \"{}\" in file.", lexer.slice()),
                    pos,
                ));
            }
            tokens.push(Token {
                token_type: lexer.token,
                slice: lexer.slice(),
                pos: pos.clone(),
            });
            if lexer.token == TokenType::End {
                break;
            }
            pos = pos.add(lexer.slice().len() as u64);
            lexer.advance();
            pos = lexer.extras.update_pos(pos);
        }

        Ok(Tokens { tokens, index: 0 })
    }

    pub fn next(&mut self) {
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.index = self.tokens.len() - 1;
        }
    }

    pub fn expect(&mut self, token_type: TokenType) -> bool {
        if self.get(self.index + 1).token_type == token_type {
            self.next();
            return true;
        }
        false
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn peek(&self) -> &Token {
        self.get(self.index + 1)
    }

    pub fn position(&self) -> Position {
        self.current().pos.clone()
    }

    pub fn slice(&self) -> &str {
        self.current().slice
    }

    fn get(&self, index: usize) -> &Token {
        if index >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[index]
    }
}

#[cfg(test)]
impl<'a> Tokens<'a> {
    pub fn new_test(code: &'a str) -> Self {
        Tokens::lex(code, "test_code").expect("Failed to tokenize test_data:")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_of_each() {
        let mut tokens = Tokens::new_test("test 42 ( ) , ;");
        assert_eq!(tokens.current().token_type, TokenType::Identifier);
        tokens.next();
        assert_eq!(tokens.current().token_type, TokenType::Number);
        tokens.next();
        assert_eq!(tokens.current().token_type, TokenType::ParenthesesOpen);
        tokens.next();
        assert_eq!(tokens.current().token_type, TokenType::ParenthesesClose);
        tokens.next();
        assert_eq!(tokens.current().token_type, TokenType::Comma);
        tokens.next();
        assert_eq!(tokens.current().token_type, TokenType::Semicolon);
        tokens.next();
        assert_eq!(tokens.current().token_type, TokenType::End);
    }

    #[test]
    fn go_pass_end() {
        let mut tokens = Tokens::new_test("42");
        tokens.next(); // At end.
        tokens.next(); // over the end.
        assert_eq!(tokens.current().token_type, TokenType::End);
    }

    #[test]
    fn expect_true() {
        let mut tokens = Tokens::new_test("test 42");
        assert_eq!(tokens.expect(TokenType::Number), true);
        assert_eq!(tokens.current().token_type, TokenType::Number);
    }

    #[test]
    fn expect_false() {
        let mut tokens = Tokens::new_test("test 42");
        assert_eq!(tokens.expect(TokenType::Semicolon), false);
        assert_eq!(tokens.current().token_type, TokenType::Identifier);
    }

    #[test]
    fn get_pass_end() {
        let tokens = Tokens::new_test("42");
        assert_eq!(tokens.get(2).token_type, TokenType::End);
    }

    #[test]
    fn track_column() {
        let mut tokens = Tokens::new_test("test 2 3 4");
        assert_eq!(tokens.current().pos.column, 1);
        tokens.next();
        assert_eq!(tokens.current().pos.column, 6);
        tokens.next();
        assert_eq!(tokens.current().pos.column, 8);
        tokens.next();
        assert_eq!(tokens.current().pos.column, 10);
        tokens.next();
        assert_eq!(tokens.current().pos.column, 11);
    }

    #[test]
    fn track_line() {
        let mut tokens = Tokens::new_test("test 2\r\n3 4");
        assert_eq!(tokens.current().pos.line, 1);
        tokens.next();
        assert_eq!(tokens.current().pos.line, 1);
        tokens.next();
        assert_eq!(tokens.current().pos.line, 2);
        tokens.next();
        assert_eq!(tokens.current().pos.line, 2);
        tokens.next();
        assert_eq!(tokens.current().pos.line, 2);
        assert_eq!(tokens.current().pos.column, 4);
    }

    #[test]
    fn position() {
        let mut tokens = Tokens::lex("test 2\r\n3 4", "test.omg").unwrap();
        tokens.next();
        tokens.next();
        tokens.next();
        assert_eq!(tokens.position(), Position::new("test.omg").with_pos(2, 3));
    }

    #[test]
    fn slice() {
        let tokens = Tokens::lex("test", "test.omg").unwrap();
        assert_eq!(tokens.slice(), "test");
    }

    #[test]
    fn unknown_token() {
        let result = Tokens::lex("test @", "test.omg");
        let err = result.expect_err("@ should not be valid omg code for this test");

        assert_eq!(err.msg, "Found unknown character in \"@\" in file.");
    }
}
