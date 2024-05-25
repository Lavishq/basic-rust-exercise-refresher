// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

// cheat for below line, i wrote #derive (Debug)
#[derive(Debug)]
struct Adult {
    name: String,
    age: u32
}

impl Adult {
    fn new(name: &str, age:u32) -> Result<Self, &str> { // i used incorrect Result(Ok(Adult), Err(String))
        if age >=21 {
            Ok(Self{name: name.to_owned(), age})
        } else {
            Err("Age must be at least 21")
        }
    }
}

fn main() {
    let adult1 = Adult::new( "Lavishq", 22);
    let adult2 = Adult::new( "Victor",  14);

    let adults = vec![adult1, adult2];

    for adult in adults {
        match adult {
            // i was stuck on how to print since the impl has new, so do i have to create a print as well in impl ... lets peek the soln
            Ok(adult) => println!("{} is {} years old, testing again after removing a a print", adult.name, adult.age),
            Err(e) => println!("{e}")
        }
    }
}

// final note... this was diff.. had struct errors, Result errors, argument etc etc but after a lot of effort it was solved in the end