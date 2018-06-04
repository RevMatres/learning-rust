fn main(){

    let mut current: u8 = 0;

    println!("{}",current);

    loop {
        current += 1;

        if current == 42 {
            println!("The Answer has been found! {}", current);
            continue;   // skip the rest of this iteration and restart
        }

        println!("{}",current);

        if current == 255 {
            println!("We are done here!");
            break;      // exit the loop
        }
    }


    // loops with labels

    current = 0;
    let mut inner_counter;

    'outer: loop {
        println!("outer counter: {}", current);
        current += 1;

        inner_counter = 0;

        'inner: loop{
            println!(" > inner counter: {}",inner_counter);
            inner_counter += 2;
            if inner_counter == 240 { break; }
            if current == 5 { break 'outer; }
        }
    }
}
