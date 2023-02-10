// Topic: Functions

// Program requirements:
// Displays your first and last name

// Notes:
// Use a function to display your first name
// Use a function to display your last name
// Use the println macro to display messages to the terminal
use std::io;
#[derive(Debug)]
enum NameType {
    FirstName(String),
    LastName(String),
}

fn read_name() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input
}

fn display_name(name: &NameType) {
    match &name {
        NameType::FirstName(firstname) => println!("Your first name is {:?}", firstname.trim()),
        NameType::LastName(lastname) => println!("Your last name is {:?}", lastname.trim()),
    }
}

fn main() {
    println!("enter your first name: ");
    let first_name = NameType::FirstName(read_name());

    println!("enter your last name: ");
    let last_name = NameType::LastName(read_name());

    display_name(&first_name);
    display_name(&last_name);
}
