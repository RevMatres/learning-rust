fn main() {


  // STRUCTS AND TUPLES
  /* "A struct, or structure, is a custom data type"
     "If you're familiar with an object-oriented language, a struct is like an object's data attributes."
     Basically it's a description of the type so we can make one of these.
     Class Inheritance OOP isn't a thing Rust really does, it's compositional after all.
  */

  // Description of the TYPE "Person"
  struct Person {
     name: String,
     age: u64,
     height: u64,
  }

  // Let's make a Variable of the Type Person
  let dieter = Person {
     name: String::from("Dieter Dűmpel"),
     age: 256,
     height: 9,
  };			// note that this is a var-assignment operation, so remember the semicolon!

  println!("dieter's age: {}",dieter.age);

  // Let's make a mutable Variable of the Type Person
  let mut gretchen = Person {
     name: String::from("Gretchen Gans"),
     age: 32399,
     height: 10,
  };

  println!("gretchen's height: {}", gretchen.height);

  gretchen.height = 12;

  println!("gretchen's new height: {}", gretchen.height);


  // NOW LET'S BUILD A CUBE WITH A FUNCTION!
  let boxy = make_cube(20,9,18,17);
  println!("boxy coords: {}, {}, {}",boxy.pos_x,boxy.pos_y,boxy.pos_z);
}

// First define the struct TYPE description
struct Cube {
    pos_x:u32,
    pos_y:u32,
    pos_z:u32,
    side:u32,
}

// Then make a function to fill in one of these types and (automatically) return it
fn make_cube(x:u32,y:u32,pos_z:u32,side:u32) -> Cube {	// note the custom return type!
    Cube {
        pos_x:x,
        pos_y:y,
        pos_z,		// because the parameter and this field are both called the same
        side,		// you can use the 'field init shorthand syntax'
    }
}
