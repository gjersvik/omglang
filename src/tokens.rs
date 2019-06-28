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

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = TokenType::lexer(code);
    while lexer.token != TokenType::End {
        tokens.push(Token {
            token_type: lexer.token,
            slice: lexer.slice(),
        });
        lexer.advance();
    }
    tokens
}
