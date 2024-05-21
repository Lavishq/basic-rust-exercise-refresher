// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let x: bool = true;
    let y: bool = false;

    display(x);
    display(y);
}

fn display(x: bool) {
    match x {
        true => println!("it true"),
        false => println!("it false"),
    }
}
