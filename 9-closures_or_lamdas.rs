fn main(){
    // just a simple closure
    let square = |i| i*i;
    println!("square 2: {}",square(2));
    println!("square 3: {}",square(3));
    println!("square 25: {}",square(25));


    // capture stuff

    let mut stack: Vec<u8> = Vec::new();

    // push_to_stack automatically &mut borrows stack
    // no need to specify -> that's bloody brilliant!
    let mut push_to_stack = |i| {
        stack.push(i);
        println!("the stack is now: {:?}",stack);
    };

    push_to_stack(20);
    push_to_stack(9);
    push_to_stack(247);
    push_to_stack(33);
    push_to_stack(88);
    push_to_stack(250);
}
