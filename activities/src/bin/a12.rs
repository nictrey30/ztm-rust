// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// Print the characteristics of a shipping box
// Must include dimensions, weight, and color
//
// Notes:
// Use a struct to encapsulate the box characteristics
// Use an enum for the box color
// Implement functionality on the box struct to create a new box
// Implement functionality on the box struct to print the characteristics
#[derive(Debug)]
enum Color {
    Red,
    Brown,
    White,
    Black,
}

#[derive(Debug)]
struct Dimensions(u32, u32, u32);

#[derive(Debug)]
struct ShippingBox {
    weight: u32,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
    fn create_box(weight: u32, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

    fn print_box(&self) {
        println!("The box: {:?}", self);
    }
}

fn main() {
    let box_1 = ShippingBox::create_box(22, Color::Brown, Dimensions(10, 30, 20));
    let box_2 = ShippingBox {
        weight: 32,
        color: Color::Black,
        dimensions: Dimensions(10, 10, 15),
    };
    box_1.print_box();
    box_2.print_box();
}
