
// this is for TYPE ALIASES
type Inch = u64;


fn main(){
    // in 1-custom_types.rs let vs const and mutability is illustrated

    // by default unused variables push an error, to fix that:
    let _unused_variable = "Duh.";

    // copy variables
    let variable_1 = 32i32;
    println!("variable_1 {}",variable_1);
    let variable_2 = variable_1;
    println!("variable_2 {}",variable_2);

    let _variable_3 = "I'm immutable, I can't be changed unless redefined";
    let mut _variable_4 = "I'm mutable!";

    // variables respect scoping
    {       // note: this is a block...
        let variable_1 = 28937492i32;
        println!("variable_1 {} inner scope",variable_1);
    }
    println!("variable_1 {} outer scope",variable_1);

    // variables can be declared first, then initialized
    let variable_5;

    variable_5 = 42u8;
    println!("{}",variable_5);


    // casting
    let decimal = 93847.9327984f64;
    println!("{}",decimal);
    let integer = decimal as u64;
    println!("{}",integer);


    // TYPE ALIASES
    // one can define custom aliases for types
    let size: Inch = 32;
    println!("size: {} inches",size);
}
