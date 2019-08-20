use crate::pipeline::ast::*;
use crate::{
    error::{OmgError, Result},
    pipeline::{Token, Tokens},
    value::Value,
};

pub fn parse_block(tokens: &mut Tokens) -> Result<Exp> {
    let mut statements = Vec::new();
    let pos = tokens.position();
    loop {
        let exp = parse(tokens)?;
        tokens.next();
        let token = tokens.current();
        let slice = tokens.slice();
        if token == Token::Semicolon {
            statements.push(exp);
            tokens.next();
        } else if token != Token::EndOfFile {
            return Err(OmgError::new(
                format!("Expected ; found {}", slice),
                tokens.position(),
            ));
        }
        if tokens.current() == Token::EndOfFile {
            break;
        }
    }
    Ok(Exp::new_block(statements, pos))
}

pub fn parse(tokens: &mut Tokens) -> Result<Exp> {
    // parse left hand side;
    let lhs = match tokens.current() {
        Token::Identifier => parse_identifier(tokens),
        Token::Number => match tokens.slice().parse() {
            Ok(i) => Ok(Exp::new_literal(Value::Number(i), tokens.position())),
            Err(err) => Err(OmgError::new(
                format!(
                    "Unable to covert {} into an integer: {}",
                    tokens.slice(),
                    err
                ),
                tokens.position(),
            )),
        },
        Token::True => Ok(Exp::new_literal(Value::True, tokens.position())),
        Token::False => Ok(Exp::new_literal(Value::False, tokens.position())),
        _ => Err(OmgError::new(
            format!("Expected identifier or number found {}", tokens.slice()),
            tokens.position(),
        )),
    }?;

    let op_type = match tokens.peek() {
        Token::OpAdd => OpType::Add,
        Token::OpSubtract => OpType::Subtract,
        Token::OpMultiply => OpType::Multiply,
        Token::OpDivide => OpType::Divide,
        Token::OpEqual => OpType::Equal,
        Token::OpGreaterThan => OpType::GreaterThan,
        Token::OpLessThan => OpType::LessThan,
        _ => return Ok(lhs),
    };

    tokens.next(); // at Operator
    tokens.next(); // at next expression
    let rhs = parse(tokens)?;
    let pos = lhs.position();
    Ok(Exp::new_operator(
        op_type,
        Box::new(lhs),
        Box::new(rhs),
        pos,
    ))
}

fn parse_identifier(tokens: &mut Tokens) -> Result<Exp> {
    match tokens.peek() {
        Token::ParenthesesOpen => parse_call(tokens),
        Token::Assignment => {
            let name = tokens.slice().to_string();
            let pos = tokens.position();
            tokens.next(); // at assignment
            tokens.next(); // at next expression
            let exp = parse(tokens)?;
            Ok(Exp::new_assignment(name, Box::new(exp), pos))
        }
        _ => Ok(Exp::new_variable(
            tokens.slice().to_string(),
            tokens.position(),
        )),
    }
}

fn parse_call(tokens: &mut Tokens) -> Result<Exp> {
    let pos = tokens.position();
    let name = tokens.slice().to_string();
    tokens.next(); // ParenthesesOpen
    let mut args = Vec::new();
    if !tokens.expect(Token::ParenthesesClose) {
        loop {
            tokens.next();
            args.push(parse(tokens)?);
            tokens.next();
            match tokens.current() {
                Token::ParenthesesClose => break,
                Token::Comma => (),
                _ => {
                    return Err(OmgError::new(
                        format!("Expected ) or , found {}", tokens.slice()),
                        tokens.position(),
                    ))
                }
            };
        }
    }
    Ok(Exp::new_call(name, args, pos))
}
