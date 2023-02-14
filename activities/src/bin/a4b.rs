// Topic: Decision making with match
//
// Program requirements:
// Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// Use a variable set to any integer
// Use a match expression to determine which message to display
// Use an underscore (_) to match on any value

use std::io;

fn main() {
    println!("Please input an integer: ");
    let input: i32 = user_input();
    match input {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
fn user_input() -> i32 {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input only integers!");
                input.clear();
                continue;
            }
        };
        return input;
    }
}
