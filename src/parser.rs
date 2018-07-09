use crate::lexer::*;

#[derive(Debug)]
pub enum AstNode {
    Number(f64),
    Boolean(bool),
    StringLiteral(String),
    Grouping(Box<AstNode>),
    UnaryExpr {
        operator: UnaryOp,
        operand: Box<AstNode>,
    },
    BinaryExpr {
        operator: BinaryOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Nil,
    Empty,
}

#[derive(Debug)]
pub enum UnaryOp {
    Not,
    Minus,
}

#[derive(Debug)]
pub enum BinaryOp {
    Multiply,
    Divide,
    Add,
    Subtract,
}

pub struct Parser<'source> {
    lexer: std::iter::Peekable<Lexer<'source>>,
}

impl Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Parser {
            lexer: Lexer::new(source).peekable(),
        }
    }

    pub fn parse(&mut self) -> AstNode {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> AstNode {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> AstNode {
        let lhs = self.parse_multiplication();
        if let Some(Token { kind, .. }) = self.lexer.peek().cloned() {
            return match kind {
                TokenKind::Plus => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: BinaryOp::Add,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_multiplication()),
                    }
                }
                TokenKind::Minus => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: BinaryOp::Subtract,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_multiplication()),
                    }
                }
                _ => lhs,
            };
        }
        lhs
    }

    fn parse_multiplication(&mut self) -> AstNode {
        let lhs = self.parse_unary();
        if let Some(Token { kind, .. }) = self.lexer.peek().cloned() {
            return match kind {
                TokenKind::Star => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: BinaryOp::Multiply,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_unary()),
                    }
                }
                TokenKind::Slash => {
                    self.lexer.next();
                    AstNode::BinaryExpr {
                        operator: BinaryOp::Divide,
                        lhs: Box::new(lhs),
                        rhs: Box::new(self.parse_unary()),
                    }
                }
                _ => lhs,
            };
        }
        lhs
    }

    fn parse_unary(&mut self) -> AstNode {
        if let Some(Token { kind, .. }) = self.lexer.peek().cloned() {
            return match kind {
                TokenKind::Bang => {
                    self.lexer.next();
                    AstNode::UnaryExpr {
                        operator: UnaryOp::Not,
                        operand: Box::new(self.parse_unary()),
                    }
                }
                TokenKind::Minus => {
                    self.lexer.next();
                    AstNode::UnaryExpr {
                        operator: UnaryOp::Minus,
                        operand: Box::new(self.parse_unary()),
                    }
                }
                _ => self.parse_primary(),
            };
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> AstNode {
        if let Some(Token { kind, .. }) = self.lexer.next() {
            return match kind {
                TokenKind::Number(n) => AstNode::Number(n),
                TokenKind::Boolean(b) => AstNode::Boolean(b),
                TokenKind::StringLiteral(s) => AstNode::StringLiteral(s),
                TokenKind::Nil => AstNode::Nil,
                TokenKind::OpenParen => {
                    let ret = self.parse_expression();
                    match self.lexer.next() {
                        Some(Token {
                            kind: TokenKind::CloseParen,
                            ..
                        }) => AstNode::Grouping(Box::new(ret)),
                        _ => panic!("Unclosed paren grouping"),
                    }
                }
                _ => AstNode::Empty,
            };
        }
        AstNode::Empty
    }
}
