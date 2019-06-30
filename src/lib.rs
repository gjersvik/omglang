#![warn(clippy::all)]
mod tokens;

use std::vec::Vec;
use tokens::{TokenType, Tokens};

#[derive(Debug, PartialEq)]
enum Exp {
    Block(Vec<Exp>),
    Call(String, Vec<Exp>),
    LiteralUInt(u64),
    InValid,
}

enum Value {
    Nothing,
    UInt(u64),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::UInt(i) => format!("{}", i),
            Value::Nothing => "Nothing".to_string(),
        }
    }
}

fn parse_block(tokens: &mut Tokens) -> Exp {
    let mut expressions = Vec::new();
    loop {
        let exp = parse(tokens);
        if exp == Exp::InValid {
            break;
        }
        tokens.next();
        let t = tokens.current();
        if t.token_type == TokenType::Semicolon {
            expressions.push(exp);
            tokens.next();
        }
    }
    Exp::Block(expressions)
}

fn parse(tokens: &mut Tokens) -> Exp {
    let token = tokens.current();

    match token.token_type {
        TokenType::Identifier => {
            let name = token.slice.to_string();
            tokens.next();
            if tokens.current().token_type != TokenType::ParenthesesOpen {
                return Exp::InValid;
            }
            let mut args = Vec::new();
            if !tokens.expect(TokenType::ParenthesesClose) {
                loop {
                    tokens.next();
                    args.push(parse(tokens));
                    tokens.next();
                    match tokens.current().token_type {
                        TokenType::ParenthesesClose => break,
                        TokenType::Comma => (),
                        _ => return Exp::InValid,
                    };
                }
            }
            Exp::Call(name, args)
        }
        TokenType::Number => Exp::LiteralUInt(token.slice.parse().unwrap()),
        _ => Exp::InValid,
    }
}

fn run_exp(exp: &Exp) -> Value {
    match exp {
        Exp::Call(i, args) => {
            if i != "print" {
                panic!("Cant find function named {} to call", i);
            }
            for exp in args {
                println!("{}", run_exp(&exp).to_string());
            }
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
    println!("Tokens: {:?}", tokens);
    let exp = parse_block(&mut tokens);
    println!("Exp: {:?}", exp);
    println!("Running program:");
    run_exp(&exp);
    println!("Program done");
}
