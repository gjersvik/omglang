use crate::{
    error::{OmgError, Position, Result},
    tokens::{TokenType, Tokens},
    value::Value,
};

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Exp>,
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: String,
    pub args: Vec<Exp>,
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
    pub value: Value,
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub enum Exp {
    Block(Block),
    Call(Call),
    Literal(Literal),
}

impl Exp {
    fn new_block(statements: Vec<Exp>, pos: Position) -> Exp {
        Exp::Block(Block { statements, pos })
    }

    fn new_call(name: String, args: Vec<Exp>, pos: Position) -> Exp {
        Exp::Call(Call { name, args, pos })
    }

    fn new_literal(value: Value, pos: Position) -> Exp {
        Exp::Literal(Literal { value, pos })
    }
}

pub fn parse_block(tokens: &mut Tokens) -> Result<Exp> {
    let mut statements = Vec::new();
    let pos = tokens.position();
    loop {
        let exp = parse(tokens)?;
        tokens.next();
        let t = tokens.current();
        if t.token_type == TokenType::Semicolon {
            statements.push(exp);
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
    Ok(Exp::new_block(statements, pos))
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
            Ok(Exp::new_call(name, args, pos))
        }
        TokenType::Number => match token.slice.parse() {
            Ok(i) => Ok(Exp::new_literal(Value::Int(i), pos)),
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

    #[cfg_attr(tarpaulin, skip)]
    impl Exp {
        fn block(&self) -> Result<&Block> {
            if let Exp::Block(exp) = self {
                Ok(&exp)
            } else {
                Err(self.type_error("Block"))
            }
        }

        fn call(&self) -> Result<&Call> {
            if let Exp::Call(exp) = self {
                Ok(&exp)
            } else {
                Err(self.type_error("Call"))
            }
        }

        fn literal(&self) -> Result<&Literal> {
            if let Exp::Literal(exp) = self {
                Ok(&exp)
            } else {
                Err(self.type_error("Literal"))
            }
        }

        fn position(&self) -> Position {
            match self {
                Exp::Block(b) => b.pos.clone(),
                Exp::Call(c) => c.pos.clone(),
                Exp::Literal(l) => l.pos.clone(),
            }
        }

        fn name(&self) -> &'static str {
            match self {
                Exp::Block(_) => "Block",
                Exp::Call(_) => "Call",
                Exp::Literal(_) => "Literal",
            }
        }

        fn type_error(&self, expected: &str) -> OmgError {
            OmgError::new(
                format!("Expected {} found {}", expected, self.name()),
                self.position(),
            )
        }
    }

    #[test]
    fn number() {
        let mut tokens = Tokens::new_test("42");
        let exp = parse(&mut tokens).unwrap();
        let literal = exp.literal().unwrap();
        assert_eq!(literal.value, Value::Int(42));
    }

    #[test]
    fn number_to_big() {
        let mut tokens = Tokens::new_test("12345678901234567890");
        parse(&mut tokens).unwrap_err();
    }

    #[test]
    fn call() {
        let mut tokens = Tokens::new_test("print(42)");
        let exp = parse(&mut tokens).unwrap();
        let call = exp.call().unwrap();
        assert_eq!(call.name, "print".to_string());
        let arg = call.args[0].literal().unwrap();
        assert_eq!(arg.value, Value::Int(42));
    }

    #[test]
    fn call_two_args() {
        let mut tokens = Tokens::new_test("print(1,2)");
        let exp = parse(&mut tokens).unwrap();
        let call = exp.call().unwrap();
        assert_eq!(call.name, "print".to_string());
        let arg = call.args[0].literal().unwrap();
        assert_eq!(arg.value, Value::Int(1));
        let arg = call.args[1].literal().unwrap();
        assert_eq!(arg.value, Value::Int(2));
    }

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

    #[test]
    fn block() {
        let mut tokens = Tokens::new_test("42;");
        let exp = parse_block(&mut tokens).unwrap();
        let block = exp.block().unwrap();
        let statement = block.statements[0].literal().unwrap();
        assert_eq!(statement.value, Value::Int(42));
    }

    #[test]
    fn block_error() {
        let mut tokens = Tokens::new_test("42(");
        parse_block(&mut tokens).unwrap_err();
    }

    #[test]
    fn in_valid() {
        let mut tokens = Tokens::new_test(",");
        parse(&mut tokens).unwrap_err();
    }
}
