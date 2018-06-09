fn main() {

  // hashmaps aren't part of the rust prelude
  use std::collections::HashMap;

  // you can use type inference with hashmaps
  let mut object = HashMap::new();

  object.insert("key 1".to_string(), "value 1".to_string());
  object.insert("key 2".to_string(), "nine-hundred-and-ninety-nine".to_string());
  object.insert("key 3".to_string(), "quackadoodle".to_string());
  object.insert("key 4".to_string(), "wheeeeeeeeee".to_string());

  {
  // access for specific key
  // get(&T) takes a reference
  // get() returns an Option<T>
  let a = object.get("key 3");

  // to deconstruct the Option<T>::Some(T)
  match a {
    Some(value) => println!("{}",value),
    _ => (),
  }
  } // note: I put this in a separate scope, so a will be dropped
    // a is a share-borrow to object, and before a is dropped I can't create a mutable borrow, which is needed for changing the HashMap
  
  // overwriting a value
  object.insert("key 3".to_string(), "knock knock bitches".to_string());

  // insert a value if the key doesn't yet exist
  object.entry(String::from("key 3")).or_insert("meek and small".to_string());
  object.entry("key 19".to_string()).or_insert(String::from("arrogant and belittleing"));

  // or use if let as implemented in this handy function
  print_value_from_key(object.get("key 19"));	// will print the value
  print_value_from_key(object.get("key nope")); // won't do anything

  // access by key and value
  for (key, value) in &object {
    println!("{}: {}",key,value);
  }

  // NOTE: a HashMap takes ownership of values, that don't implement the Copy trait
  // primitives implement Copy, Collections and complex values, like custom types don't [by default]
  // insert() moves non-Copy values into the HashMap and invalidates previous references

}

fn print_value_from_key(a:Option<&String>){
  if let Some(value) = a {
    println!("{}", value);
  }
}
