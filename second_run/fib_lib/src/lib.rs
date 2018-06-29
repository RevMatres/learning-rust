use std::env;
use std::process;

// Setup a Memory Vector
pub fn make_memo() -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    v.push(0);
    v.push(1);
    v
}

// Get input arguments
pub fn get_args() -> u32 {

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

// Parse argument to number and return
fn parse_arg(a: &String) -> u32 {

    match a.parse::<u32>() {
        Ok(value) => return value,
        Err(error) => {
            eprintln!("Problem parsing arguments: {}\nFound argument: {}\nThe argument must be a positive integer!", error, a);
            process::exit(1);
        },
    }
    
}


// calculate a fibonacci number and add it to a memory vector
pub fn fib(memo: &mut Vec<u32>, n: u32) -> u32 {

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
