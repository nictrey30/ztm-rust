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
struct Dimensions {
    width: f64,
    height: f64,
    length: f64,
}

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
        let volume = self.dimensions.height * self.dimensions.length * self.dimensions.width;
        println!(
            "A new {:?} box with a volume of {:.2} liters and a weight of {:?} kg",
            self.color,
            volume / 1000.0,
            self.weight
        );
    }
}

fn main() {
    let box_1 = ShippingBox::create_box(
        22,
        Color::Brown,
        Dimensions {
            width: 23.2,
            height: 14.5,
            length: 19.0,
        },
    );
    let box_2 = ShippingBox {
        weight: 32,
        color: Color::Black,
        dimensions: Dimensions {
            width: 11.2,
            height: 4.5,
            length: 8.0,
        },
    };
    box_1.print_box();
    box_2.print_box();
}
