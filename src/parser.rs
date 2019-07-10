use crate::{
    error::{OmgError, Position, Result},
    tokens::{TokenType, Tokens},
};

#[derive(Debug, PartialEq)]
pub struct Exp {
    pub value: ExpValue,
    pub pos: Position,
}

impl Exp {
    pub fn new(value: ExpValue, pos: Position) -> Self {
        Exp { value, pos }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExpValue {
    Block(Vec<Exp>),
    Call(String, Vec<Exp>),
    LiteralUInt(u64),
}

pub fn parse_block(tokens: &mut Tokens) -> Result<Exp> {
    let mut expressions = Vec::new();
    let pos = tokens.position();
    loop {
        let exp = parse(tokens)?;
        tokens.next();
        let t = tokens.current();
        if t.token_type == TokenType::Semicolon {
            expressions.push(exp);
            tokens.next();
        } else if t.token_type != TokenType::End {
            return Err(OmgError::new(
                format!("Expected ; found {}", t.slice),
                tokens.position(),
            ));
        }
        if tokens.current().token_type == TokenType::End {
            break;
        }
    }
    Ok(Exp::new(ExpValue::Block(expressions), pos))
}

pub fn parse(tokens: &mut Tokens) -> Result<Exp> {
    let token = tokens.current();
    let pos = tokens.position();

    match token.token_type {
        TokenType::Identifier => {
            let name = token.slice.to_string();
            tokens.next();
            if tokens.current().token_type != TokenType::ParenthesesOpen {
                return Err(OmgError::new(
                    format!("Expected ( found {}", tokens.current().slice),
                    tokens.position(),
                ));
            }
            let mut args = Vec::new();
            if !tokens.expect(TokenType::ParenthesesClose) {
                loop {
                    tokens.next();
                    args.push(parse(tokens)?);
                    tokens.next();
                    match tokens.current().token_type {
                        TokenType::ParenthesesClose => break,
                        TokenType::Comma => (),
                        _ => {
                            return Err(OmgError::new(
                                format!("Expected ) or , found {}", tokens.current().slice),
                                tokens.position(),
                            ))
                        }
                    };
                }
            }
            Ok(Exp::new(ExpValue::Call(name, args), pos))
        }
        TokenType::Number => match token.slice.parse() {
            Ok(i) => Ok(Exp::new(ExpValue::LiteralUInt(i), pos)),
            Err(err) => Err(OmgError::new(
                format!(
                    "Unable to covert {} into an integer: {}",
                    tokens.current().slice,
                    err
                ),
                tokens.position(),
            )),
        },
        _ => Err(OmgError::new(
            format!(
                "Expected identifier or number found {}",
                tokens.current().slice
            ),
            tokens.position(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let mut tokens = Tokens::new_test("42");
        let exp = parse(&mut tokens).unwrap();
        assert_eq!(exp.value, ExpValue::LiteralUInt(42));
    }

    // #[test]
    // fn call() {
    //     let mut tokens = Tokens::new_test("print(42)");
    //     let exp = parse(&mut tokens).unwrap();
    //     assert_eq!(
    //         exp.value,
    //         ExpValue::Call("print".to_string(), vec![ExpValue::LiteralUInt(42)])
    //     );
    // }

    // #[test]
    // fn call_two_args() {
    //     let mut tokens = Tokens::new_test("print(1,2)");
    //     let exp = parse(&mut tokens).unwrap();
    //     assert_eq!(
    //         exp.value,
    //         ExpValue::Call(
    //             "print".to_string(),
    //             vec![ExpValue::LiteralUInt(1), ExpValue::LiteralUInt(2)]
    //         )
    //     );
    // }

    #[test]
    fn call_no_open() {
        let mut tokens = Tokens::new_test("print42)");
        parse(&mut tokens).unwrap_err();
    }

    #[test]
    fn call_no_end() {
        let mut tokens = Tokens::new_test("print(42");
        parse(&mut tokens).unwrap_err();
    }

    // #[test]
    // fn block() {
    //     let mut tokens = Tokens::new_test("42;");
    //     let exp = parse_block(&mut tokens).unwrap();
    //     assert_eq!(exp.value, ExpValue::Block(vec![ExpValue::LiteralUInt(42)]));
    // }

    #[test]
    fn in_valid() {
        let mut tokens = Tokens::new_test(",");
        parse(&mut tokens).unwrap_err();
    }
}
