//! # This library contains an implementation of the recursive Fibonnaci-Function
//! It makes use of a Memory Vector to keep the calculation fast.  
//! Calculations can be made up to the Nth Fibonnaci number, before the recursion gets to deep for
//! the computer to handle.

// Required for retrieving arguments and exiting properly
use std::env;
use std::process;

/// This function returns a **mutable** memory vector for the Fibonnaci sequence, that contains the first two
/// values of the sequence, which are the recursion's base cases.
// Setup a Memory Vector
pub fn make_memo() -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    v.push(0);
    v.push(1);
    v
}

/// "Get Arguments" from the calling command and parse them.
/// If too many arguments are provided, exit with an error.
/// 
/// After retrieving the arguments, they are parsed using **parse_arg()**.
// Get input arguments
pub fn get_args() -> u64 {

    // Get input arguments (the fib-number to calculate)
    let args: Vec<String> = env::args().collect();

    // Err on more than one number provided
    if args.len() != 2 {
        eprintln!("Please provide exactly one argument!");
        process::exit(1);
    }

    // Parse
    parse_arg(&args[1])

}

/// "Parse Arguments" from a String to a Number, that can be used for the Fibonnaci calculation.
/// If the conversion fails, exit with an error message.
// Parse argument to number and return
fn parse_arg(a: &String) -> u64 {

    match a.parse::<u64>() {
        Ok(value) => return value,
        Err(error) => {
            eprintln!("Problem parsing arguments: {}\nFound argument: {}\nThe argument must be a positive integer!", error, a);
            process::exit(1);
        },
    }
    
}

/// ## The Fibonnaci function.
/// Calculates the Nth Fibonacci number using a recursion.  
/// The function first checks, whether the looked-for result is already in the provided memory
/// vector. If it is, that result is used, otherwise the new result is added to the memory vector.
///
/// ### Input Parameters
/// The function takes two arguments: `memo` and `n`.  
/// `memo` is a **mutable** reference to a memory vector, preferrably created with the make_memo()
/// function.  
/// `n` is a 64-bit integer; it is the index of the Fibonnaci number, that is to be calculated.
///
/// ### Expected Output
/// The largest Fibonnaci number, that can be calculated using this function, is the 93rd. Beyond
/// that one gets overflow errors.  
/// Using an implementation with only 32-bit integers, one can calculate up to the 45th number.
/// Beyond that the program exits with a stack overflow error, because the recursion gets to deep.
// calculate a fibonacci number and add it to a memory vector
pub fn fib(memo: &mut Vec<u64>, n: u64) -> u64 {

    // Vector Indices are usizes
    let i = n as usize;

    // Check for the base cases:
    // n is 0 or 1
    if n == 0 | 1 {
        return memo[i]
    // the nth fib-number has already been computed
    } else if memo.len() > i {
        return memo[i]
    }

    // If the fib hasn't been computed yet, do so:
    let a = fib(memo, n-2);
    let b = fib(memo, n-1);

    memo.push(a+b);

    memo[i]
}
