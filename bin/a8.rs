// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Vanilla, Chololate, Water
}

// cheated struct, well i didnt watch videos or took any help so it was okay i guess
struct FlavorInfo {
    flavor: Flavor,
    ounce: i32
}

fn main() {
    let temporory_flavor = FlavorInfo{flavor:Flavor::Water, ounce:49};  
    drink_flavor_and_ounces(temporory_flavor);
}

fn drink_flavor_and_ounces(temporory_flavor: FlavorInfo) {
    match temporory_flavor.flavor {
        // below was difficult, a little
        Flavor::Vanilla => println!("flavor vanilla {:?}",temporory_flavor.ounce),
        Flavor:: Chololate => println!("flavor chocolate {:?}", temporory_flavor.ounce),
        Flavor:: Water => println!("flavor water {}", temporory_flavor.ounce),
    }
}