# `isaac`

An <ins>**i**</ins>ndubitably <ins>**s**</ins>imple <ins>**a**</ins>llocation
<ins>**a**</ins>rena <ins>**c**</ins>rate.

## Example

```rust
use cavea::Arena;

// We want to make a simple graph structure like this:
//
//                       1
//                      / \
//                     2   3
//                         |
//                         4
//
// The root node (1) will have two children (2 and 3),
// and node 3 will have one child (4).

// Initialising a new arena.
let arena = &mut Arena::new();

// Inserting new values into the arena.
let str_1 = arena.insert("1");
let str_2 = arena.insert("2");
let str_3 = arena.insert("3");
let str_4 = arena.insert("4");

// `str_1` is the parent of `str_2` and `str_3`.
str_1
    .add_child(arena, str_2)
    .add_child(arena, str_3);

// `str_3` is the parent of `str_4`.
str_3
    .add_child(arena, str_4);

// Asserting parent relationships
assert_eq!(str_1.parent(arena), None);
assert_eq!(str_2.parent(arena), Some(str_1));
assert_eq!(str_3.parent(arena), Some(str_1));
assert_eq!(str_4.parent(arena), Some(str_3));

// Asserting children relationships
assert_eq!(str_1.children(arena), &vec![str_2, str_3]);
assert_eq!(str_2.children(arena), &vec![]);
assert_eq!(str_3.children(arena), &vec![str_4]);
assert_eq!(str_4.children(arena), &vec![]);
```
