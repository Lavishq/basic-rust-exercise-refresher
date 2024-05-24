// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter


struct Item {
    id: i32,
    quantity: i32
}

fn main() {
    let i = Item {id:0047, quantity:7};
    println!("id {} quantity {}", display_id(&i), display_quantity(&i));
}

fn display_quantity(item: &Item) ->i32 {
    item.quantity
}

fn display_id(item: &Item)-> i32 {
    item.id
}

// had errors yet again in specifying type of struct(i mean initializing) and passing the borrow to another function but compiler helpd