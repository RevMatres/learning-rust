
fn main() {

  // TUPLE STRUCTS

  // Remember tuples are collections of different types,
  // in contrast to the arrays, which are collections of the same type.

  // Tuple structs are basically just tuples with a type-name associated to them.

  #[derive(Debug)]		// cause I don't yet know, how to implement fmt traits for the println! macro
  struct Point(i32, i32, i32); 

  let X = Point(9,17,33);
  println!("{:?}",X);
}
