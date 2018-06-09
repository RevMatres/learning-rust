use std::collections::HashMap;

fn main() {

  // keys are department names
  // values are vectors of people names
  let mut company_db: HashMap<String, Vec<String>> = HashMap::new();

  add_person_to_department("Add Greg to Sales",&mut company_db);
  add_person_to_department("Add Tony to Sales",&mut company_db);
  add_person_to_department("Add Ella to Advertising",&mut company_db);

  let a = company_db.get("Sales");
  if let Some(value) = a {
    println!("people in sales:");
    for person in value {
      println!("  {}",person);
    }
  }
}

// function takes a command and runs it on the data
fn add_person_to_department(command:&str, data:&mut HashMap<String, Vec<String>>){
  let (person, department) = destructure_command(command);
  mutate_data(person, department, data);
}

// finds the second and fourth word in the four-word command
fn destructure_command(command:&str)->(&str, &str){

  // separate out the words and keep them with their 1-indexed position in the command-string
  let mut words: HashMap<&str, i32> = HashMap::new();
  let mut word_count = 0;
  for word in command.split_whitespace() {
    word_count += 1;
    words.entry(word).or_insert(word_count);
  }
  
  // get the second and fourth position words (which are the person and the department)
  let mut a: &str = "";
  let mut b: &str = "";
  for (word, index) in words {
    if index==2 {
      a = word
    } else if index==4 {
      b = word
    }
  }

  // return the Person and the Department
  (a,b)
}

// insert a person into the database
fn mutate_data(person:&str, department:&str, data:&mut HashMap<String, Vec<String>>){
  // unless the department already exists, add it
  let dep = data.entry(department.to_string()).or_insert(Vec::new());
  // add the person to the department
  dep.push(String::from(person));
}
