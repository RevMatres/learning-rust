fn main() {

    // a closure
    let wombat = || println!("Wombat!\n");
    wombat();

    // multi-line closure with input parameter accessing its environment
    let x = vec![344, 92, 12];

    let sum_added_a = |a| {
        let mut sum = 0i32;
        for number in x.iter() {
            sum += number
        }
        sum + a
    };

    println!("print the Vector: {:?}\n  the Vector's sum: {}\n  the Vector's sum + 33: {}\n", x, sum_added_a(0), sum_added_a(33));

    // closures refer to the context they're defined in
    {
        let x = vec![9, 9, 9];
        println!("in this scope the Vector is: {:?}\n  the Vector's sum: {}\n  Notice, that that's the sum of the vector in the closure's definition-scope!\n", x, sum_added_a(0));
    }

    // Let's have some fun with iterators, cause they are great!
    // Note: Iterators are Rust's implementation of higher-order functions, which in functional
    // languages are mostly used for iterations…
    let fruit: [&str; 5] = ["Banana", "Apple", "Orange", "Duck", "Peach"];
    let colors: [&str; 5] = [ "Blue", "Yellow", "Orange", "Red", "Lilac"];

    let phrases: Vec<_> = fruit.iter().zip(colors.iter()).collect();
    let phrases: Vec<_> = phrases.iter()
                                 .map(|val| { format!("The {} is {}!", val.0, val.1) })
                                 .collect();

    println!("Now for Iterators:");
    phrases.iter().for_each(|val| println!("  {}", val));

}
