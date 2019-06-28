#![warn(clippy::all)]
mod tokens;

use std::vec::Vec;
use tokens::{tokenize, Token, TokenType};

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
    Exp::Block(expressions)
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
        TokenType::Print => Exp::Print(Box::new(parse(tokens))),
        TokenType::Number => Exp::LiteralUInt(token.slice.parse().unwrap()),
        _ => Exp::InValid,
    }
}

fn run_exp(exp: &Exp) -> Value {
    match exp {
        Exp::Print(e) => {
            let text = match run_exp(&e) {
                Value::UInt(i) => format!("{}", i),
                Value::Nothing => "Nothing".to_string(),
            };
            println!("{}", text);
            Value::Nothing
        }
        Exp::Block(block) => {
            for e in block {
                run_exp(e);
            }
            Value::Nothing
        }
        Exp::LiteralUInt(int) => Value::UInt(*int),
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
