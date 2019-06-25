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

#[derive(Debug)]
enum Exp {
    Print(Box<Exp>),
    LiteralUInt(u64),
    InValid,
}

//enum Value {
//    Nothing,
//    UInt(u64)
//}

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut lexer = TokenType::lexer(code);
    while lexer.token != TokenType::End {
        tokens.push(Token {
            token_type: lexer.token,
            slice: lexer.slice(),
        });
        lexer.advance();
    }
    return tokens;
}

fn parse(tokens: &[Token]) -> Exp {
    let token = &tokens[0];
    let rest = &tokens[1..];
    match token.token_type {
        TokenType::Print => return Exp::Print(Box::new(parse(rest))),
        TokenType::Number => return Exp::LiteralUInt(token.slice.parse().unwrap()),
        _ => return Exp::InValid,
    }
}

pub fn run(code: &str) {
    let tokens = tokenize(code);
    for token in &tokens {
        println!("Token {:?}: {}", token.token_type, token.slice)
    }
    let exp = parse(&tokens);
    println!("Exp: {:?}", exp);

    println!("Program done");
}
