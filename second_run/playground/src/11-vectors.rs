fn main() {
  
  // make an i32 Vector
  let mut a: Vec<i32> = Vec::new();

  /* the <> syntax is for generically typed Collections
     collections are heap-allocated, which means they can have types of any size -- generic types
     if you know what type the collection has (no inference required) you can simply use a type in the <>s, instead of <T>
  */

  // add values to it
  a.push(33);
  a.push(99);
  a.push(231);
  a.push(98794275);
  a.push(30284);
  a.push(888);

  // you can read a value by taking a borrow
  let third_value: &i32 = &a[2];

  // print each value
  for i in &a {
    println!("{}",i);
  }

  // you can also use type inference with vectors
  let mut v = Vec::new();
  v.push(String::from("sweet baby jesus"));

  // Vectors are collections of one type
  // if you need to have more than one type to be stored in the vec, use an enum
  use custom_types_collection_type::*;

  let mut b: Vec<custom_types_collection_type> = Vec::new();

  b.push(type1("quackadoodle".to_string()));
  b.push(type3{key1:"knock".to_string(),key2:9});
  b.push(type2(17));

  for i in &b {
    match i {
      type1(value) => println!("{}",value),
      type2(value) => println!("{}",value),
      type3{key1:value1, key2:value2} => println!("{} {}",value1,value2),
      type4 => println!("it's type 4"),
    }
  }

}

#[derive(Debug)]
enum custom_types_collection_type {
  type1(String),
  type2(i32),
  type3{ key1:String, key2:i32 },
  type4,
}
