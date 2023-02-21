// Topic: Strings
//
// Requirements:
// Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// Use a struct for a persons age, name, and favorite color
// The color and name should be stored as a String
// Create and store at least 3 people in a vector
// Iterate through the vector using a for..in loop
// Use an if expression to determine which person's info should be printed
// The name and colors should be printed using a function
#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Brown,
    Magenta,
}
#[derive(Debug)]
struct People {
    name: String,
    age: u32,
    color: Colors,
}

fn print(data: &str) {
    println!("{}", data)
}

fn main() {
    let people = vec![
        People {
            name: "Ana".to_owned(),
            age: 54,
            color: Colors::Brown,
        },
        People {
            name: "Matei".to_owned(),
            age: 8,
            color: Colors::Blue,
        },
        People {
            name: "Miruna".to_owned(),
            age: 2,
            color: Colors::Yellow,
        },
        People {
            name: "fabian".to_owned(),
            age: 30,
            color: Colors::Red,
        },
    ];
    for person in &people {
        if person.age <= 10 {
            print(&person.name);
            println!("color: {:?}", person.color);
        }
    }
}
