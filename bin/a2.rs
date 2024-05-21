// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let a = 4;
    let b = 3;
    display(a,b);
}

fn display(a:u32,b:u32) {
    let sum = add(a,b);
    println!("addition, {:?}", sum);
}

fn add(a:u32,b:u32) -> u32 {
    a+b
}
