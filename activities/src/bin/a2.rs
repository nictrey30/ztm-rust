// Displays the result of the sum of two numbers
// Use a function to add two numbers together
// Use a function to display the result
// Use the "{:?}" token in the println macro to display the result

fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
fn display_result(num: i32) {
    println!("The result is {:?}", num);
}

fn main() {
    let num1 = 50;
    let num2 = 30;
    display_result(sum(num1, num2))
}
