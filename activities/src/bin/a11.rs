// Topic: Ownershi
// Requirements:
// Print out the quantity and id number of a grocery item
//
// Notes:
// Use a struct for the grocery item
// Use two i32 fields for the quantity and id number
// Create a function to display the quantity, with the struct as a parameter
// Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id_number: i32,
    quantity: u32,
}
impl GroceryItem {
    fn display_item(&self) {
        println!("Quantity of {:?}: {:?}", self.id_number, self.quantity);
    }
}

fn main() {
    let my_item = GroceryItem {
        id_number: 43,
        quantity: 87,
    };
    my_item.display_item();
    // my_item.display_item();
}
