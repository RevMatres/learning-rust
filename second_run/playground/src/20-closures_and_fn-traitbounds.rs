fn main() {

    // Fn, FnOnce, FnMut are the traits a closure or function-type can have
    // When working with generic types and you want to pass a closure as an argument, these are
    // your trait-bounds
    
    // A closure of Fn-Type (doesn't own or borrow any values from the scope)
    let a = || println!("I'm just an Fn-Type\n");
    a();

    // Another closure of Fn-Type (only borrows from the scope)
    let b = "wheeeee";
    let c = || println!("{}\n", b);
    c();

    // A closure of FnMut-Type (borrows mutably from the scope)
    let mut d = 92;
    let mut e = || {
        println!("d: {}", d);
        d += 1;
        println!("d: {}\n", d);
    };
    e();
    // Note: e holds a mutable borrow on d now, so only after e goes out of scope can other things
    // reference d again…

    // A closure of FnOnce-Type (takes ownership of a value from the scope)
    let f = vec![9, 9, 9];
    let g = move || println!("{:?}\n", f);    // the move keyword makes g take ownership of f, making g an FnOnce-Type
    g();
    // println!("{:?}", f);     // this call fails with a 'moved' error

    // Note: movement only happens with non-primitives, non-primitive are as always auto-cloned
    let h = 9;
    let i = move || println!("{}", h);
    i();
    println!("{}\n", h);          // therefore, this is perfectly valid!


    // Now for the use of the Fn-Traits

    // define a struct, that holds a Fn
    struct Container<T, U, V> 
        where T: Fn(i32),
              U: FnMut(i32),
              V: FnOnce(i32)
        {
            lambda: T,
            omega: U,
            theta: V,
    }

    let number1 = 3200;
    let mut number2 = 10;
    let number3 = vec![298];

    // create an instance
    let mut alpha = Container {
        lambda: |n| println!("{}", number1+n),
        omega: |_n| { number2 += 900; println!("{}", number2) },
        theta: move |_n| println!("{}", number3[0]),
    };

    // call the instance's closures
    (alpha.lambda)(8);  // alpha.lambda returns the closure, so (alpha.lambda)() calls the closure → alpha.lambda() would mean, lambda is a method
    println!("{}\n", number1);

    (alpha.omega)(0);
    // Just like with e: omega now holds a mutable reference, so no immutable accesses are allowed

    (alpha.theta)(0);

    /* println!("{}", number3[0]);  // this one again, can't be compiled, cause theta takes
     * ownership of number3
     *
     * Note: you have to explicitely tell the closure in theta in the assignment of alpha to 
     * take ownership of number3 using the move keyword, or the automatic type inference for
     * closures will kick in and only borrow the value → without the 'move', number3 is still
     * available in the outside scope!
     *
     * Note: the closure in alpha.theta also takes ownership of alpha.theta, due to the
     * move
     *
     */

}
