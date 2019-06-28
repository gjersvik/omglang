#![warn(clippy::all)]
mod tokens;

use std::vec::Vec;
use tokens::{TokenType, Tokens};

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

fn parse_block(tokens: &mut Tokens) -> Exp {
    let mut expressions = Vec::new();
    loop {
        let exp = parse(tokens);
        if exp == Exp::InValid {
            break;
        }
        let t = tokens.next();
        if t.token_type == TokenType::EndOfExp {
            expressions.push(exp)
        }
    }
    Exp::Block(expressions)
}

fn parse(tokens: &mut Tokens) -> Exp {
    let token = tokens.next();

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
    let mut tokens = Tokens::lex(code);
    let exp = parse_block(&mut tokens);
    println!("Exp: {:?}", exp);
    println!("Running program:");
    run_exp(&exp);
    println!("Program done");
}
