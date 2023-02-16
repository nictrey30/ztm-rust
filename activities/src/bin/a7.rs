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
use std::io;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
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

fn read_user_input() -> i32 {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Please input a positive integer not 0!");
                    input.clear();
                    continue;
                } else if num < 0 {
                    println!("Please input a positive integer");
                    input.clear();
                    continue;
                } else if num > 50 {
                    println!("Please input a number not greater than 50");
                    input.clear();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Please input a number");
                input.clear();
                continue;
            }
        };
        return input;
    }
}

fn return_occurance_percentage(elem: i32, total_elem: i32) -> i32 {
    let percentage: f32 = (elem as f32 / total_elem as f32 * 100.0).round();
    percentage as i32
}

fn main() {
    let mut sum_red = 0;
    let mut sum_blue = 0;
    let mut sum_green = 0;
    let mut color_vector: Vec<Colors> = Vec::new();
    println!(
        "Please input an unsiged integer for the number of iterations of random colors(max 50)"
    );
    let num_of_iterations: i32 = read_user_input();
    for _i in 0..num_of_iterations {
        color_vector.push(generate_color());
    }
    // calculate the number of occurances for each color in the vector
    for i in &color_vector {
        match i {
            Colors::Blue => sum_blue += 1,
            Colors::Red => sum_red += 1,
            Colors::Green => sum_green += 1,
        }
        print_color(i);
    }

    println!(
        "red appears {}% times, green appears {}% times, blue appears {}% times",
        return_occurance_percentage(sum_red, num_of_iterations),
        return_occurance_percentage(sum_green, num_of_iterations),
        return_occurance_percentage(sum_blue, num_of_iterations)
    );
}
