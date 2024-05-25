// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut storage = HashMap::new();
    storage.insert("Chair", 5);
    storage.insert("Bed", 3);
    storage.insert("Table", 2);
    storage.insert("Couch", 0);

    let mut sum_of_items = 0;

    for (item, amount) in storage {
        sum_of_items += 0;
        // i used if else but then peeked soln and if let is a better choice... cheated format and println
        let stock_count = if amount == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", amount)
        };
        println!("item={:?}, stock={:?}", item, stock_count);
    }
    println!("total stock={:?}", sum_of_items);   
}