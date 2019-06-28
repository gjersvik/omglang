use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    #[end]
    End,

    #[error]
    Error,

    #[token = "print"]
    Print,

    #[regex = "\\d+"]
    Number,

    #[token = ";"]
    EndOfExp,
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub slice: &'a str,
}

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

    pub fn next(&mut self) -> &Token {
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.index = self.tokens.len() - 1;
        }
        self.current()
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index]
    }
}
