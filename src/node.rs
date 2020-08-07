#![allow(dead_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxNode {
    kind: SyntaxNodeKind,
}

impl SyntaxNode {
    pub fn new(kind: SyntaxNodeKind) -> Self {
        Self { kind }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyntaxNodeKind {
    BinaryExpression,
    LiteralExpression,
}
