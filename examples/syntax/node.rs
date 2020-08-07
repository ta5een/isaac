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
    Expr(Expr),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Binary,
    Unary,
}

#[test]
fn test_syntax_node() {
    println!("{}", std::mem::size_of::<SyntaxNode>());
    println!("{}", std::mem::size_of::<SyntaxNodeKind>());
}
