fn main(){

    for n in 10..16 {
        println!("n is {}",n);
    }

    let array_1: [i32;5] = [1,2,3,4,5];
    for n in array_1.iter() {
        println!("the {}th value of array_1 is {}",n-1,n);
    }
}
