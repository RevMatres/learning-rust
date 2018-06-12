fn main() {

  // a struct that shall implement Print
  #[derive(Debug)]
  struct Quack {
    field: String,
  }

  // how 'bout a struct that shan't get Print?
  #[derive(Debug)]
  struct Derp {
    field: String,
  }

  // an enum that shall implement Print
  #[derive(Debug)]
  enum Quacky {
    variant_1(i32),
    variant_2(Quack),
  }

  // the print trait with its default value
  trait Print {
    fn print(&self) {
      println!("this type has the print trait");
    }
  }

  // implement print for quack
  impl Print for Quack {
    fn print(&self) {
      println!("{}", self.field);
    }
  }

  // implement print for quacky, ahem, I mean just use the default impl
  impl Print for Quacky {}

  // a function to test the types with
  fn TakesPrint<T: Print + std::fmt::Debug>(param: &T) {
    println!("{:?}", param);
    param.print();
    println!("");
  }

  // if you've gotten complicated Generics with Trait Bounds you can use the where syntax
  fn TakesPrintAsWell<T,U>(param: &T) 
    where T: Print + std::fmt::Debug,
          U: Print
  {
    println!("{:?}", param);
    param.print();
    println!("");
  }

  /* note that in Rust you can play around with the Syntax basically as much as you want to,
     as long as the symbols are in the correct order and not missing it doesn't care about
     WHITESPACE
  */

  /* ========================================================================================= */
  
  // make some instances to play with
  let a = Quack { field: "me-haaaa".to_string() };
  let b = Quacky::variant_1(99);
  let c = Quacky::variant_2(Quack { field: String::from("tranquility") });
  //let d = Quacky::variant_2(a);	// this one moves a
  let e = Derp { field: String::from("herpe-de-derpe") };

  // now run your tests on them!
  TakesPrint(&a);
  TakesPrint(&b);
  TakesPrint(&c);

  // destructure that variant_2 that's gotten a quack in it
  if let Quacky::variant_2(value) = c {
    value.print()
  }

  // this one won't work, cause Derp ain't a Print, eyh!
  //TakesPrint(&e);

}
