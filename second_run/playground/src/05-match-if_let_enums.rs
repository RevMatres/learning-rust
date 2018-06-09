fn main() {

  #[derive(Debug)]
  enum test_enumerator {
    Field1,			// type_of_enum with empty Tuple, which is the null-type
    Field2(i32,i32,i32),	// type_of_enum with i32s
    Field3(String),		// type_of_enum with String
    Field4 { a:i32, b:i32 },	// type_of_enum with an associated anonymous Struct instead of a Tuple
    //Field5(19),		// you can't define a type_of_enum with specific info associated, the enumerator/Field name's got to do
  }

  // you can make methods on enums:
  impl test_enumerator {

    fn print(&self){

      match self {
        test_enumerator::Field3(value) => println!("{}",value),	// if an enumerator binds to a value, you can give that value a pattern-name
	test_enumerator::Field2(a,b,c) => println!("{}",a),	// to access it with the match-control-flow operator
	_ => println!("not a String"),				// catch-all
      }

    }

  }

  let test_case = test_enumerator::Field2(9,10,11);
  println!("{:?}",test_case);

  let test_case2 = test_enumerator::Field3(String::from("Wheeeeeee"));
  test_case2.print();

  let mut x = 32i32;
  x = x+99;
  println!("{}",x);

  let a = 92u8;
  if let 92u8 = a {			// if let matches against only one case and ignores the rest
    println!("the case was met.");
  }
  /* it's the same as:
  match a {
    92u8 => println!("the case was met."),
    _ => (),
  }
  */

}
