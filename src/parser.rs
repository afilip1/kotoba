use crate::lexer::*;

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
    Nil,
    Empty,
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
        self.parse_program()
    }

    fn parse_program(&mut self) -> AstNode {
        let mut exprs = vec![self.parse_expression()];

        while let Some(t) = self.lexer.next() {
            match t.kind {
                TokenKind::Semicolon => {
                    let expr = self.parse_expression();
                    if expr == AstNode::Empty {
                        break;
                    }
                    exprs.push(expr);
                }
                _ => panic!("Unexpected token {:?} at {}", t.kind, t.position),
            }
        }

        AstNode::Program(exprs)
    }

    fn parse_expression(&mut self) -> AstNode {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> AstNode {
        if let Some(t) = self.lexer.expect(&TokenKind::Let) {
            if let Some(id) = self.lexer.expect_identifier() {
                if self.lexer.expect(&TokenKind::Equal).is_some() {
                    AstNode::Assignment {
                        identifier: id,
                        operand: Box::new(self.parse_expression()),
                    }
                } else {
                    panic!("Missing = in assignment at {}", t.position)
                }
            } else {
                panic!("Missing variable name in assignment at {}", t.position)
            }
        } else {
            self.parse_disjunction()
        }
    }

    fn parse_disjunction(&mut self) -> AstNode {
        let mut acc = self.parse_conjunction();

        while self.lexer.expect(&TokenKind::Or).is_some() {
            acc = AstNode::BinaryExpr {
                operator: Op::Or,
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_conjunction()),
            }
        }

        acc
    }

    fn parse_conjunction(&mut self) -> AstNode {
        let mut acc = self.parse_equality();

        while self.lexer.expect(&TokenKind::And).is_some() {
            acc = AstNode::BinaryExpr {
                operator: Op::And,
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_equality()),
            }
        }

        acc
    }

    fn parse_equality(&mut self) -> AstNode {
        let lhs = self.parse_comparison();

        if let Some(t) = self
            .lexer
            .expect_any(&[TokenKind::EqualEqual, TokenKind::BangEqual])
        {
            return AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_comparison()),
            };
        }

        lhs
    }

    fn parse_comparison(&mut self) -> AstNode {
        let lhs = self.parse_addition();

        if let Some(t) = self.lexer.expect_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            return AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(lhs),
                rhs: Box::new(self.parse_addition()),
            };
        }

        lhs
    }

    fn parse_addition(&mut self) -> AstNode {
        let mut acc = self.parse_multiplication();

        while let Some(t) = self.lexer.expect_any(&[TokenKind::Plus, TokenKind::Minus]) {
            acc = AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_multiplication()),
            }
        }

        acc
    }

    fn parse_multiplication(&mut self) -> AstNode {
        let mut acc = self.parse_unary();

        while let Some(t) = self.lexer.expect_any(&[TokenKind::Star, TokenKind::Slash]) {
            acc = AstNode::BinaryExpr {
                operator: (&t.kind).into(),
                lhs: Box::new(acc),
                rhs: Box::new(self.parse_unary()),
            }
        }

        acc
    }

    fn parse_unary(&mut self) -> AstNode {
        if let Some(t) = self.lexer.expect_any(&[TokenKind::Bang, TokenKind::Minus]) {
            return AstNode::UnaryExpr {
                operator: (&t.kind).into(),
                operand: Box::new(self.parse_unary()),
            };
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> AstNode {
        if let Some(t) = self.lexer.next() {
            match t.kind {
                TokenKind::Number(n) => AstNode::Number(n),
                TokenKind::Boolean(b) => AstNode::Boolean(b),
                TokenKind::StringLiteral(s) => AstNode::StringLiteral(s),
                TokenKind::Identifier(id) => AstNode::Identifier(id),
                TokenKind::Nil => AstNode::Nil,
                TokenKind::OpenParen => {
                    let expr = self.parse_expression();

                    if self.lexer.expect(&TokenKind::CloseParen).is_some() {
                        return AstNode::Grouping(Box::new(expr));
                    }

                    panic!("Unclosed paren grouping at {}", t.position)
                }
                _ => panic!("Unexpected token {:?} at {}", t.kind, t.position),
            }
        } else {
            AstNode::Empty
        }
    }
}
