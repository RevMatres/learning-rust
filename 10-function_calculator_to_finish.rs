type XY = (f32,f32);

fn main() {

    // arguments
    let left_bound = -100 as f32;               // these are to be taken from the commandline
    let right_bound = 100 as f32;
    let step_size = 0.1 as f32;

    // necesseties
    let mut output: Vec<XY> = Vec::new();

    // calculation
    calc(left_bound,right_bound,step_size,&mut output);

    //output
    println!("{:?}",output)                     // formatting to some format still needs to occur
                                                // output to some standard filename if not otherwise specified needs to occur
}

fn calc(left_bound:f32,right_bound:f32,step_size:f32,output: &mut Vec<XY>) {
    let mut x = left_bound;
    while x <= right_bound {
        let y = x*2;                            // here goes the 'parsed' function
        let result: XY = (x,y);
        output.push(result);
        x += step_size;
    }
}
