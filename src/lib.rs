use logos::Logos;
use std::vec::Vec;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
enum TokenType {
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

struct Token<'a> {
    token_type: TokenType,
    slice: &'a str,
}

//enum Exp {
//    Print(Box<Exp>),
//    LiteralUInt(u64)
//}

//enum Value {
//    Nothing,
//    UInt(u64)
//}

fn tokenize(code: &str) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut lexer = TokenType::lexer(code);
    while lexer.token != TokenType::End {
        tokens.push(Token { token_type: lexer.token, slice: lexer.slice()});
        lexer.advance();
    }
    return tokens;
}

pub fn run(code: &str) {
    let tokens = tokenize(code);
    for token in tokens {
        println!("{:?}: {}", token.token_type, token.slice)
    }
    println!("Program done");
}
