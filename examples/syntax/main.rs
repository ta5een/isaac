mod builder;
mod cache;
mod node;
mod token;

use builder::SyntaxBuilder;
use cavea::Arena;

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
    let mut builder = SyntaxBuilder::new(Arena::new());

    make!(builder => binary {
        make!(builder => binary {
            make!(builder => token { "a", 0 }),
            make!(builder => token { "*", 2 }),
            make!(builder => group {
                make!(builder => binary {
                    make!(builder => unary {
                        make!(builder => token { "-", 5 }),
                        make!(builder => token { "2", 6 }),
                    }),
                    make!(builder => token { "+", 8 }),
                    make!(builder => token { "a", 10 }),
                }),
            }),
        }),
        make!(builder => token { "-", 13 }),
        make!(builder => token { "10", 15 }),
    });

    println!("{:#?}", builder.nodes());
}
