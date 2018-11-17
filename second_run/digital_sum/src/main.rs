use std::process::exit; // Error handling
use std::io::Write; // Need the write trait's flush method
use std::io; // Need access to stdin

/**
 * Function, that'll compute a base 10 cross sum
 */
fn digit_sum(mut num: u64) -> u64 {

    let mut sum = 0;

    while num != 0 {
        sum += num % 10;
        num /= 10;
    }

    return sum;
}

fn main() {

    // Print a welcoming message
    println!("Honk");
    print!("\nEnter a number: ");

    // I have to flush explicitely to empty the stdout buffer,
    // or the system will only run the println! after the read_line()
    // below...
    io::stdout().flush().unwrap();
    // Note: flush is part of the write trait

    // Make a string to get the input
    let mut input = String::new();

    // Get the input and handle potential failure
    match io::stdin().read_line(&mut input) {
        Ok(_v) => {},
        Err(_e) => {
            println!("Couldn't read your input");
            exit(1);
        },
    }

    // Check for maximally encodable number
    if input.trim().len() > 20 {
        println!("Your number is too large to be represented.");
        exit(1);
    }

    // Turn the string into a number... remember to trim whitespace
    let number: u64 = match input.trim()
        .parse::<u64>() {
            Ok(n) => n, // You have to put this first: match infers its return type from the first match arm
            Err(_e) => {
                println!("Your 'number' contains invalid digits.");
                exit(1);
            },
    };

    // Keep score of the digital sum
    let mut digital_sum;

    // Compute a first digit sum (cross sum)
    digital_sum = digit_sum(number);

    // If that digit sum ain't one digit long...
    while digital_sum > 9 {
        // ... redo the cross summing until it bloody is!
        digital_sum = digit_sum(digital_sum);
    }

    // Output.
    println!("The digital sum of {} is {}", number, digital_sum);

}
