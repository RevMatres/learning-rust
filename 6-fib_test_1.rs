
fn main(){
    let mut fib_memo: Vec<u64> = vec![0,1];

    let fib_93 = fib(93,&mut fib_memo);
    println!("the 93rd fibonacci number is {}",fib_93);

    println!("the fib_memo Vector now looks like this:");
    println!("{:?}",fib_memo)
}

/*fn fib(n:u64) -> u64 {
    if n == 0 { return 0 }
    if n == 1 { return 1 }
    fib(n-1) + fib(n-2)
}*/

fn fib(n:u64, memo: &mut Vec<u64>) -> u64 {

    // if fib(n) is already in fib_memo return memo[n]
    // NOTE: memo.len() is usize, n is u64, in a comparison both need be the same type
    // NOTE: vector indexing works only with usize
    if n < memo.len() as u64 { return memo[n as usize] }

    // if fib(n) isn't there, calculte new value and add it to fib_memo
    let value = fib(n-1, memo) + fib(n-2, memo);
    memo.push(value);
    value
}
