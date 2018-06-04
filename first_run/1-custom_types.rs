// got some unused things for showcase
#![allow(dead_code)]

// this is for enums
#[derive(Debug)]
enum Enumj {
    Cat,
    Duck,
    Chicken
}


// this is for const
const CONSTANT_VARIABLE: i32 = 49237589;


fn main() {

    // Tuples
    // collection of FIXED SIZE of values of DIFFERENT types
    let tuple_1 = (123u8, 255u8, -20i8, "dabaDEEEE");
    println!("{:?}",tuple_1);           // {:?} is the DEBUG notation
    println!("{}",tuple_1.1);

    // multi-level tuple
    let tuple_2 = ((934857u32, 234.5655f32),("aaasldkfj","lsadfj"));
    println!("{:?}",tuple_2);
    println!("{:?}",tuple_2.1);

    println!("{}",(tuple_2.1).0);       // multi-level tuple deconstruction: first get the index-1-value of tuple_2, then get the index-0-value of tuple_2.1

    // binding for deconstruction
    let tuple_3 = (1,2,3,4,5);
    let (val1, val2, val3, val4, val5) = tuple_3;       // 5 vars are created, (val1 val2 etc), those are BOUND to the corresponding values in tuple_3
    println!("{:?} {:?} {:?} {:?} {:?}",val1, val2, val3, val4, val5);


    // Arrays
    // collection of FIXED SIZE of values of SAME types
    let array_1: [i32;5] = [100,200,300,400,42];
    println!("{:?}",array_1);

    // initialize to the same value
    let array_2: [i32; 100] = [42; 100];        // [type; array_length] = [value; array_length]
    println!("{}",array_2[32]);
    println!("{}",array_2.len());       // get the length of the array

    // Slices
    // borrows parts of an array, length isn't known at compile time
    let slice_1: &[i32] = &array_1[1..4];       // slice_1 is an array of i32 and borrows the indices 1 through 4 from pointer &array_1
    println!("{:?}",slice_1);


    // Vecs
    // Vectors are Stacks or resizable arrays


    // Structs
    // structures are custom types that can contain all sorts of things

    // tuple-struct
    // this is a tuple-constructor with a name, it ISN'T a TYPE
    #[derive(Debug)]
    struct CustomTuple(i32,i32);

    let struct_1 = CustomTuple(12345,4231);
    println!("{:?}",struct_1);

    // C struct
    // this IS a custom TYPE
    #[derive(Debug)]
    struct Rgb{
        r: u8,
        g: u8,
        b: u8
    };

    let struct_2: Rgb = Rgb{ r:200, g:255, b:10};
    println!("{:?}", struct_2);

    // destructure C struct with binding
    let Rgb{r:red, g:green, b:blue} = struct_2;
    println!("red: {} green: {} blue: {}", red, green, blue);

    // Unit struct
    // this is a struct without any fields, it's a TYPE
    // unit structs resemble the empty tuple (), they are mostly useless except for special cases...
    #[derive(Debug)]
    struct UnitStruct;
    let unit = UnitStruct;
    println!("{:?}",unit);

    // structs as fields of another struct
    #[derive(Debug)]
    struct FieldStruct{
        val_1: Rgb,
        val_2: CustomTuple
    };

    let struct_3: FieldStruct = FieldStruct{ val_1: Rgb{r:1,b:2,g:3}, val_2: CustomTuple(42, 24)};
    println!("{:?}",struct_3);

    // destructure Structs
    println!("{}", struct_3.val_1.b);


    // Enums
    // enums are structures that have variants

    #[derive(Debug)]
    enum Enumi{
        Variant1,                   // unit variant
        Variant3(u8,u8),            // tuple variant
        Variant4{r:u8,g:u8,b:u8}    // struct variant
    };

    let enum_1 = Enumi::Variant1;
    println!("{:?}",enum_1);
    let enum_2 = Enumi::Variant3(255,9);
    println!("{:?}",enum_2);
    let enum_3 = Enumi::Variant4{r:9,g:6,b:4};
    println!("{:?}",enum_3);

    // 'use'
    // use creates an Alias ('Duck') for Enumj::Duck
    use Enumj::{Duck};
    let enum_4 = Duck;          // same as: let enum_4 = Enumj::Duck;
    println!("{:?}",enum_4);
    // use Enumj::*; imports all names


    // const
    // const declares a constant variable, in this case in global scope
    // note: globally scoped variables are in upper-case
    println!("{}",CONSTANT_VARIABLE);

    // in contrast, let-declared variables can be REDEFINED
    let variable_1: u8 = 155;
    println!("{}",variable_1);
    let variable_1: u8 = 2;
    println!("{}",variable_1);
    // however, they aren't mutable (mutation-able)
    let mut variable_2: u8 = 155;
    variable_2 = variable_2 + 1;
    println!("{}",variable_2);
}
