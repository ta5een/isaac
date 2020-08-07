mod token;

use tree::arena::Arena;
use tree::node::*;
use token::*;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Syntax {
    Root,
    Node(SyntaxNode),
    Token(SyntaxToken),
}

fn main() {
    // 1 + 2 * 1 + foo
    // ---------------
    //       +
    //      / \
    //     1   +
    //        / \
    //       *   foo
    //      / \
    //     2   1
    let mut arena = Arena::new();

    let raw_lit_one =
        Rc::new(RawSyntaxTokenData::new(SyntaxTokenKind::Literal, "1"));
    let raw_sym_plus =
        Rc::new(RawSyntaxTokenData::new(SyntaxTokenKind::Symbol, "+"));
    let raw_lit_two =
        Rc::new(RawSyntaxTokenData::new(SyntaxTokenKind::Literal, "2"));
    let raw_sym_asterisk =
        Rc::new(RawSyntaxTokenData::new(SyntaxTokenKind::Symbol, "*"));
    let raw_idt_foo =
        Rc::new(RawSyntaxTokenData::new(SyntaxTokenKind::Identifier, "foo"));

    let root =
        arena.insert(Syntax::Root);
    let binary_one_plus_expr =
        arena.insert(Syntax::Node(SyntaxNode::new(SyntaxNodeKind::BinaryExpression)));
    let binary_two_times_one =
        arena.insert(Syntax::Node(SyntaxNode::new(SyntaxNodeKind::BinaryExpression)));
    let binary_expr_plus_foo =
        arena.insert(Syntax::Node(SyntaxNode::new(SyntaxNodeKind::BinaryExpression)));

    let lit_one_1 =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_lit_one), 0, 1)));
    let sym_plus_1 =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_sym_plus), 2, 1)));
    let lit_two =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_lit_two), 4, 1)));
    let sym_asterisk =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_sym_asterisk), 6, 1)));
    let lit_one_2 =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_lit_one), 8, 1)));
    let sym_plus_2 =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_sym_plus), 10, 1)));
    let idt_foo =
        arena.insert(Syntax::Token(SyntaxToken::new(Rc::clone(&raw_idt_foo), 12, 3)));

    root.add_child(&mut arena, binary_one_plus_expr);

    binary_one_plus_expr
        .add_child(&mut arena, lit_one_1)
        .add_child(&mut arena, sym_plus_1)
        .add_child(&mut arena, binary_expr_plus_foo);

    binary_expr_plus_foo
        .add_child(&mut arena, binary_two_times_one)
        .add_child(&mut arena, sym_plus_2)
        .add_child(&mut arena, idt_foo);

    binary_two_times_one
        .add_child(&mut arena, lit_two)
        .add_child(&mut arena, sym_asterisk)
        .add_child(&mut arena, lit_one_2);

    println!("{:#?}", arena.nodes());
}
