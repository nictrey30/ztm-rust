// Topic: Advanced match
//
// Requirements:
// Print out a list of tickets and their information for an event
// Tickets can be Backstage, Vip, and Standard
// Backstage and Vip tickets include the ticket holder's name
// All tickets include the price
//
// Notes:
// Use an enum for the tickets with data associated with each variant
// Create one of each ticket and place into a vector
// Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Tickets::Backstage("Alin".to_owned(), 22.4),
        Tickets::Vip("Maia".to_owned(), 14.0),
        Tickets::Standard(13.4),
        Tickets::Vip("Gina".to_owned(), 14.0),
    ];
    for ticket in tickets {
        match ticket {
            Tickets::Standard(price) => println!("The price of a standard ticket: {:?}", price),
            Tickets::Vip(name, price) => {
                println!("The price of a vip ticket: {:?} for {:?}", price, name)
            }
            Tickets::Backstage(name, price) => println!(
                "The price of a Backstage ticket: {:?} for {:?}",
                price, name
            ),
        }
    }
}
