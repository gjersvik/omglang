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
        while lexer.token != TokenType::End {
            tokens.push(Token {
                token_type: lexer.token,
                slice: lexer.slice(),
            });
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

    pub fn get(&self, index: usize) -> &Token {
        if index >= self.tokens.len() {
            return &Token {
                token_type: TokenType::End,
                slice: "",
            };
        }
        &self.tokens[index]
    }
}
