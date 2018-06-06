// define the struct for use in main
#[derive(Debug)]
struct Rectangle {
  height: i32,
  width: i32,
}

// implement functions for the struct
impl Rectangle {

  fn area(&self) -> i32 {	// pass the Self of the Struct to the function = make it a method
    self.height * self.width
  }

  fn can_contain(&self, other_rect:&Rectangle) -> bool {	// the Self always comes first
    self.area() > other_rect.area()
  }

  // This is an associated function,
  // its not a method cause it doesn't have the struct's Self
  fn make_square(size: i32) -> Rectangle {
    Rectangle {
      height: size,	// note that we're filling in fields here, not variables, which is why this isn't assigned
      width: size,	// using the binding operator '=', but rather with a colon
    }
  }
  
}

fn mk_rect(width: i32, height: i32) -> Rectangle {
  Rectangle {
    height,
    width,
  }
}

fn main() {
  
  // how to access tuple's values (by dotted index)
  let a = (55, 99);
  println!("{} {}",a.1,a.0);

  // structs with methods
  // note: a method is a function, that has a struct as its Self
  let b = mk_rect(92,29);
  let c = b.area();
  println!("rectangle: {:?}",b);
  println!("area: {:?}",c);

  // methods, that take arguments
  let d = mk_rect(12,8);
  let e = b.can_contain(&d);
  println!("can b contain d?: {}",e);

  // calling an associated function
  let f = Rectangle::make_square(22);
  println!("here's a square: {:?}",f);

}
