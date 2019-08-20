use super::source::Source;
use super::tokens::Tokens;
use crate::error::{OmgError, Position, Result};
use crate::pipeline::tokens::Token;
use logos::{Extras, Logos};

pub fn lexer(source: Source) -> Result<Tokens> {
    let mut tokens = Tokens::new(source.path.clone());
    let mut lexer = TokenType::lexer(&source.source[..]);
    let mut pos = Pos { line: 0, column: 0 };
    loop {
        if lexer.token == TokenType::Error {
            return Err(OmgError::new(
                format!("Found unknown character in \"{}\" in file.", lexer.slice()),
                Position::new(source.path).with_pos(pos.line, pos.column),
            ));
        }
        tokens.push(
            to_token(lexer.token),
            lexer.slice().to_string(),
            pos.line,
            pos.column,
        );
        if lexer.token == TokenType::End {
            break;
        }
        pos.column += lexer.slice().len() as u64;
        lexer.advance();
        pos = lexer.extras.update_pos(pos);
    }

    Ok(tokens)
}

fn to_token(token_type: TokenType) -> Token {
    match token_type {
        TokenType::End => Token::EndOfFile,
        TokenType::Error => Token::EndOfFile,
        TokenType::Identifier => Token::Identifier,
        TokenType::Number => Token::Number,
        TokenType::True => Token::True,
        TokenType::False => Token::False,
        TokenType::ParenthesesOpen => Token::ParenthesesOpen,
        TokenType::ParenthesesClose => Token::ParenthesesClose,
        TokenType::Comma => Token::Comma,
        TokenType::Semicolon => Token::Semicolon,
        TokenType::Assignment => Token::Assignment,
        TokenType::OpAdd => Token::OpAdd,
        TokenType::OpSubtract => Token::OpSubtract,
        TokenType::OpMultiply => Token::OpMultiply,
        TokenType::OpDivide => Token::OpDivide,
        TokenType::OpEqual => Token::OpEqual,
        TokenType::OpGreaterThan => Token::OpGreaterThan,
        TokenType::OpLessThan => Token::OpLessThan,
    }
}

struct Pos {
    line: u64,
    column: u64,
}

#[derive(Default)]
struct TokenExtra {
    line: u64,
    column: u64,
}

impl TokenExtra {
    fn update_pos(&mut self, pos: Pos) -> Pos {
        let mut pos = pos;
        while self.line > 0 {
            pos.line += 1;
            pos.column = 0;
            self.line -= 1;
        }
        pos.column += self.column;
        self.column = 0;

        pos
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
enum TokenType {
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

    #[token = "-"]
    OpSubtract,

    #[token = "*"]
    OpMultiply,

    #[token = "/"]
    OpDivide,

    #[token = "=="]
    OpEqual,

    #[token = ">"]
    OpGreaterThan,

    #[token = "<"]
    OpLessThan,
}
