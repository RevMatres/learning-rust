extern crate fib;

use fib::*;

fn main() {

    // Setup fib memory
    let mut fib_memo: Vec<u32> = Vec::new();
    fib_memo.push(0);
    fib_memo.push(1);

    // Collect commandline arguments
    let number = get_args();

    // Print the fibonacci number
    println!("The {}th Fibonacci number is: {}", number, fib(&mut fib_memo, number));

    for (index, value) in fib_memo.iter().enumerate() {
        println!("{}    {}", index, value);
    }

}

// Tests for lib.rs
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fib_sequence() {

        let mut fib_memo: Vec<u32> = Vec::new();
        fib_memo.push(0);
        fib_memo.push(1);

        assert_eq!(fib(&mut fib_memo, 0), 0);
        assert_eq!(fib(&mut fib_memo, 1), 1);
        assert_eq!(fib(&mut fib_memo, 5), 5);

    }
}
