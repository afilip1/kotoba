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
        operand: Box<AstNode>
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

pub struct Parser<'source> {
    lexer: Lexer<'source>,
}

impl Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            lexer: Lexer::new(source)
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
        if let Some(t @ Token { kind: TokenKind::Let, .. }) = self.lexer.peek() {
            self.lexer.next();
            if let Some(Token { kind:TokenKind::Identifier(id), .. }) = self.lexer.next() {
                if let Some(Token  { kind:TokenKind::Equal,.. }) = self.lexer.next() {
                    AstNode::Assignment {
                        identifier: id,
                        operand: Box::new(self.parse_expression())
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
        let lhs = self.parse_conjunction();
        if let Some(t) = self.lexer.peek() {
            match t.kind {
                TokenKind::Or => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Or,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_conjunction()),
                    }
                }
                _ => lhs,
            }
        } else {
            lhs
        }
    }

    fn parse_conjunction(&mut self) -> AstNode {
        let lhs = self.parse_equality();
        if let Some(t) = self.lexer.peek() {
            match t.kind {
                TokenKind::And => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::And,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_equality()),
                    }
                }
                _ => lhs,
            }
        } else {
            lhs
        }
    }

    fn parse_equality(&mut self) -> AstNode {
        let lhs = self.parse_comparison();
        if let Some(t) = self.lexer.peek() {
            match t.kind {
                TokenKind::EqualEqual => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::EqualEqual,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_comparison()),
                    }
                }
                TokenKind::BangEqual => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::BangEqual,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_comparison()),
                    }
                }
                _ => lhs,
            }
        } else {
            lhs
        }
    }

    fn parse_comparison(&mut self) -> AstNode {
        let lhs = self.parse_addition();
        if let Some(t) = self.lexer.peek() {
            match t.kind {
                TokenKind::Greater => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Greater,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_addition()),
                    }
                }
                TokenKind::GreaterEqual => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::GreaterEqual,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_addition()),
                    }
                }
                TokenKind::Less => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Less,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_addition()),
                    }
                }
                TokenKind::LessEqual => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::LessEqual,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_addition()),
                    }
                }
                _ => lhs,
            }
        } else {
            lhs
        }
    }

    fn parse_addition(&mut self) -> AstNode {
        let mut acc = self.parse_multiplication();
        while let Some(t) = self.lexer.peek() {
            acc = match t.kind {
                TokenKind::Plus => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Plus,
                        lhs: Box::new(acc),
                        rhs: Box::new(self.parse_multiplication()),
                    }
                }
                TokenKind::Minus => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Minus,
                        lhs: Box::new(acc),
                        rhs: Box::new(self.parse_multiplication()),
                    }
                }
                _ => break,
            };
        }
        acc
    }

    fn parse_multiplication(&mut self) -> AstNode {
        let mut acc = self.parse_unary();
        while let Some(t) = self.lexer.peek() {
            acc = match t.kind {
                TokenKind::Star => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Star,
                        lhs: Box::new(acc),
                        rhs: Box::new(self.parse_unary()),
                    }
                }
                TokenKind::Slash => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: Op::Slash,
                        lhs: Box::new(acc),
                        rhs: Box::new(self.parse_unary()),
                    }
                }
                _ => break,
            };
        }
        acc
    }

    fn parse_unary(&mut self) -> AstNode {
        if let Some(t) = self.lexer.peek() {
            match t.kind {
                TokenKind::Bang => {
                    self.lexer.next();
                    AstNode::UnaryExpr {
                        operator: Op::Bang,
                        operand: Box::new(self.parse_unary()),
                    }
                }
                TokenKind::Minus => {
                    self.lexer.next();
                    AstNode::UnaryExpr {
                        operator: Op::Minus,
                        operand: Box::new(self.parse_unary()),
                    }
                }
                _ => self.parse_primary(),
            }
        } else {
            AstNode::Empty
        }
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
                    match self.lexer.next() {
                        Some(Token {
                            kind: TokenKind::CloseParen,
                            ..
                        }) => AstNode::Grouping(Box::new(expr)),
                        _ => panic!("Unclosed paren grouping at {}", t.position),
                    }
                }
                _ => panic!("Unexpected token {:?} at {}", t.kind, t.position),
            }
        } else {
            AstNode::Empty
        }
    }
}
