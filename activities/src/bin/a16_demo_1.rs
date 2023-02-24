struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem {
            name: "bananas".to_owned(),
            qty: 4,
        },
        GroceryItem {
            name: "eggs".to_owned(),
            qty: 12,
        },
        GroceryItem {
            name: "bread".to_owned(),
            qty: 1,
        },
    ];
    for item in groceries {
        if item.name == name {
            // using return to return early from a function
            return Some(item.qty);
        }
    }
    None
}

fn main() {
    let mark = Customer {
        age: Some(32),
        email: "mark@example.com".to_owned(),
    };
    let becky = Customer {
        age: None,
        email: "becky@example.com".to_owned(),
    };
    match becky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided"),
    }
    println!("bananas: {:?}", find_quantity("bananas"));
    println!("peanuts: {:?}", find_quantity("peanuts"));
}
