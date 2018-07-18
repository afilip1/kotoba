use crate::lexer::*;

type Result = std::result::Result<AstNode, Error>;

#[derive(Debug)]
enum Error {
    UnclosedGrouping(Token),
    UnexpectedToken(Token),
    UnexpectedEof,
    AssignmentMissingEqual(Token),
    AssignmentMissingIdentifier(Token),
    IfMissingThen(Token),
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    Grouping(Box<AstNode>),
    Identifier(String),
    UnaryExpr {
        operator: Op,
        operand: Box<AstNode>,
    },
    BinaryExpr {
        operator: Op,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Program(Vec<AstNode>),
    Assignment {
        identifier: String,
        operand: Box<AstNode>,
    },
    If {
        check: Box<AstNode>,
        then: Box<AstNode>,
        otherwise: Option<Box<AstNode>>,
    },
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Bang,
    Star,
    Slash,
    Plus,
    Minus,
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
        match self.parse_program() {
            Ok(p) => p,
            err => {
                println!("syntax error: {:?}", err);
                AstNode::Nil
            }
        }
    }

    fn parse_program(&mut self) -> Result {
        let mut exprs = vec![];

        if self.lexer.peek().is_some() {
            exprs.push(self.parse_expression()?);
        }

        while let Some(t) = self.lexer.next() {
            match t.kind {
                TokenKind::Semicolon => {
                    exprs.push(self.parse_expression()?);
                }
                _ => return Err(Error::UnexpectedToken(t)),
            }
        }

        Ok(AstNode::Program(exprs))
    }

    fn parse_expression(&mut self) -> Result {
        if let Some(t) = self.lexer.expect(&TokenKind::Let) {
            self.parse_assignment(t)
        } else if let Some(t) = self.lexer.expect(&TokenKind::If) {
            self.parse_if(t)
        } else {
            self.parse_disjunction()
        }
    }

    fn parse_assignment(&mut self, t: Token) -> Result {
        if let Some(id) = self.lexer.expect_identifier() {
            if self.lexer.expect(&TokenKind::Equal).is_some() {
                Ok(AstNode::Assignment {
                    identifier: id,
                    operand: Box::new(self.parse_expression()?),
                })
            } else {
                Err(Error::AssignmentMissingEqual(t))
            }
        } else {
            Err(Error::AssignmentMissingIdentifier(t))
        }
    }

    fn parse_if(&mut self, t: Token) -> Result {
        let check = self.parse_expression()?;
        if self.lexer.expect(&TokenKind::Then).is_some() {
            let then = self.parse_expression()?;
            if self.lexer.expect(&TokenKind::Else).is_some() {
                let otherwise = self.parse_expression()?;
                Ok(AstNode::If {
                    check: Box::new(check),
                    then: Box::new(then),
                    otherwise: Some(Box::new(otherwise)),
                })
            } else {
                Ok(AstNode::If {
                    check: Box::new(check),
                    then: Box::new(then),
                    otherwise: None,
                })
            }
        } else {
            Err(Error::IfMissingThen(t))
        }
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
        let lhs = self.parse_addition()?;

        if let Some(t) = self.lexer.expect_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            return Ok(AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_addition()?),
            });
        }

        Ok(lhs)
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
            match t.kind {
                TokenKind::Number(n) => Ok(AstNode::Number(n)),
                TokenKind::Boolean(b) => Ok(AstNode::Boolean(b)),
                TokenKind::StringLiteral(s) => Ok(AstNode::StringLiteral(s)),
                TokenKind::Identifier(id) => Ok(AstNode::Identifier(id)),
                TokenKind::Nil => Ok(AstNode::Nil),
                TokenKind::OpenParen => {
                    let expr = self.parse_expression();

                    if self.lexer.expect(&TokenKind::CloseParen).is_some() {
                        Ok(AstNode::Grouping(Box::new(expr?)))
                    } else {
                        Err(Error::UnclosedGrouping(t))
                    }
                }
                _ => Err(Error::UnexpectedToken(t)),
            }
        } else {
            Err(Error::UnexpectedEof)
        }
    }
}
