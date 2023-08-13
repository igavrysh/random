// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity
// * Create a function to display the id number

struct Item {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &Item) {
    println!("Quantity is {:?}", item.quantity);
}

fn display_id(item: &Item) {
    println!("Id is {:?}", item.id);
}

fn main() {
    let item = Item { quantity: 1, id: 123 };
    display_quantity(&item);
    display_id(&item);
}