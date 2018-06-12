enum Coin {
  Penny(String),
  Nickel(String),
  Dime(String),
  Quarter(String),
}

fn main() {

  let piece = Coin::Quarter("Alabamur".to_string());

  match piece {
    Coin::Penny(a) => println!("the Penny says: {}", a),
    Coin::Nickel(a) => println!("the says: {}", a),
    Coin::Dime(a) => println!("the says: {}", a),

    // MATCH GUARD
    // additional test pattern that has to return true for the match arm to execute
    Coin::Quarter(ref a) if a.len() > 10 => {
      println!("the Quarter's gotten a LONG text: {}", a)
    },
    Coin::Quarter(a) => println!("the says: {}", a),
    /* note 'ref a': since a is a String (heap alloc) it would move into the match guard
       if 'ref' wasn't put there, in which case a wouldn't be available for the next
       match arm to use
       in the context of a pattern, '&' matches a reference and gives you its value,
       but 'ref' matches a value and gives you a reference to it
    */
  }
 
}
