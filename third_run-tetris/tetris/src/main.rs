mod lib;

use lib::*;

fn main() {
    //honk();


    // Lets make a vector with values yay
    let mut rows = vec!(1, 3, 4, 4, 3, 4, 5, 3, 7, 3, 9, 3);

    // Output the values from the vector
    print!("      0 1 2 3 ...        10 11\n");
    print!("rows: "); rows.iter().for_each(|x| print!("{} ", x)); print!("\n");

    // Figure out, how many numbers in rows are even (those will be purged)
    let removed = rows.iter().filter(|x| *x%2 == 0).count();

    // Create replacement vals
    let mut empties = Vec::new();

    // If there are numbers to be removed
    if removed > 0 {
        // Add a 0 for each to-be-removed number to the replacement vals
        for i in 0..removed { empties.push(0); }
    }

    println!("\nnumber of even numbers: {}", removed);

    // Remove and Replace
    rows = empties.into_iter()          // Start with a collection of empties (cause I want row 0 to be the top-most row)
        .chain(                         // Add to the empties...
            rows.into_iter()            // ... the rows...
                .filter(|x| *x%2 != 0)  // ... but only the odd ones!
        ).collect();                    // And make that into a collection again


    // Output
    print!("rows: "); rows.iter().for_each(|x| print!("{} ", x)); print!("\n");
}
