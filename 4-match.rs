#[allow(dead_code)]
#[allow(non_camel_case_types)]

fn main(){

    // match with variables

    let variable = 3;

    match variable {
        1 => println!("It's a one."),
        2 | 4 | 6 => println!("made with 2,4,6"),
        7...41 => println!("smaller than 42"),
        42 => println!("It is the Answer to Life, the Universe and Everything..."),
        _ => println!("I do not care about {}", variable),
    }

    // match with tuples
    // let tuple = (1,424242,42,33);
    let tuple = (42,1,1,1);

    match tuple {
        (42,_a,_b,_c) => println!("there is a 42, the other values include {} and {}",_a,_b),
        (_a,42,_b,_c) => println!("there is a 42, the other values include {} and {}",_c,_b),
        (_a,_b,42,_c) => println!("there is a 42, the other values include {} and {}",_a,_b),
        (_a,_b,_c,42) => println!("there is a 42, the other values include {} and {}",_b,_c),
        _ => println!("there is no 42, therefore this is a useless tuple... meh.")
    }

    // match with enums
    type age = u8;
    type blueness = u8;
    type height = u8;

    enum Person {
        Doctor(age,blueness,height),
        Driver(age,blueness,height),
        Dentist(age,blueness,height),
        Dimm(age,blueness,height)
    }

    let henry = Person::Driver(22,70,3);

    match henry {
        Person::Doctor(age, blueness, height) => println!("Henry is a Doctor, he has an age of {}, a blueness level of {}, and a height of {}",age,blueness,height),
        Person::Driver(age, blueness, height) => println!("Henry is a Driver, he has an age of {}, a blueness level of {}, and a height of {}",age,blueness,height),
        Person::Dentist(age, blueness, height) => println!("Henry is a Dentist, he has an age of {}, a blueness level of {}, and a height of {}",age,blueness,height),
        Person::Dimm(age, blueness, height) => println!("Henry is Dimm, he has an age of {}, a blueness level of {}, and a height of {}",age,blueness,height)
    }

    // match with pointers/ref
    // match with a reference
    let variable = 24.33;
    let reference = &variable;

    // match reference via destructuring
    match reference {
        &val => println!("value via destructuring of reference: {}",val)
        // the match compares &f32 from the reference to &val and finds f32 corresponds to val
    }

    // match reference via dereferencing the reference
    match *reference {
        val => println!("value via dereferencing of reference: {}",val)
        // the *... syntax is for dereferencing
        // *reference dereferences the reference so that val corresponds to the f32 like above
    }


    // match with structs
    // this is mainly about destructuring the structs
    // destructuring of structs is usually done with re-binding the variables

    #[derive(Debug)]
    struct FooBar { tuple: (f32, u8), number: u8 }

    let structure = FooBar { tuple: (33.33, 255), number: 42 };
    let FooBar { tuple: (a,b), number: c } = structure;             // this is the re-binding, the variables on the left-hand are assigned to the FooBar struct on the right-hand
    println!("the tuple is made of a: {} and b: {}",a,b);
    println!("the number is c: {}",c);





    // match guards
    // the guards are basically if-statements controlling the output of the match
    // let tuple = (23,64);
    // let tuple = (64,64);
    let tuple = (42,9);

    match tuple {
        (a,b) if a == b => println!("both numbers are: {}",a),
        (a,_) if a == 42 => println!("a is 42"),
        _ => println!("the numbers are just numbers.")
    };
}
