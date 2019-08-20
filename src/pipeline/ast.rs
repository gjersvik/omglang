use crate::value::Value;
use crate::error::Position;

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
pub enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq)]
pub struct Operator {
    pub op_type: OpType,
    pub lhs: Box<Exp>,
    pub rhs: Box<Exp>,
    pub pos: Position,
}

#[derive(Debug, PartialEq)]
pub enum Exp {
    Block(Block),
    Call(Call),
    Literal(Literal),
    Assignment(Assignment),
    Variable(Variable),
    Operator(Operator),
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

    pub fn new_operator(op_type: OpType, lhs: Box<Exp>, rhs: Box<Exp>, pos: Position) -> Exp {
        Exp::Operator(Operator {
            op_type,
            lhs,
            rhs,
            pos,
        })
    }

    pub fn position(&self) -> Position {
        match self {
            Exp::Block(b) => b.pos.clone(),
            Exp::Call(c) => c.pos.clone(),
            Exp::Literal(l) => l.pos.clone(),
            Exp::Assignment(a) => a.pos.clone(),
            Exp::Variable(v) => v.pos.clone(),
            Exp::Operator(a) => a.pos.clone(),
        }
    }
}