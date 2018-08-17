mod prelude;

use crate::parser::*;
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

type Result = std::result::Result<Type, Internal>;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match self {
            Type::Number(n) => n.to_string(),
            Type::Boolean(b) => b.to_string(),
            Type::String(s) => format!("\"{}\"", s.clone()),
            Type::Nil => "nil".to_string(),
        };
        write!(f, "{}", out)
    }
}

#[derive(Debug)]
enum Internal {
    Return(Type),
}

#[derive(Debug)]
#[allow(dead_code)]
crate enum Callable {
    Builtin(fn(Vec<Type>) -> Type),
    UserDefined, // TODO: impl
}

impl Callable {
    fn call(&self, args: Vec<Type>) -> Type {
        match self {
            Callable::Builtin(f) => f(args),
            Callable::UserDefined => unimplemented!(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Env {
    ctx_var: HashMap<String, Type>,
    ctx_fn: HashMap<String, Callable>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Rc<RefCell<Env>> {
        let env = Env {
            ctx_fn: prelude::init(),
            ..Env::default()
        };

        Rc::new(RefCell::from(env))
    }

    fn extend(env: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::from(Env {
            parent: Some(env),
            ..Default::default()
        }))
    }

    pub fn eval(env: Rc<RefCell<Env>>, ast: &AstNode) -> Type {
        Env::eval_internal(env, ast).unwrap()
    }

    fn eval_internal(env: Rc<RefCell<Env>>, ast: &AstNode) -> Result {
        match ast {
            AstNode::Nil => Ok(Type::Nil),
            AstNode::Number(n) => Ok(Type::Number(*n)),
            AstNode::Boolean(b) => Ok(Type::Boolean(*b)),
            AstNode::StringLiteral(s) => Ok(Type::String(s.clone())),
            AstNode::Grouping(expr) => Env::eval_internal(env, expr),

            AstNode::Identifier(id) => {
                if let Some(val) = env.borrow().ctx_var.get(id) {
                    return Ok(val.clone());
                }
                if let Some(ref p) = env.borrow().parent {
                    return Env::eval_internal(p.clone(), &AstNode::Identifier(id.clone()));
                }
                panic!("No such variable: {}", id);
            }

            AstNode::FnCall { identifier, args } => {
                if let Some(func) = env.borrow().ctx_fn.get(identifier) {
                    let args_evaled = args
                        .iter()
                        .map(|a| Env::eval_internal(env.clone(), a).unwrap())
                        .collect();
                    return Ok(func.call(args_evaled));
                }
                if let Some(ref p) = env.borrow().parent {
                    return Env::eval_internal(
                        p.clone(),
                        &AstNode::FnCall {
                            identifier: identifier.clone(),
                            args: args.clone(),
                        },
                    ); //FIXME: cloning ಠ_ಠ
                }
                Ok(Type::Nil)
            }

            AstNode::Program(stmts) => {
                let local = Env::extend(env);

                for s in stmts {
                    match s {
                        AstNode::RetStmt(expr) => {
                            return Err(Internal::Return(
                                Env::eval_internal(local.clone(), expr).unwrap(),
                            ))
                        }
                        _ => {
                            Env::eval_internal(local.clone(), s).unwrap();
                        }
                    }
                }

                Ok(Type::Nil)
            }

            AstNode::ProgramRoot(stmts) => {
                let mut ret = Type::Nil;

                for s in stmts {
                    match s {
                        AstNode::RetStmt(expr) => {
                            return Err(Internal::Return(
                                Env::eval_internal(env.clone(), expr).unwrap(),
                            ))
                        }
                        _ => {
                            ret = Env::eval_internal(env.clone(), s).unwrap();
                        }
                    }
                }

                Ok(ret)
            }

            AstNode::Assignment {
                identifier,
                operand,
                nonlocal,
            } => {
                let res = Env::eval_internal(env.clone(), operand).unwrap();
                if *nonlocal {
                    Env::update_value(env, identifier, res);
                } else {
                    env.borrow_mut().ctx_var.insert(identifier.clone(), res);
                }
                Ok(Type::Nil)
            }

            AstNode::IfStmt {
                condition,
                then_body,
                else_body,
            } => match Env::eval_internal(env.clone(), condition).unwrap() {
                Type::Boolean(true) => Env::eval_internal(env, then_body),
                Type::Boolean(false) => match else_body {
                    Some(prog) => Env::eval_internal(env, prog),
                    _ => Ok(Type::Nil),
                },
                _ => {
                    println!("An if check must be a boolean expression");
                    std::process::exit(5);
                }
            },

            AstNode::WhileStmt { condition, body } => {
                while let Type::Boolean(true) = Env::eval_internal(env.clone(), condition).unwrap()
                {
                    Env::eval_internal(env.clone(), body)?;
                }
                Ok(Type::Nil)
            }

            AstNode::FnStmt { .. } => Ok(Type::Nil), // temp

            AstNode::UnaryExpr { operator, operand } => Ok(match (
                operator,
                Env::eval_internal(env, operand).unwrap(),
            ) {
                (Op::Minus, Type::Number(n)) => Type::Number(-n),
                (Op::Bang, Type::Boolean(b)) => Type::Boolean(!b),
                _ => {
                    println!(
                        "Unary operator {:?} can not be applied to type: {:?}",
                        operator, operand
                    );
                    std::process::exit(2);
                }
            }),

            AstNode::BinaryExpr { operator, lhs, rhs } => Ok(match (
                operator,
                Env::eval_internal(env.clone(), lhs).unwrap(),
                Env::eval_internal(env, rhs).unwrap(),
            ) {
                (Op::EqualEqual, lhs, rhs) => Type::Boolean(lhs == rhs),
                (Op::BangEqual, lhs, rhs) => Type::Boolean(lhs != rhs),
                (Op::And, Type::Boolean(lhs), Type::Boolean(rhs)) => Type::Boolean(lhs && rhs),
                (Op::Or, Type::Boolean(lhs), Type::Boolean(rhs)) => Type::Boolean(lhs || rhs),
                (operator, Type::Number(lhsn), Type::Number(rhsn)) => match operator {
                    Op::Plus => Type::Number(lhsn + rhsn),
                    Op::Minus => Type::Number(lhsn - rhsn),
                    Op::Star => Type::Number(lhsn * rhsn),
                    Op::Slash => Type::Number(lhsn / rhsn),
                    Op::Percent => Type::Number(lhsn % rhsn),
                    Op::Greater => Type::Boolean(lhsn > rhsn),
                    Op::GreaterEqual => Type::Boolean(lhsn >= rhsn),
                    Op::Less => Type::Boolean(lhsn < rhsn),
                    Op::LessEqual => Type::Boolean(lhsn <= rhsn),
                    _ => {
                        println!(
                            "Operator {:?} can not be applied to types: {:?}, {:?}",
                            operator, lhs, rhs
                        );
                        std::process::exit(3);
                    }
                },
                (Op::Plus, Type::String(lhs), Type::String(rhs)) => Type::String(lhs + &rhs),
                _ => {
                    println!(
                        "Operator {:?} can not be applied to types: {:?}, {:?}",
                        operator, lhs, rhs
                    );
                    std::process::exit(3);
                }
            }),

            AstNode::RetStmt(_) => unreachable!(),
        }
    }

    fn update_value(env: Rc<RefCell<Env>>, id: &str, val: Type) {
        if let Some(v) = env.borrow_mut().ctx_var.get_mut(id) {
            *v = val;
            return;
        }

        let tmp = env.replace(Env::default());
        if let Some(p) = &tmp.parent {
            Env::update_value(p.clone(), id, val);
        }
        env.replace(tmp);
    }
}
