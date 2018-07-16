use crate::parser::*;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Type {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match self {
                Type::Number(n) => n.to_string(),
                Type::Boolean(b) => b.to_string(),
                Type::String(s) => format!("\"{}\"", s.clone()),
                Type::Nil => "nil".to_string(),
            }
        )
    }
}

pub fn eval(ast: &AstNode) -> Type {
    match ast {
        AstNode::Nil => Type::Nil,
        AstNode::Grouping(expr) => eval(expr),
        AstNode::Number(n) => Type::Number(*n),
        AstNode::Boolean(b) => Type::Boolean(*b),
        AstNode::StringLiteral(s) => Type::String(s.clone()),
        AstNode::UnaryExpr { operator, operand } => match (operator, eval(operand)) {
            (Op::Minus, Type::Number(n)) => Type::Number(-n),
            (Op::Bang, Type::Boolean(b)) => Type::Boolean(!b),
            _ => {
                println!(
                    "Unary operator {:?} can not be applied to type: {:?}",
                    operator, operand
                );
                std::process::exit(2);
            }
        },
        AstNode::BinaryExpr { operator, lhs, rhs } => match (operator, eval(lhs), eval(rhs)) {
            (Op::EqualEqual, lhs, rhs) => Type::Boolean(lhs == rhs),
            (Op::BangEqual, lhs, rhs) => Type::Boolean(lhs != rhs),
            (operator, Type::Number(lhs), Type::Number(rhs)) => match operator {
                Op::Plus => Type::Number(lhs + rhs),
                Op::Minus => Type::Number(lhs - rhs),
                Op::Star => Type::Number(lhs * rhs),
                Op::Slash => Type::Number(lhs / rhs),
                Op::Greater => Type::Boolean(lhs > rhs),
                Op::GreaterEqual => Type::Boolean(lhs >= rhs),
                Op::Less => Type::Boolean(lhs < rhs),
                Op::LessEqual => Type::Boolean(lhs <= rhs),
                _ => unreachable!(),
            },
            (Op::Plus, Type::String(lhs), Type::String(rhs)) => Type::String(lhs + &rhs),
            _ => {
                println!(
                    "Operator {:?} can not be applied to types: {:?}, {:?}",
                    operator, lhs, rhs
                );
                std::process::exit(3);
            }
        },
        _ => Type::Nil, //temporary
    }
}
