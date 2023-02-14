// Topic: Decision making with match
//
// Program requirements:
// Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// Use a variable set to either true or false
// Use a match expression to determine which message to display

use rand::Rng;

fn main() {
    let my_bool = rand::thread_rng().gen_bool(0.5);
    match my_bool {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}
