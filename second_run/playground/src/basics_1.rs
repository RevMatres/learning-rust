fn main() {

/*
   This contains:
   Primitives and mutability
   Constants
   Shadowing
   Control Flow
   Iterators
   Ownership
   Pointers/Borrows
   Slices
*/


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
    /* collection types are iterable, if they implement the Iterator-trait

       the for-loop always uses an iterator, but unlike the .iter() methods,
       which can be used with e.g. .for_each() the for-loop uses an iterator
       of references to the collections' values, while the .iter() path uses
       the values directly

       this is stuff, you'll pick up over time…
    */


    // OWNERSHIP STUFF

    // MOVE
    // With non-primitive types you have the type's data [pointer, size, capacity, etc]
    // on the stack and the actual information on the heap.
    
    let s1 = String::from("wheeeee");	// create a String-type with s1 as its owner
    let s2 = s1;			// create another variable that points to the heap

    /* In other languages this is called a shallow copy, because in them only the type's data
       is copied, which is why you end up with more than one pointer to the same place on the heap.
       In rust this doesn't happen, instead the type's data is MOVED, not SHALLOW COPIED.
       That means s1 is invalidated after s2 is assigned the type's data.
       That way you never have more than one OWNER and more than one data-set pointing to the same
       spot on the heap.
       A DEEP COPY would be a CLONE, where both the data and the heap-stuff is duplicated, creating
       a functional clone of the initial object.
    */

   //  OWNERSHIP AND BORROWS AND POINTERS
   /*  In rust a variable has only one owner.
       You can borrow the variable to a different scope via a share-pointer [&var],
       in which case both the owner and the other scope can read the variable, but neither
       can edit/mutate it.
       You can offer up one mutable borrow to a scope via a mutable-share-pointer [&mut var],
       in which case the owner can't edit the variable anymore, and only the owner of the
       mut-borrow can.
       This basically means only one scope at a time can edit any one value.
       Variables are dropped, when they go out of scope. If they are borrowed to a scope, the
       used pointer can go out of scope and is dropped, without the variable being dropped.
    */

    // SLICE TYPE
    /* Slices are references/borrows of parts of collection types like the String or the Array.
       They are basically a direct pointer to a heap location, that already is part of a collection type.
       They are share borrows, so they are immutable!
    */

    let e = [1, 2, 3, 4, 5];
    
    let e_slice_1 = &e[1..3];	// range 1..3

    let e_slice_2 = &e[..3];	// range 0..3
    
    let e_slice_3 = &e[2..];	// range 2..e.len()

    println!("{:?}",e_slice_1);
    println!("{:?}",e_slice_2);
    println!("{:?}",e_slice_3);

}
