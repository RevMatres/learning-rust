extern crate env_logger;

extern crate fib_lib;
use fib_lib::make_memo;

use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;


fn main() {
    env_logger::init();

    //let mut memory = make_memo();
    //let n1 = fib(&mut memory, 93u64);
    

    // SETUP and SETTINGS part

    // Set the to-be-calculated numbers
    let num_a = 42u64;
    let num_b = 65u64;

    // Setup memory Vector as an Atomically Reference Counted Vector, with a Mutex Lock
    let mut memory = Arc::new(Mutex::new(make_memo()));

    println!("{}", Arc::strong_count(&memory));


    // THREADING

    // Thread for num_a
    //
    // Create a Mutex for the memory vector
    let mutex_a = Arc::clone(&memory);
    println!("{}", Arc::strong_count(&memory));
    // Create the Thread
    let calc_a = thread::spawn(move || {

        let fib_num = fib(mutex_a, num_a, 1);

        println!("Thread 1 {}", fib_num);
    });

    // Thread for num_b
    let mutex_b = Arc::clone(&memory);
    println!("{}", Arc::strong_count(&memory));
    let calc_b = thread::spawn(move || {

        let fib_num = fib(mutex_b, num_b, 2);

        println!("Thread 2 {}", fib_num);
    });
    
    // COLLECTING
    // Make sure all threads have time to finish, before exiting main()
    //thread::sleep(Duration::from_millis(10000));
    calc_a.join().unwrap();
    calc_b.join().unwrap();
/*
    let b = Arc::clone(&memory);
    let a = fib(&mut b.lock().unwrap(), 6);
    println!("{}", a);
    */
}

//fn fib(mutex_lock: &mut std::sync::MutexGuard<Vec<u64>>, n: u64, t: i32) -> u64 {
fn fib(mut mutex: Arc<Mutex<Vec<u64>>>, n: u64, t: i32) -> u64 {

    println!("{} {}", t, Arc::strong_count(&mutex));

    // Vector Indices are usizes
    let i = n as usize;

    // the nth fib-number has already been computed
    {
        let a = mutex.lock().unwrap();
        println!("{} {}", t, Arc::strong_count(&mutex));
        if a.len() > i {
            return a[i]
        }
    }
    // the nth fib-number has already been computed
    

    // If the fib hasn't been computed yet, do so:
    let a = fib(Arc::clone(&mutex), n-2, 1);
    let b = fib(Arc::clone(&mutex), n-1, 1);
    let s = a+b;

    {
        let mut m = mutex.lock().unwrap();
        m.push(s);
        println!("Thread {}: {}", t, m[i]);
    }

    thread::sleep(Duration::from_millis(1));

    s

}
