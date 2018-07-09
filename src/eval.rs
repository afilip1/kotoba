use crate::parser::*;

#[derive(Debug)]
pub enum KotobaType {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

pub fn eval(ast: &AstNode) -> KotobaType {
    match ast {
        AstNode::Number(n) => KotobaType::Number(*n),
        AstNode::Boolean(b) => KotobaType::Boolean(*b),
        AstNode::StringLiteral(s) => KotobaType::String(s.clone()),
        AstNode::Nil => KotobaType::Nil,
        AstNode::Grouping(expr) => eval(expr),
        AstNode::UnaryExpr {
            operator: UnaryOp::Minus,
            operand,
        } => {
            if let KotobaType::Number(n) = eval(operand) {
                KotobaType::Number(-n)
            } else {
                println!("Unary operator can only be applied to numeric expressions.");
                std::process::exit(2);
            }
        }
        AstNode::BinaryExpr { operator, lhs, rhs } => {
            let (lhs, rhs) = (eval(lhs), eval(rhs));
            if let (KotobaType::Number(lhs), KotobaType::Number(rhs)) = (&lhs, &rhs) {
                match operator {
                    BinaryOp::Add => KotobaType::Number(lhs + rhs),
                    BinaryOp::Subtract => KotobaType::Number(lhs - rhs),
                    BinaryOp::Multiply => KotobaType::Number(lhs * rhs),
                    BinaryOp::Divide => KotobaType::Number(lhs / rhs),
                }
            } else if let (BinaryOp::Add, KotobaType::String(lhs), KotobaType::String(rhs)) =
                (operator, lhs, rhs)
            {
                KotobaType::String(lhs + &rhs)
            } else {
                println!("Binary operators can only be applied to numeric expressions.");
                std::process::exit(3);
            }
        }
        _ => KotobaType::Nil,
    }
}
