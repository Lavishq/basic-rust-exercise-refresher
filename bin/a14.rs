// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: u32,
    fav_color: String
}
fn main() {
    // why cant i create a struct directly `Person("Lvixnatiqsh", 19, "B&W"),`
    let persons = vec!(
        Person{name: String::from("Lavishq"), age: 20, fav_color:String::from("B&W")},
        Person{name:String::from("Lvixnatiqsh"), age:19, fav_color:String::from("B&W")},
        Person{name:String::from("Victor"), age:14, fav_color:String::from("Blue")},
        Person{name:String::from("Michael"), age:7, fav_color:String::from("Red")},
    );
    for person in persons {
        if person.age <= 10 {
            println!("name: {}, fav_color: {}", person.name, person.age);
        }
    }
}

// the hard part here was writing the vector due to String::from() // also "<strin>".to_owned() can be used
// wow, after i finished i check soln and saw use of `&` which i didnt use... have to make a habit of remembering