// Topic: Data management using tuples
//
// Requirements:
// Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// Use a function that returns a tuple
// Destructure the return value into two variables
// Use an if..else if..else block to determine what to print
use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Point(i32, i32);

fn read_user_input() -> i32 {
    println!("Input a coord between -10..10");
    let mut my_input = String::new();
    loop {
        io::stdin()
            .read_line(&mut my_input)
            .expect("failed to read line.");
        let my_input: i32 = match my_input.trim().parse() {
            Ok(num) => {
                if num > 10 {
                    println!("Please input a number smaller than 10");
                    my_input.clear();
                    continue;
                } else if num < -10 {
                    println!("Please input a number greater than -10");
                    my_input.clear();
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Please input only numbers!");
                my_input.clear();
                continue;
            }
        };
        return my_input;
    }
}

fn generate_random() -> i32 {
    // generate a random coord between -10.. 10
    let rand_coord: i32 = rand::thread_rng().gen_range(-10..11);
    rand_coord
}

fn point_coord() -> Point {
    Point(generate_random(), generate_random())
}

fn compare_point(coord: i32, compare_coord: i32) {
    match coord.cmp(&compare_coord) {
        Ordering::Equal => println!(
            "random coord: {} is equal with your compare_coord: {}",
            coord, compare_coord
        ),
        Ordering::Less => println!(
            "random coord: {} is less than your compare_coord: {}",
            coord, compare_coord
        ),
        Ordering::Greater => println!(
            "random coord: {} is greater than your compare_coord: {}",
            coord, compare_coord
        ),
    }
}

fn main() {
    let random_point: Point = point_coord();
    let Point(x, y) = random_point;
    let compare_coord = read_user_input();
    compare_point(x, compare_coord);
    compare_point(y, compare_coord);
}
