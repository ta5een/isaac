# `cavea`

A simple allocation arena.

## Example

```rust
use cavea::Arena;

// Initialising a new arena.
let arena = &mut Arena::new();

// Inserting new values into the arena.
let str_one   = arena.insert("1");
let str_two   = arena.insert("2");
let str_three = arena.insert("3");
let str_four  = arena.insert("4");

// `str_one` is the parent of `str_two` and `str_three`.
str_one
    .add_child(arena, str_two)
    .add_child(arena, str_three);

// `str_three` is the parent of `str_four`.
str_three
    .add_child(arena, str_four);

assert_eq!(*arena, Arena {
    nodes: vec![
        Node {
            id: NodeId(0),
            data: "1",
            parent: None,
            children: vec![NodeId(1), NodeId(2)],
        },
        Node {
            id: NodeId(1),
            data: "2",
            parent: Some(NodeId(0)),
            children: vec![],
        },
        Node {
            id: NodeId(2),
            data: "3",
            parent: Some(NodeId(0)),
            children: vec![NodeId(3)],
        },
        Node {
            id: NodeId(3),
            data: "4",
            parent: Some(NodeId(2)),
            children: vec![],
        },
    ],
    root: Some(NodeId(0)),
});
```
