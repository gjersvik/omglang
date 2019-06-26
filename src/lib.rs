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

#[derive(Debug, PartialEq)]
enum Exp {
    Block(Vec<Exp>),
    Print(Box<Exp>),
    LiteralUInt(u64),
    InValid,
}

enum Value {
    Nothing,
    UInt(u64),
}

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

fn parse_block<'a, I>(tokens: &mut I) -> Exp
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut expressions = Vec::new();
    loop {
        let exp = parse(tokens);
        if exp == Exp::InValid {
            break;
        }
        match tokens.next() {
            Some(t) => {
                if t.token_type == TokenType::EndOfExp {
                    expressions.push(exp)
                }
            }
            None => break,
        };
    }
    return Exp::Block(expressions);
}

fn parse<'a, I>(tokens: &mut I) -> Exp
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let token = match tokens.next() {
        Some(t) => t,
        None => return Exp::InValid,
    };

    match token.token_type {
        TokenType::Print => return Exp::Print(Box::new(parse(tokens))),
        TokenType::Number => return Exp::LiteralUInt(token.slice.parse().unwrap()),
        _ => return Exp::InValid,
    }
}

fn run_exp(exp: &Exp) -> Value {
    match exp {
        Exp::Print(e) => {
            let text = match run_exp(&e) {
                Value::UInt(i) => format!("{}", i),
                Value::Nothing => format!("Nothing"),
            };
            println!("{}", text);
            return Value::Nothing;
        }
        Exp::Block(block) => {
            for e in block {
                run_exp(e);
            }
            return Value::Nothing;
        }
        Exp::LiteralUInt(int) => return Value::UInt(*int),
        Exp::InValid => panic!("Invalid expression found."),
    }
}

pub fn run(code: &str) {
    let tokens = tokenize(code);
    for token in &tokens {
        println!("Token {:?}: {}", token.token_type, token.slice)
    }
    let exp = parse_block(&mut tokens.iter());
    println!("Exp: {:?}", exp);
    println!("Running program:");
    run_exp(&exp);
    println!("Program done");
}
