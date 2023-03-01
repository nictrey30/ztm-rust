use std::io;

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn read_choice() -> String {
    println!("please input choice: mainmenu/start/quit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read line.");
    let input = input.trim();
    input.to_owned()
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match &input.to_lowercase()[..] {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("input not found".to_owned()),
    }
}

fn print_choice(choice: &Result<MenuChoice, String>) {
    match choice {
        Ok(result) => println!("valid choice: {:?}", result),
        Err(error) => println!("error: {:?}", error),
    }
}

fn main() {
    let read_input = read_choice();
    let choice = get_choice(&read_input);
    print_choice(&choice);
}
