fn main(){
    let mut str1 = String::from("This is a test String...");
    str1 = printer(str1);

    println!("printed from main function: \"{}\"",str1);

    let str2 = String::from("This is a second test String...");
    let str2_reference = &str2;                                     // &str2 is a reference to str2's value
    println!("printed str2 from reference: {}",str2_reference);
    println!("printed str2 from dereferenced reference: {} ... I know, it does the same, cause it's a reference, not a pointer, duh",*str2_reference);
    print_from_reference(&str2);
    println!("printed str2 from str2: {}",str2);

    println!("");

    let str3 = String::from("Third test String...");
    let str3_pointer = &str3 as *const String;
    println!("print the raw pointer to str3: {:?}",str3_pointer);
}


// printer() returns the value that was passed into it
// by reassigning str1 to printer(str1) str1's value is returned to it
fn printer(string: String) -> String{
    println!("printed from printer function: \"{}\"",string);
    string
}

// print_from_reference() takes a reference to a value, that means
// it borrows the value, but it doesn't take ownership and the value isn't dropped at the end of scope
fn print_from_reference(string: &String){
    println!("printed from print_from_reference: {}",string);
}

// TODO: draw up a general explanation of references and pointers and how they work in rust
