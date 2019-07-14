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
pub struct Assignment {
    pub name: String,
    pub value: Box<Exp>,
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub enum Exp {
    Block(Block),
    Call(Call),
    Literal(Literal),
    Assignment(Assignment),
    Variable(Variable),
}

impl Exp {
    pub fn new_block(statements: Vec<Exp>, pos: Position) -> Exp {
        Exp::Block(Block { statements, pos })
    }

    pub fn new_call(name: String, args: Vec<Exp>, pos: Position) -> Exp {
        Exp::Call(Call { name, args, pos })
    }

    pub fn new_literal(value: Value, pos: Position) -> Exp {
        Exp::Literal(Literal { value, pos })
    }

    pub fn new_assignment(name: String, value: Box<Exp>, pos: Position) -> Exp {
        Exp::Assignment(Assignment { name, value, pos })
    }

    pub fn new_variable(name: String, pos: Position) -> Exp {
        Exp::Variable(Variable { name, pos })
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
    match tokens.current().token_type {
        TokenType::Identifier => parse_identifier(tokens),
        TokenType::Number => match tokens.current().slice.parse() {
            Ok(i) => Ok(Exp::new_literal(Value::Int(i), tokens.position())),
            Err(err) => Err(OmgError::new(
                format!(
                    "Unable to covert {} into an integer: {}",
                    tokens.current().slice,
                    err
                ),
                tokens.position(),
            )),
        },
        TokenType::True => Ok(Exp::new_literal(Value::True, tokens.position())),
        TokenType::False => Ok(Exp::new_literal(Value::False, tokens.position())),
        _ => Err(OmgError::new(
            format!(
                "Expected identifier or number found {}",
                tokens.current().slice
            ),
            tokens.position(),
        )),
    }
}

fn parse_identifier(tokens: &mut Tokens) -> Result<Exp> {
    match tokens.peek().token_type {
        TokenType::ParenthesesOpen => parse_call(tokens),
        TokenType::Assignment => {
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
    let token = tokens.current();
    let pos = tokens.position();
    let name = token.slice.to_string();
    tokens.next(); // ParenthesesOpen
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

        fn assignment(&self) -> Result<&Assignment> {
            if let Exp::Assignment(exp) = self {
                Ok(&exp)
            } else {
                Err(self.type_error("Assignment"))
            }
        }

        fn variable(&self) -> Result<&Variable> {
            if let Exp::Variable(exp) = self {
                Ok(&exp)
            } else {
                Err(self.type_error("Variable"))
            }
        }

        fn position(&self) -> Position {
            match self {
                Exp::Block(b) => b.pos.clone(),
                Exp::Call(c) => c.pos.clone(),
                Exp::Literal(l) => l.pos.clone(),
                Exp::Assignment(a) => a.pos.clone(),
                Exp::Variable(v) => v.pos.clone(),
            }
        }

        fn name(&self) -> &'static str {
            match self {
                Exp::Block(_) => "Block",
                Exp::Call(_) => "Call",
                Exp::Literal(_) => "Literal",
                Exp::Assignment(_) => "Assignment",
                Exp::Variable(_) => "Variable",
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
    fn assignment() {
        let mut tokens = Tokens::new_test("test = 42");
        let exp = parse(&mut tokens).unwrap();
        let assignment = exp.assignment().unwrap();
        assert_eq!(assignment.name, "test");
        let value = assignment.value.literal().unwrap();
        assert_eq!(value.value, Value::Int(42));
    }

    #[test]
    fn variable() {
        let mut tokens = Tokens::new_test("test");
        let exp = parse(&mut tokens).unwrap();
        let variable = exp.variable().unwrap();
        assert_eq!(variable.name, "test");
    }

    #[test]
    fn in_valid() {
        let mut tokens = Tokens::new_test(",");
        parse(&mut tokens).unwrap_err();
    }
}
