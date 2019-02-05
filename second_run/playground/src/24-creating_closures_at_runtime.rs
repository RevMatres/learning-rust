fn main() {

    /*
     *
     * YOU CAN DYNAMICALLY CREATE FUNCTIONS AND STORE THEM IN A LIST.
     * This file contains an example of this.
     *
     * This has some huge implications/ramifications:
     * YOU CAN CREATE APPLICATIVE FUNCTORS AND GENERATE FUNCTIONS AT RUNTIME IN RUST.
     * Basically, you can do all the cool shite, a functional language allows you to
     * do, with the one caviat, that you're probably not getting around using state,
     * somewhere.
     *
     */

    // NOTE The rust reference says, that the type of a closure expression can't be written out.
    // I'm guessing that's because its size can't be known at compile time, but you very much
    // can use them like I'm doing in this program.
    // Takeaway: No type annotations for closures, let the compiler infer!

    let mut v = Vec::new();

    for i in 0..10 {

        let c = move |n:i32| -> i32 { i*n };

        v.push(c);

    }


    v.iter().for_each(|closure| {

        println!("{}", closure(42));

    })

}
