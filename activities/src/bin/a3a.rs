// Program requirements:
// Displays a message based on the value of a boolean variable
// When the variable is set to true, display "hello"
// When the variable is set to false, display "goodbye"
//
// Notes:
// Use a variable set to either true or false
// Use an if..else block to determine which message to display
// Use the println macro to display messages to the terminal

use rand::Rng;

fn main() {
    let option = rand::thread_rng().gen_bool(0.5);
    match option {
        true => println!("hello"),
        false => println!("goodbye"),
    }
}
