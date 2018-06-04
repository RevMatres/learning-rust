fn main() {

    // MUTABILITY
    // variables need to be explicitely mutable, if they are supposed to change
    let mut a = 5u32;
    println!("{}",a);
    a = 77;
    println!("{}",a);
    
    // CONSTANTS
    // constants are always unmutable, which makes them inheritable across scopes
    // also they are primitives, so inheritance
    const CONSTANT: u32 = 100_000;
    {
	    println!("{}",CONSTANT)
    }

    // SHADOWING... or rather saying +=, etc.
    //
    // with mutable variables:
    let mut b = 5;
    b = b*2;
    println!("{}",b);

    // with immutable variables:
    let c = 5;
    let c = c + 12;
    println!("{}",c);
    // -> note: this is one form of breaking immutability
    //    with a constant this would be impossible

    // use-case example:
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}",spaces);
    // you don't want the actual string, you just want its length
    // this way you don't have to use two variable names, can be more elegant/siccinct

    // CONTROL FLOW
    // if statemtents
    let d = 17;
    if d < 20 {
        println!("if it's smaller than 20: {}",d);
    } else if d == 20 {
        println!("it's 20");
    } else {
        println!("it's bigger than 20");
    }
    // point here is: you don't need parenthesis around the expressions

    // the same goes for while, for and loop-loops!

    // ITERATIONS
    // collection types are iterable, if they implement the Iterator-trait

    // the for-loop always uses an iterator, but unlike the .iter() methods,
    // which can be used with e.g. .for_each() the for-loop uses an iterator
    // of references to the collections' values, while the .iter() path uses
    // the values directly

    // this is stuff, you'll pick up over time…
}
