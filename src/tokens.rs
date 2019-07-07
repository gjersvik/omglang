use crate::error::{OmgError, Position, Result};

use logos::{Extras, Logos};

#[derive(Default)]
pub struct TokenExtra {
    line: u64,
    column: u64,
}

impl Extras for TokenExtra {
    fn on_advance(&mut self) {}

    fn on_whitespace(&mut self, byte: u8) {
        if byte == 10 {
            self.line += 1;
            self.column = 1;
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

    #[token = "("]
    ParenthesesOpen,

    #[token = ")"]
    ParenthesesClose,

    #[token = ","]
    Comma,

    #[token = ";"]
    Semicolon,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub slice: &'a str,
    pub line: u64,
    pub column: u64,
}

#[derive(Debug)]
pub struct Tokens<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
    file: String,
}

impl<'a> Tokens<'a> {
    pub fn lex(code: &'a str, file: String) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut lexer = TokenType::lexer(code);
        lexer.extras.line = 1;
        lexer.extras.column = 1;
        loop {
            if lexer.token == TokenType::Error {
                return Err(OmgError::new(
                    format!("Found unknown character in \"{}\" in file.", lexer.slice()),
                    Position::new(file.to_string(), lexer.extras.line, lexer.extras.column),
                ));
            }
            tokens.push(Token {
                token_type: lexer.token,
                slice: lexer.slice(),
                line: lexer.extras.line,
                column: lexer.extras.column,
            });
            if lexer.token == TokenType::End {
                break;
            }
            lexer.extras.column += lexer.slice().len() as u64;
            lexer.advance();
        }

        Ok(Tokens {
            tokens,
            index: 0,
            file,
        })
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

    pub fn position(&self) -> Position {
        let token = self.current();
        Position::new(self.file.to_string(), token.line, token.column)
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
        Tokens::lex(code, "test_code".to_string()).expect("Failed to tokenize test_data:")
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
        assert_eq!(tokens.current().column, 1);
        tokens.next();
        assert_eq!(tokens.current().column, 6);
        tokens.next();
        assert_eq!(tokens.current().column, 8);
        tokens.next();
        assert_eq!(tokens.current().column, 10);
        tokens.next();
        assert_eq!(tokens.current().column, 11);
    }

    #[test]
    fn track_line() {
        let mut tokens = Tokens::new_test("test 2\r\n3 4");
        assert_eq!(tokens.current().line, 1);
        tokens.next();
        assert_eq!(tokens.current().line, 1);
        tokens.next();
        assert_eq!(tokens.current().line, 2);
        tokens.next();
        assert_eq!(tokens.current().line, 2);
        tokens.next();
        assert_eq!(tokens.current().line, 2);
        assert_eq!(tokens.current().column, 4);
    }

    #[test]
    fn position() {
        let mut tokens = Tokens::lex("test 2\r\n3 4", "test.omg".to_owned()).unwrap();
        tokens.next();
        tokens.next();
        tokens.next();
        assert_eq!(
            tokens.position(),
            Position::new("test.omg".to_owned(), 2, 3)
        );
    }

    #[test]
    fn unknown_token() {
        let result = Tokens::lex("test @", "test.omg".to_owned());
        let err = result.expect_err("@ should not be valid omg code for this test");

        assert_eq!(err.msg, "Found unknown character in \"@\" in file.");
    }
}
