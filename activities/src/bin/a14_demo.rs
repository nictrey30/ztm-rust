struct LineItem {
    name: String,
    count: u32,
}

fn print_name(name: &str) {
    println!("name: {}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 2,
        },
        LineItem {
            name: String::from("fruit"),
            count: 2,
        },
    ];
    for item in receipt {
        print_name(&item.name);
        println!("count: {}", item.count);
    }
}
