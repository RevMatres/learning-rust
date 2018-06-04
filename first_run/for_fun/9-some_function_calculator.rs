// type OutputTuple = (f32,f32);

fn main() {

    // input variables
    let start_boundary: f32 = -50.0;
    let end_boundary: f32 = 50.0;
    let step_size: f32 = 0.5;

    // necessary variables
    let mut x_values: Vec<f32> = Vec::new();
    let mut output: Vec<f32> = Vec::new();

    // functional stuff
    square(start_boundary,end_boundary,step_size,&mut x_values,&mut output);

    println!("{:?}",x_values);
    println!("{:?}",output);
}

fn square(start: f32, end: f32, step: f32, output_x: &mut Vec<f32>, output_y: &mut Vec<f32>) {
    let mut x: f32 = start;
    loop {
        let result = f(x);
        output_x.push(x);
        output_y.push(result);

        x += step;
        if x >= end {break};
    }
}

fn f(x:f32) -> f32 {
    x*x
}
