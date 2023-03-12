// Topic: Result
//
// Requirements:
// Determine if a customer is able to make a restricted purchase
// Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// Use a struct to store at least the age of a customer
// Use a function to determine if a customer can make a restricted purchase
// Return a result from the function
// The Err variant should detail the reason why they cannot make a purchase

use std::cmp::Ordering;

struct Customer {
    name: String,
    age: i32,
}

fn restricted_purchase(customer: &Customer) -> Result<String, String> {
    match customer.age.cmp(&21) {
        Ordering::Less => Err("cannot buy, under 21y".to_owned()),
        Ordering::Equal => Ok("can buy cigs".to_owned()),
        Ordering::Greater => Ok("can buy cigs".to_owned()),
    }
}

fn print_result(customer: &Customer, enquiry: &Result<String, String>) {
    match enquiry {
        Ok(result) => println!("{:?} {:?}", customer.name, result),
        Err(error) => println!("{:?} {:?}", customer.name, error),
    }
}

fn main() {
    let customer_1 = Customer {
        name: "Ionel".to_owned(),
        age: 32,
    };
    let customer_2 = Customer {
        name: "Gigel".to_owned(),
        age: 17,
    };
    print_result(&customer_1, &restricted_purchase(&customer_1));
    print_result(&customer_2, &restricted_purchase(&customer_2));
}
