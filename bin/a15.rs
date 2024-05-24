// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, u32), Vip(String, u32), Standard(u32)
}

fn main() {
    let tickets = vec![Ticket::Backstage(String::from("Lavishq"), 17), Ticket::Standard(4), Ticket::Vip(String::from("L"),47)];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => println!("name: {}, price {}", name, price),
            Ticket::Vip(name, price) => println!("name: {}, price {}", name, price),
            Ticket::Standard(price) => println!("price {}", price)

        }
    }
}
