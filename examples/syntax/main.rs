mod builder;
mod cache;
mod node;
mod token;

use builder::SyntaxBuilder;
use isaac::Arena;

fn main() {
    // a * (-2 + a) - 10
    // -----------------
    //         -
    //        / \
    //       *   10
    //      / \
    //     a   +
    //        / \
    //       -   a
    //       |
    //       2
    let mut b = SyntaxBuilder::new(Arena::new());

    make!(b => binary {
        make!(b => binary {
            make!(b => token { "a", 0 }),
            make!(b => token { "*", 2 }),
            make!(b => group {
                make!(b => binary {
                    make!(b => unary {
                        make!(b => token { "-", 5 }),
                        make!(b => token { "2", 6 }),
                    }),
                    make!(b => token { "+", 8 }),
                    make!(b => token { "a", 10 }),
                }),
            }),
        }),
        make!(b => token { "-", 13 }),
        make!(b => token { "10", 15 }),
    });

    println!("{:#?}", b.nodes());
}
