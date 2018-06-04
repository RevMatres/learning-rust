
// regular comment

/*
    Block comment
*/



fn main() {
    println!("Hello World!");

    println!("");

    let answer = 42;
    println!("The Answer to Life, the Universe and Everything is {}", answer);

    println!("");
    println!("Let's do some primitive types, shall we?");
    println!("");

    // signed integers
    let a: i8 = 127;        // 127 is largest in i8 because there is all the space for the negatives
    println!(" {}",a);
    let b = -127i8;         // suffix annotation
    println!("{}",b);

    // unsigned integers
    let c: u8 = 255;        // 255 is largest in u8 because 8-bits without the negatives
    println!(" {}",c);

    // floats
    let d: f32 = 324.6248957;   // println! by default only prints 4 digits after comma
    println!(" {}",d);

    // booleans
    let e: bool = true;
    println!("{}",e);

    // characters and strings
    let f = "lsjflksadjf;ljdf;alsdjfawjfeohkjsfhalsjf TEEEEEEEXT";
    println!("{}",f);
    let e: char = 'a';
    println!("{}",e);

    // arrays are done using vectors
    // tuples are standard objects
    // I'll put both of these to custom types, cause that feels better
}
