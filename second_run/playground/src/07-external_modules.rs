mod external_module;	// This is a declaration of a module
			// but it doesn't contain any code.
			// rust will look for a file 'external_module.rs' to find the code for the module's scope


fn main() {
  external_module::print_me();
  println!("the sum of 9^2 and 17^2 is {}", external_module::add_squares(9,17));
}
