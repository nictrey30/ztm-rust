// Requirements:
// Print "its big" if a variable is > 100
// Print "its small" if a variable is <= 100
//
// Notes:
// Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// Use a function to print the messages
// Use a match expression to determine which message to print
use std::io;

fn read_user_input() -> i32 {
    println!("input an integer");
    let mut my_input = String::new();
    loop {
        io::stdin()
            .read_line(&mut my_input)
            .expect("failed to read line.");
        let my_input: i32 = match my_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input only numbers!");
                my_input.clear();
                continue;
            }
        };
        return my_input;
    }
}

fn print_message(num: i32, comparison_result: bool) {
    match comparison_result {
        true => println!("The number {} is lower than 100", num),
        false => println!("The number {} is greater/=than 100", num),
    }
}

fn main() {
    let my_num = read_user_input();
    let is_lower_100: bool = my_num < 100;
    print_message(my_num, is_lower_100);
}
