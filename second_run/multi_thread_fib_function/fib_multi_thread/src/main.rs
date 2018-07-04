extern crate fib_lib;
use fib_lib::{ make_memo, fib };


fn main() {

    let mut memory = make_memo();
    let n1 = fib(&mut memory, 93u64);

    println!("{}", n1);

}
