// Topic: Flow control using if..else if..else
//
// Program requirements:
// Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// Use a variable set to any integer value
// Use an if..else if..else block to determine which message to display
// Use the println macro to display messages to the terminal
use std::io;

fn main() {
    println!("Input an integer: ");
    let user_input: i32 = read_user_input();
    println!("User input: {}", user_input);
    if user_input == 5 {
        println!("{} is =5", user_input);
    } else if user_input < 5 {
        println!("{} is <5", user_input);
    } else {
        println!("{} is >5", user_input);
    }
}

fn read_user_input() -> i32 {
    let mut user_input = String::new();
    loop {
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");
        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input only integers!");
                user_input.clear();
                continue;
            }
        };
        return user_input;
    }
}
