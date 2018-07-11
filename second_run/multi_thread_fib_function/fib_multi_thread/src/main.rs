//! # Fibonnaci in Multiple Threads
//! Sounds straight forward, right? This is just an implementation of a Fibonnaci Calculator, that
//! computes two Fib-numbers in parallel in two separate threads, using a shared-state memory
//! vector between them, right?
//!
//! Well, no! Not quite.
//!
//! ## What's odd here.
//! There are two interesting things happening with this program. The code in *main.rs* implements
//! what is described above. That isn't what the final compiled executable does, though.  
//! For one it only calculates the Fibonnaci sequence in one of the threads, and for another you
//! can create a Race Condition with this program. (It's not implemented to have that, but there is
//! a simple change to the code that can be made to create it; see below)
//!
//! ### The Code vs The Compiler
//! The fib-function in this code prints the thread it's running in and the fib-number it just
//! calculated.
//!
//! Thread 1 gets the call "fib(42)", thread 2 gets the call "fib(65)".
//! 
//! So one would expect to get mixed outputs from thread 1 and 2, until thread 1 finishes its
//! operation. After that one expects to get the missing fib-numbers from thread 2 until the 65th
//! fib-number is found. One wouldn't expect any duplicate outputs from thread 1 and 2, because they
//! simply use the exisiting fib-numbers from the memory vector and therefore can reuse each
//! other's computations.
//!
//! That is, as alluded earlier, not what happens.  
//! (At least on my machine, that is...)
//!
//! When the program is running, only thread 1 outputs any fib-numbers. Only thread 1 does any
//! computations at all. Thread 2 only prints anything, once thread 1 has calculated all
//! fib-numbers, including the ones up until and including the 65th, even though it should only
//! calculate up to the 42nd.
//!
//! This happens due to a compiler optimization. A Mutex, which I use to handle the shared-state
//! memory vector, possesses so called "memory barriers or fences". A memory barrier forces the CPU
//! to enforce an access-ordering on memory operations concerning the "membar". Modern compilers
//! often merge operations on related memory fences.
//!
//! The two threads use the same Mutex, so the memory fences used with the Mutex locks are related.
//! The rust-underlying LLVM compiler realises this, and merges the operations from both threads
//! into one sequential operation, which is faster than having to switch back and forth between the
//! threads all the time.  
//! That means only one thread computes the Fibonnaci sequence, while the other thread just waits
//! for the one to finish, so that it can look-up the result of its function call.
//!
//! ### The Data Race
//! The fib-function used in *main.rs* first acquires the Mutex lock for the memory vector to check
//! for its base case: The to-be-computed fib-number already exists in the memory vector. It then
//! releases the Mutex lock again and in the case, that the fib-num doesn't yet exist, makes its
//! recursive calls. After those return it acquires the Mutex lock again, pushes the computed
//! fib-numbers on the memory vector and releases the lock. Then it returns the newly computed
//! fib-number to the function caller.
//!
//! In *main.rs* in between the lines `` and `` the following code is written:
//!
//! ```
//! // add the new fib-num only, if no other fib-call has added the value since the last check
//! if m.len() == i {
//!     m.push(sum);
//! }
//!
//! // To create a race condition, uncomment the following line
//! // m.push(s);
//! ```
//!
//! This is the code, that adds the newly calculated fib-number `sum` to the memory vector `m`.
//!
//! The `if`-statement makes sure **again** (that has been checked, when looking for the base case
//! already!), that the new fib-number doesn't yet exist in the memory vector.
//!
//! If you delete or comment-out the `if`-block and uncomment the m.push(s), below it, you create a
//! Data Race.
//!
//! The race happens, because between checking for the base case and adding the new fib-number to
//! the vector the Mutex lock is released and re-acquired. The nature of the Fibonnaci function is
//! such, that the recursion tree it creates contains many mathematically identical sub-trees. Two
//! identical calls, that are to compute the same fib-number, check for their base cases; both find
//! that their fib-number isn't in the memory vector, because they both check before either can add
//! their number to the vector. Then they both will add their value to the memory vector, creating
//! a duplicate in the Fibonnaci sequence. It follows that all subsequent results are also
//! incorrect.  
//! By forcing the function to check for the computed fib-number *again*, right before it writes it
//! to the memory vector, we can make sure, that no other call has changed the vector between the
//! check for the base case and the writing of the result.
//!
//! Because the unexpected/faulty change to the memory vector happens between the base-case check
//! and the result-writing, this qualifies as a Data Race, where an unexpected interrupt from one
//! concurrent agent, changes a piece of memory, that another agent is also currently accessing.

// Get crates
#[macro_use]
extern crate log;

extern crate fib_lib;
use fib_lib::make_memo;

use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;


fn main() {



    /* 
     * SETUP and SETTINGS part
     *
     */

    // Set the to-be-calculated numbers
    let num_a = 42u64;
    let num_b = 65u64;

    // Setup memory Vector as an Atomically Reference Counted Mutex containing the memory Vector
    let memory = Arc::new(Mutex::new(make_memo()));

    trace!("{}", Arc::strong_count(&memory));



    /*
     * THREADING
     *
     */

    // Thread for num_a

    // Create a Mutex for the memory vector
    let mutex_a = Arc::clone(&memory);

    trace!("{}", Arc::strong_count(&memory));

    // Create the Thread
    let calc_a = thread::spawn(move || {

        // Call the fib function with the Reference to the memory vector, the number, and a
        // Thread-signifier
        let fib_num = fib(mutex_a, num_a, 1);

        // When the above call has finished, print this:
        println!("\nThread 1's function call is complete !!!!     {}", fib_num);
    });


    // Thread for num_b

    // Create a Mutex for the memory vector
    let mutex_b = Arc::clone(&memory);

    trace!("{}", Arc::strong_count(&memory));

    // Create the Thread
    let calc_b = thread::spawn(move || {

        // Call the fib function with the Reference to the memory vector, the number, and a
        // Thread-signifier
        let fib_num = fib(mutex_b, num_b, 2);

        // When the above call has finished, print this:
        println!("\nThread 2's function call is complete !!!!     {}", fib_num);
        // comparison result:
        println!("\nthe {}th fib number is {} (linear recursion calculation)", num_b, fib_lib::fib(&mut make_memo(), 65u64));
    });



    /*
     * COLLECTING
     * Make sure all threads have time to finish, before exiting main()
     */

    calc_a.join().unwrap();
    calc_b.join().unwrap();

    info!("\n{:?}", memory);

}

fn fib(mutex: Arc<Mutex<Vec<u64>>>, n: u64, t: i32) -> u64 {

    trace!("{} {}", t, Arc::strong_count(&mutex));


    // Vector Indices are usizes
    let i = n as usize;


    // Check for base case
    // "The nth fib-number has already been computed"

    // acquire the Mutex lock
    let a = mutex.lock().unwrap();

    trace!("{} {}", t, Arc::strong_count(&mutex));

    // check, whether the current fib-num already exists
    if a.len() > i {
        // if it does, return it
        return a[i]
    }

    // release the Mutex lock
    drop(a);
    

    // Fibonnaci Recursion
    // If the fib hasn't been computed yet, do so:
    let a = fib(Arc::clone(&mutex), n-2, 1);
    let b = fib(Arc::clone(&mutex), n-1, 1);
    let sum = a+b;


    // Write the new fib-number to the memory vector

    // acquire the Mutex lock
    let mut m = mutex.lock().unwrap();

    // add the new fib-num only, if no other fib-call has added the value since the last check
    if m.len() == i {
        m.push(sum);
    }

    // To create a race condition, uncomment the following line
    // m.push(s);

    // output the thread and the newly-made fib-num
    println!("Thread {}: {}", t, m[i]);

    // release the Mutex lock
    drop(m);


    // Give the system time to switch threads
    // This line can be commented-out, but it keeps the calls in order.
    // If it is commented-out, the fib-calcs don't finish in the correct order, necessarily, but
    // still produce the same result.
    thread::sleep(Duration::from_millis(1));


    // Return the fib-num
    sum

}
