// functions can be accessed by main
mod functions {

  // add_squares can be accessed by anything higher than itself, that can be accessed
  // it's accessible in main
  pub fn add_squares(n1:isize,n2:isize)->isize{
    additions::add( multiplications::square(n1), multiplications::square(n2) )
  }

  // multiplications can only be accessed by the scope itself is in
  // it can only be accessed in functions
  mod multiplications {
    // square can be accessed by anything higher than itself, that can be accessed
    // it's only accessible in functions, because multiplications is only accessible in functions
    // it is not accessible in main
    pub fn square(n:isize)->isize{
      n*n
    }
  }

  pub mod additions {
    pub fn add(n1:isize,n2:isize)->isize{
      n1+n2
    }

    // double can only be accessed in its scope and within children of that scope
    // it can not be accessed outside of additions, even though additions is public
    fn double(n:isize)->isize{
      n+n
    }

    // additions is public, subtractions is public, so subtractions can be accessed by main
    pub mod subtractions {
      // since subtractions is a child of additions it can access double
      pub fn subtract_double(n1:isize,n2:isize)->isize{
        n1 - ::functions::additions::double(n2)	// the first :: refers to the root-module of the file, which is main()
	/* module access is always relative to the current module
	   that's why calling double() directly wouldn't work here
	   using :: you can override that behaviour
	*/
      }

      pub fn double_subtract_double(n1:isize,n2:isize)->isize{
	// to access the next higher module you can also use the super keyword:
	super::double(n1) - super::double(n2)
	// super refers to additions in this case, because double_subtract_double's scope is subtractions - a child of additions
      }

    }
  }

}


enum TrafficLight {
  Red,
  Green,
  Blue,
}

fn main() {
  println!("22-2*10={}",functions::additions::subtractions::subtract_double(22,10));

  // use brings subtract_double into main's scope, making it directly accessible
  use functions::additions::subtractions::subtract_double;
  println!("93-2*5={}",subtract_double(93,5));
  // NOTE: use statements are not relative to the current module, but to the root-module!
  //       that behaviour can be overridden with super, which makes the use-path relative to the current module as well!


	/* NOTE: ENUMS ARE NAMESPACES, TOO */

  // use TrafficLight namespace to access Green type
  let orange = TrafficLight::Green;

  // bring Green type into main's scope
  use TrafficLight::Green;
  let skyblue = Green;	// access Green type directly

  // use a glob operator to bring all types in TrafficLight into scope
  use TrafficLight::*;
  let purple = Red;
  let violet = Blue;
}

