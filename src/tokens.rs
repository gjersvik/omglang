use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
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
}

#[derive(Debug)]
pub struct Tokens<'a> {
    tokens: Vec<Token<'a>>,
    index: usize,
}

impl<'a> Tokens<'a> {
    pub fn lex(code: &'a str) -> Self {
        let mut tokens = Vec::new();
        let mut lexer = TokenType::lexer(code);
        loop {
            tokens.push(Token {
                token_type: lexer.token,
                slice: lexer.slice(),
            });
            if lexer.token == TokenType::End {
                break;
            }
            lexer.advance();
        }

        Tokens { tokens, index: 0 }
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

    fn get(&self, index: usize) -> &Token {
        if index >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1];
        }
        &self.tokens[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_of_each() {
        let mut tokens = Tokens::lex("test 42 ( ) , ;");
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
        let mut tokens = Tokens::lex("42");
        tokens.next(); // At end.
        tokens.next(); // over the end.
        assert_eq!(tokens.current().token_type, TokenType::End);
    }

    #[test]
    fn expect_true() {
        let mut tokens = Tokens::lex("test 42");
        assert_eq!(tokens.expect(TokenType::Number), true);
        assert_eq!(tokens.current().token_type, TokenType::Number);
    }

    #[test]
    fn expect_false() {
        let mut tokens = Tokens::lex("test 42");
        assert_eq!(tokens.expect(TokenType::Semicolon), false);
        assert_eq!(tokens.current().token_type, TokenType::Identifier);
    }

    #[test]
    fn get_pass_end() {
        let tokens = Tokens::lex("42");
        assert_eq!(tokens.get(2).token_type, TokenType::End);
    }
}
