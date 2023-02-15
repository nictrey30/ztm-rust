// Topic: Working with an enum
//
// Program requirements:
// Prints the name of a color to the terminal
//
// Notes:
// Use an enum with color names as variants
// Use a function to print the color name
// The function must use the enum as a parameter
// Use a match expression to determine which color name to print
use rand::Rng;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}
fn main() {
    let color = generate_color();
    print_color(&color);
}
fn print_color(color: &Colors) {
    match &color {
        Colors::Blue => println!("The color is blue"),
        Colors::Red => println!("The color is red"),
        Colors::Green => println!("The color is green"),
    }
}
fn generate_color() -> Colors {
    let random_number = rand::thread_rng().gen_range(1..4);
    if random_number == 1 {
        Colors::Red
    } else if random_number == 2 {
        Colors::Blue
    } else {
        Colors::Green
    }
}
