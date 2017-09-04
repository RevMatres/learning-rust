fn main() {
    let is_there_stuff: bool = false;

    println!("Question: Is there stuff?");
    println!("Answer: {}",if is_there_stuff {"yes"} else {"no"});

    if is_there_stuff {
        println!("The Timey-Wimey-Detector says: ");
        loop { println!("Ding!"); println!("..."); println!("..."); println!("...");}
    } else { println!("The Timey-Wimey-Detector says: The stuff is a lie."); }
}
