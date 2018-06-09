use std::collections::HashMap;

fn main() {

  let numbers = vec![ 19, 0, 7, 7, 8, 35, 256, 82 ];
  println!("vector contains:");
  for i in &numbers {
    println!("  {}",i);
  }
  println!("");

  // calculate mean value
  let mean_value = mean(&numbers);
  println!("mean of numbers: {}",mean_value);
  println!("");

  // get the mode (value that appears the most often)
  let mode_value = mode(&numbers);
  println!("mode of numbers: {}",mode_value);
}

fn mean(values:&Vec<i32>)->f32{
  let sum: f32 = sum(values) as f32;
  let len: f32 = values.len() as f32;
  sum / len 
}

fn sum(values:&Vec<i32>) -> i32 {
  let mut sum: i32 = 0;
  for i in values {
    sum += i
  }
  sum
}

fn mode(values:&Vec<i32>) -> i32 {

  // hashmap to count occurences of a value
  let mut vo: HashMap<i32, i32> = HashMap::new();

  // fill in the hashmap
  for number in values {
    let count = vo.entry(*number).or_insert(0);
    *count += 1;
  }

  // get the largest count in the hashmap
  let mut number_most_often: i32 = 0;
  let mut count_of_that_number: i32 = 0;

  for (number, count) in vo {
    if count > count_of_that_number {	// if number appears more often than the previously found most often appearing number
      number_most_often = number;	// make it the new number_most_often
      count_of_that_number = count;	// and keep the new highest count
    }
  }

  number_most_often
}
