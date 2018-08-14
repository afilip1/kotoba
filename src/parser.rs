use crate::lexer::*;

type Result = std::result::Result<AstNode, Error>;

#[derive(Debug)]
enum Error {
    UnclosedGrouping(Token),
    UnexpectedToken(Token),
    UnexpectedEof,
    MissingColon(Token),
    MissingSemicolon(Token),
    FnCallMissingCloseParen(Token),
    MissingIdentifier(Token),
    MissingParen(Token),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    ProgramRoot(Vec<AstNode>),

    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    Identifier(String),
    Nil,

    Grouping(Box<AstNode>),

    FnCall {
        identifier: String,
        args: Vec<AstNode>,
    },
    RetStmt(Box<AstNode>),

    UnaryExpr {
        operator: Op,
        operand: Box<AstNode>,
    },
    BinaryExpr {
        operator: Op,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },

    Assignment {
        identifier: String,
        operand: Box<AstNode>,
        nonlocal: bool,
    },
    IfStmt {
        condition: Box<AstNode>,
        then_body: Box<AstNode>,
        else_body: Option<Box<AstNode>>,
    },
    WhileStmt {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    FnStmt {
        identifier: String,
        params: Vec<String>,
        body: Box<AstNode>,
    },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Bang,
    Star,
    Slash,
    Plus,
    Minus,
    Percent,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualEqual,
    BangEqual,
    And,
    Or,
}

impl From<&TokenKind> for Op {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::Bang => Op::Bang,
            TokenKind::Star => Op::Star,
            TokenKind::Slash => Op::Slash,
            TokenKind::Plus => Op::Plus,
            TokenKind::Minus => Op::Minus,
            TokenKind::Percent => Op::Percent,
            TokenKind::Greater => Op::Greater,
            TokenKind::GreaterEqual => Op::GreaterEqual,
            TokenKind::Less => Op::Less,
            TokenKind::LessEqual => Op::LessEqual,
            TokenKind::EqualEqual => Op::EqualEqual,
            TokenKind::BangEqual => Op::BangEqual,
            TokenKind::And => Op::And,
            TokenKind::Or => Op::Or,
            _ => unimplemented!(),
        }
    }
}

pub struct Parser<'source> {
    lexer: Lexer<'source>,
}

impl Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            lexer: Lexer::new(source),
        }
    }

    pub fn parse(&mut self) -> AstNode {
        //TODO: consume token stream
        match self.parse_program() {
            Ok(AstNode::Program(p)) => AstNode::ProgramRoot(p),
            Err(err) => {
                println!("syntax error: {:#?}", err);
                AstNode::Nil
            }
            _ => unreachable!(),
        }
    }

    fn parse_program(&mut self) -> Result {
        let mut stmts = vec![];

        while let Some(t) = self.lexer.peek() {
            //FIXME: not sure how to remove repetition
            match t.kind {
                TokenKind::Semicolon => break,
                TokenKind::If => {
                    self.lexer.next();
                    stmts.push(self.parse_if(t)?)
                }
                TokenKind::While => {
                    self.lexer.next();
                    stmts.push(self.parse_while(t)?)
                }
                TokenKind::Fn => {
                    self.lexer.next();
                    stmts.push(self.parse_fn(t)?)
                }
                TokenKind::Ret => {
                    self.lexer.next();
                    stmts.push(AstNode::RetStmt(Box::new(self.parse_expression()?)))
                }
                TokenKind::Nonlocal => {
                    self.lexer.next();
                    let ret = match self.parse_expression()? {
                        AstNode::Assignment {
                            identifier,
                            operand,
                            ..
                        } => AstNode::Assignment {
                            identifier,
                            operand,
                            nonlocal: true,
                        },
                        _ => panic!("not an assigment?"),
                    };

                    stmts.push(ret)
                }
                _ => {
                    stmts.push(self.parse_expression()?);
                    if self.lexer.expect(&TokenKind::Comma).is_none() {
                        break;
                    }
                }
            }
        }

        Ok(AstNode::Program(stmts))
    }

    fn parse_if(&mut self, t: Token) -> Result {
        let condition = self.parse_expression()?;

        if self.lexer.expect(&TokenKind::Colon).is_none() {
            return Err(Error::MissingColon(t));
        }

        let then_body = self.parse_program()?;

        let else_body = if self.lexer.expect(&TokenKind::Else).is_some() {
            Some(self.parse_program()?)
        } else {
            None
        };

        if self.lexer.expect(&TokenKind::Semicolon).is_none() {
            return Err(Error::MissingSemicolon(t));
        }

        Ok(AstNode::IfStmt {
            condition: Box::new(condition),
            then_body: Box::new(then_body),
            else_body: else_body.map(Box::new),
        })
    }

    fn parse_while(&mut self, t: Token) -> Result {
        let condition = self.parse_expression()?;

        if self.lexer.expect(&TokenKind::Colon).is_none() {
            return Err(Error::MissingColon(t));
        }

        let body = self.parse_program()?;

        if self.lexer.expect(&TokenKind::Semicolon).is_none() {
            return Err(Error::MissingSemicolon(t));
        }

        Ok(AstNode::WhileStmt {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn parse_fn(&mut self, t: Token) -> Result {
        let identifier = self
            .lexer
            .expect_identifier()
            .ok_or(Error::MissingIdentifier(t.clone()))?;

        if self.lexer.expect(&TokenKind::OpenParen).is_none() {
            return Err(Error::MissingParen(t));
        }

        let mut params = vec![];
        if let Some(p) = self.lexer.expect_identifier() {
            params.push(p);

            while self.lexer.expect(&TokenKind::Comma).is_some() {
                params.push(self.lexer.expect_identifier().unwrap());
            }
        }

        if self.lexer.expect(&TokenKind::CloseParen).is_none() {
            return Err(Error::MissingParen(t));
        }

        if self.lexer.expect(&TokenKind::Colon).is_none() {
            return Err(Error::MissingColon(t));
        }

        let body = self.parse_program()?;

        if self.lexer.expect(&TokenKind::Semicolon).is_none() {
            return Err(Error::MissingSemicolon(t));
        }

        Ok(AstNode::FnStmt {
            identifier,
            params,
            body: Box::new(body),
        })
    }

    fn parse_expression(&mut self) -> Result {
        self.parse_disjunction()
    }

    fn parse_disjunction(&mut self) -> Result {
        let mut acc = self.parse_conjunction()?;

        while self.lexer.expect(&TokenKind::Or).is_some() {
            acc = AstNode::BinaryExpr {
                operator: Op::Or,
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_conjunction()?),
            }
        }

        Ok(acc)
    }

    fn parse_conjunction(&mut self) -> Result {
        let mut acc = self.parse_equality()?;

        while self.lexer.expect(&TokenKind::And).is_some() {
            acc = AstNode::BinaryExpr {
                operator: Op::And,
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_equality()?),
            }
        }

        Ok(acc)
    }

    fn parse_equality(&mut self) -> Result {
        let lhs = self.parse_comparison()?;

        if let Some(t) = self
            .lexer
            .expect_any(&[TokenKind::EqualEqual, TokenKind::BangEqual])
        {
            return Ok(AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_comparison()?),
            });
        }

        Ok(lhs)
    }

    fn parse_comparison(&mut self) -> Result {
        let lhs = self.parse_modulo()?;

        if let Some(t) = self.lexer.expect_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            return Ok(AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_modulo()?),
            });
        }

        Ok(lhs)
    }

    fn parse_modulo(&mut self) -> Result {
        let mut acc = self.parse_addition()?;

        while let Some(t) = self.lexer.expect(&TokenKind::Percent) {
            acc = AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_addition()?),
            }
        }

        Ok(acc)
    }

    fn parse_addition(&mut self) -> Result {
        let mut acc = self.parse_multiplication()?;

        while let Some(t) = self.lexer.expect_any(&[TokenKind::Plus, TokenKind::Minus]) {
            acc = AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_multiplication()?),
            }
        }

        Ok(acc)
    }

    fn parse_multiplication(&mut self) -> Result {
        let mut acc = self.parse_unary()?;

        while let Some(t) = self.lexer.expect_any(&[TokenKind::Star, TokenKind::Slash]) {
            acc = AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_unary()?),
            }
        }

        Ok(acc)
    }

    fn parse_unary(&mut self) -> Result {
        if let Some(t) = self.lexer.expect_any(&[TokenKind::Bang, TokenKind::Minus]) {
            return Ok(AstNode::UnaryExpr {
                operator: (&t.kind).into(),
                operand: Box::new(self.parse_unary()?),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result {
        if let Some(t) = self.lexer.next() {
            match t.kind.clone() {
                TokenKind::Number(n) => Ok(AstNode::Number(n)),
                TokenKind::Boolean(b) => Ok(AstNode::Boolean(b)),
                TokenKind::StringLiteral(s) => Ok(AstNode::StringLiteral(s)),
                TokenKind::Identifier(identifier) => self.parse_identifier(identifier, t),
                TokenKind::Nil => Ok(AstNode::Nil),
                TokenKind::OpenParen => self.parse_grouping(t),
                _ => Err(Error::UnexpectedToken(t)),
            }
        } else {
            Err(Error::UnexpectedEof)
        }
    }

    fn parse_identifier(&mut self, identifier: String, t: Token) -> Result {
        if self.lexer.expect(&TokenKind::OpenParen).is_some() {
            // fn call
            let mut args = vec![];

            if self.lexer.expect(&TokenKind::CloseParen).is_some() {
                Ok(AstNode::FnCall { identifier, args })
            } else if self.lexer.peek().is_some() {
                if let Ok(arg) = self.parse_expression() {
                    args.push(arg);

                    while self.lexer.expect(&TokenKind::Comma).is_some() {
                        args.push(self.parse_expression()?);
                    }
                }

                if self.lexer.expect(&TokenKind::CloseParen).is_none() {
                    Err(Error::FnCallMissingCloseParen(t))
                } else {
                    Ok(AstNode::FnCall { identifier, args })
                }
            } else {
                Err(Error::FnCallMissingCloseParen(t))
            }
        } else if self.lexer.expect(&TokenKind::Equal).is_some() {
            // assignment
            Ok(AstNode::Assignment {
                identifier,
                operand: Box::new(self.parse_expression()?),
                nonlocal: false,
            })
        } else {
            // variable access
            Ok(AstNode::Identifier(identifier))
        }
    }

    fn parse_grouping(&mut self, t: Token) -> Result {
        let expr = self.parse_expression()?;

        if self.lexer.expect(&TokenKind::CloseParen).is_some() {
            Ok(AstNode::Grouping(Box::new(expr)))
        } else {
            Err(Error::UnclosedGrouping(t))
        }
    }
}
