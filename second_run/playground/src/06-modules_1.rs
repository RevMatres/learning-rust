// module definition
mod module_1 {

  // defining a public function:
  pub fn print_me(){
    println!("I'm print_me() from module_1");
  }

  // defining a private function:
  fn square(n:isize) -> isize {
    n*n
  }
  // using a private function:
  pub fn add_squares(n1:isize,n2:isize) -> isize {
    square(n1)+square(n2)
  }
}

mod module_2 {

  // different namespaces and functions of the same name:
  pub fn print_me(){
    println!("I'm also called print_me(), but not from module_1");
  }

}

mod module_3 {

  // modules in modules:
  pub mod sub_module_1 {	// note that this module needs to be explicitely made public for me to access its print_me() fn via module_3
    pub fn print_me(){
      println!("wheeeee");
    }
  }

  mod sub_module_2 {
    pub fn square(n: isize) -> isize {
      n*n
    }
    pub fn add(n1: isize, n2: isize) -> isize {
      n1 + n2
    }
  }

  pub fn add_squares(n1:isize,n2:isize) -> isize {
    sub_module_2::add( sub_module_2::square(n1), sub_module_2::square(n2) )
  }

}

fn main() {
  // calling a public function:
  module_1::print_me();
  module_2::print_me();
  module_3::sub_module_1::print_me();

  println!("the sum of 92^2 and 33^2 is {}",module_1::add_squares(92,33));
  println!("the sum of 42^2 and 9 is {}", module_3::add_squares(42,3)); 
  println!("{}",module_3::sub_module_2::square(2));

  /* It appears that to load modules some form of dynamic link loader is used.
     You use the field access syntax from structs (::) to access fields of the module,
     because the module is loaded onto the heap in bunch, making everything within
     it a field of it.
     Also: a module is also referred to as a namespace.
  */
}
