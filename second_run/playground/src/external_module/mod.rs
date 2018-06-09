mod sub_module;
mod sub_module_2;

pub fn print_me(){
  println!("I'm print_me() from the module 'external_module' in the file 'external_module.rs'!");
}

pub fn add_squares(n1:isize,n2:isize)->isize{
  sub_module::add( sub_module::square(n1), sub_module::square(n2) )
}
