use crate::lexer::*;

enum AstNode {
    Empty,
}

struct Parser<'a> {
    lexer: &'a Lexer<'a>,
}

impl Parser<'a> {
    fn new(lexer: &'a Lexer) -> Self {
        Parser { lexer }
    }

    fn parse() -> AstNode {
        AstNode::Empty
    }
}
