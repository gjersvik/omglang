use crate::tokens::{
    Tokens,
    TokenType,
};

#[derive(Debug, PartialEq)]
pub enum Exp {
    Block(Vec<Exp>),
    Call(String, Vec<Exp>),
    LiteralUInt(u64),
    InValid,
}


pub fn parse_block(tokens: &mut Tokens) -> Exp {
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

pub fn parse(tokens: &mut Tokens) -> Exp {
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