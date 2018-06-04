
fn main(){
    // define fib_memo as a mutable u64 Vector
    // initialise it with the base-case values for the recursive function
    let mut fib_memo: Vec<u64> = vec![0,1];
    println!("the fib_memo Vector looks like this: {:?}",fib_memo);

    // call fib()
    let fib_93 = fib(93,&mut fib_memo);
    println!("the 93rd fibonacci number is {}",fib_93);

    // output
    println!("the fib_memo Vector now looks like this:");
    println!("{:?}",fib_memo)
}

// fibonacci function
// memo is a mutable reference to a vector -> that's filled with fib_memo
fn fib(fib_number:u64, memo: &mut Vec<u64>) -> u64 {

    // if fib(fib_number) is already in fib_memo return memo[n]
    if fib_number < memo.len() as u64 { return memo[fib_number as usize] }
    // NOTE: memo.len() is usize, n is u64, in a comparison both need be the same type, so memo.len() as u64
    // NOTE: vector indexing works only with usize, so n as usize

    // if fib(fib_number) isn't there, calculate new value and add it to fib_memo
    let value = fib(fib_number-1, memo) + fib(fib_number-2, memo);
    memo.push(value);

    // return the value of the fib_number's fibonacci number
    value
}
