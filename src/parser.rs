use crate::tokens::{TokenType, Tokens};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let mut tokens = Tokens::lex("42");
        let exp = parse(&mut tokens);
        assert_eq!(exp, Exp::LiteralUInt(42));
    }

    #[test]
    fn call() {
        let mut tokens = Tokens::lex("print(42)");
        let exp = parse(&mut tokens);
        assert_eq!(
            exp,
            Exp::Call("print".to_string(), vec![Exp::LiteralUInt(42)])
        );
    }

    #[test]
    fn call_two_args() {
        let mut tokens = Tokens::lex("print(1,2)");
        let exp = parse(&mut tokens);
        assert_eq!(
            exp,
            Exp::Call(
                "print".to_string(),
                vec![Exp::LiteralUInt(1), Exp::LiteralUInt(2)]
            )
        );
    }

    #[test]
    fn call_no_open() {
        let mut tokens = Tokens::lex("print42)");
        let exp = parse(&mut tokens);
        assert_eq!(exp, Exp::InValid);
    }

    #[test]
    fn call_no_end() {
        let mut tokens = Tokens::lex("print(42");
        let exp = parse(&mut tokens);
        assert_eq!(exp, Exp::InValid);
    }

    #[test]
    fn block() {
        let mut tokens = Tokens::lex("42;");
        let exp = parse_block(&mut tokens);
        assert_eq!(exp, Exp::Block(vec![Exp::LiteralUInt(42)]));
    }

    #[test]
    fn in_valid() {
        let mut tokens = Tokens::lex(",");
        let exp = parse(&mut tokens);
        assert_eq!(exp, Exp::InValid);
    }
}
