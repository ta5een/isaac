#![allow(dead_code)]

use crate::token::*;
use crate::node::*;
use crate::cache::Cache;
use cavea::{Arena, Node, NodeId};
use std::rc::Rc;

#[macro_export]
macro_rules! make {
    ($builder:expr => token { $text:expr, $start:expr $(,)? }) => {{
        $builder.make_token($text, $start)
    }};
    ($builder:expr => unary { $operator:expr, $operand:expr $(,)? }) => {{
        let operator = $operator;
        let operand = $operand;
        $builder.make_unary_expr(operator, operand)
    }};
    ($builder:expr => binary { $lhs:expr, $operator:expr, $rhs:expr $(,)? }) => {{
        let lhs = $lhs;
        let operator = $operator;
        let rhs = $rhs;
        $builder.make_binary_expr(lhs, operator, rhs)
    }};
    ($builder:expr => group { $inner_expr:expr $(,)? }) => {{
        let inner_expr = $inner_expr;
        $builder.make_group_expr(inner_expr)
    }};
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Syntax {
    Node(SyntaxNode),
    Token(SyntaxToken),
}

pub struct SyntaxBuilder {
    arena: Arena<Syntax>,
    raw_token_cache: Cache<String, Rc<RawSyntaxTokenData>>,
}

impl SyntaxBuilder {
    pub fn new(arena: Arena<Syntax>) -> Self {
        Self { arena, raw_token_cache: Cache::new() }
    }

    pub fn node_at(&self, id: NodeId) -> Option<&Node<Syntax>> {
        self.arena.node_at(id)
    }

    pub fn nodes(&self) -> Vec<Node<Syntax>> {
        self.arena.nodes()
    }

    pub fn make_token(&mut self, text: &str, start: usize) -> NodeId {
        let raw = self.raw_token_cache.lookup(text.into(), Rc::new(lex(text)));
        let token = SyntaxToken::new(Rc::clone(&raw), start, text.len());
        self.arena.insert(Syntax::Token(token))
    }

    pub fn make_unary_expr(&mut self,
                           operator: NodeId,
                           operand: NodeId) -> NodeId
    {
        let node = SyntaxNode::new(SyntaxNodeKind::Expr(Expr::Unary));
        let unary_expr = self.arena.insert(Syntax::Node(node));

        *unary_expr
            .add_child(&mut self.arena, operator)
            .add_child(&mut self.arena, operand)
    }

    pub fn make_binary_expr(&mut self,
                            lhs: NodeId,
                            operator: NodeId,
                            rhs: NodeId) -> NodeId
    {
        let node = SyntaxNode::new(SyntaxNodeKind::Expr(Expr::Binary));
        let binary_expr = self.arena.insert(Syntax::Node(node));

        *binary_expr
            .add_child(&mut self.arena, lhs)
            .add_child(&mut self.arena, operator)
            .add_child(&mut self.arena, rhs)
    }

    pub fn make_group_expr(&mut self,
                           inner_expr: NodeId) -> NodeId
    {
        let node = SyntaxNode::new(SyntaxNodeKind::Expr(Expr::Group));
        let group_expr = self.arena.insert(Syntax::Node(node));

        *group_expr
            .add_child(&mut self.arena, inner_expr)
    }
}

fn lex(s: &str) -> RawSyntaxTokenData {
    use RawSyntaxTokenData as RSTData;
    match s {
        "*" => RSTData::new(SyntaxTokenKind::Symbol(Symbol::Asterisk), s),
        "+" => RSTData::new(SyntaxTokenKind::Symbol(Symbol::Plus), s),
        "-" => RSTData::new(SyntaxTokenKind::Symbol(Symbol::Minus), s),
        "/" => RSTData::new(SyntaxTokenKind::Symbol(Symbol::ForwardSlash), s),
        "(" => RSTData::new(SyntaxTokenKind::Symbol(Symbol::LParen), s),
        ")" => RSTData::new(SyntaxTokenKind::Symbol(Symbol::RParen), s),
        s if s.chars().all(|c| c.is_numeric()) => {
            RSTData::new(SyntaxTokenKind::Literal(Literal::Integer), s)
        },
        s => RSTData::new(SyntaxTokenKind::Identifier, s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_builder() {
        let mut builder = SyntaxBuilder::new(Arena::new());

        let token_lit_one_1 = builder.make_token("1", 0);
        let token_sym_pls_1 = builder.make_token("+", 2);
        let token_sym_mns_1 = builder.make_token("-", 4);
        let token_lit_one_2 = builder.make_token("1", 5);

        let unary_expr =
            builder.make_unary_expr(
                token_sym_mns_1,
                token_lit_one_2,
            );

        let _binary_expr =
            builder.make_binary_expr(
                token_lit_one_1,
                token_sym_pls_1,
                unary_expr,
            );

        assert_eq!(builder.nodes(), {
            let arena = &mut Arena::new();

            let raw_token_lit_one =
                Rc::new(RawSyntaxTokenData::new(
                    SyntaxTokenKind::Literal(Literal::Integer), "1"
                ));
            let raw_token_sym_pls =
                Rc::new(RawSyntaxTokenData::new(
                    SyntaxTokenKind::Symbol(Symbol::Plus), "+"
                ));
            let raw_token_sym_mns =
                Rc::new(RawSyntaxTokenData::new(
                    SyntaxTokenKind::Symbol(Symbol::Minus), "-"
                ));

            let token_lit_one_1 =
                arena.insert(Syntax::Token(
                    SyntaxToken::new(Rc::clone(&raw_token_lit_one), 0, 1)
                ));
            let token_sym_pls_1 =
                arena.insert(Syntax::Token(
                    SyntaxToken::new(Rc::clone(&raw_token_sym_pls), 2, 1)
                ));
            let token_sym_mns_1 =
                arena.insert(Syntax::Token(
                    SyntaxToken::new(Rc::clone(&raw_token_sym_mns), 4, 1)
                ));
            let token_lit_one_2 =
                arena.insert(Syntax::Token(
                    SyntaxToken::new(Rc::clone(&raw_token_lit_one), 5, 1)
                ));

            let node_binary_1 =
                arena.insert(Syntax::Node(
                    SyntaxNode::new(SyntaxNodeKind::Expr(Expr::Binary))
                ));
            let node_unary_1  =
                arena.insert(Syntax::Node(
                    SyntaxNode::new(SyntaxNodeKind::Expr(Expr::Unary))
                ));

            node_unary_1
                .add_child(arena, token_sym_mns_1)
                .add_child(arena, token_lit_one_2);

            node_binary_1
                .add_child(arena, token_lit_one_1)
                .add_child(arena, token_sym_pls_1)
                .add_child(arena, node_unary_1);

            builder.nodes()
        });
    }

    #[test]
    fn test_syntax_builder_make_macro() {
        // 1 + -1
        let mut builder = SyntaxBuilder::new(Arena::new());

        make!(builder => binary {
            make!(builder => token { "1", 0 }),
            make!(builder => token { "+", 2 }),
            make!(builder => unary {
                make!(builder => token { "-", 4 }),
                make!(builder => token { "1", 5 }),
            }),
        });

        assert_eq!(builder.nodes(), {
            let mut builder = SyntaxBuilder::new(Arena::new());

            let token_lit_one_1 = builder.make_token("1", 0);
            let token_sym_pls_1 = builder.make_token("+", 2);
            let token_sym_mns_1 = builder.make_token("-", 4);
            let token_lit_one_2 = builder.make_token("1", 5);

            let unary_expr =
                builder.make_unary_expr(
                    token_sym_mns_1,
                    token_lit_one_2
                );

            let _binary_expr =
                builder.make_binary_expr(
                    token_lit_one_1,
                    token_sym_pls_1,
                    unary_expr
                );

            builder.nodes()
        });
    }
}
