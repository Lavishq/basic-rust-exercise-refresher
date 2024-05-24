// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

// borrowchecker helped with & and , instead of ; // also i glanced soln for 3s
fn main() {
    let a_vector = vec![10, 20 , 30, 40];
    for item in &a_vector {
        match item {
            30 => println!("thirty"),
            u32 => println!("{}", item)
        }
    }
    println!("vector elements {}", a_vector.len());
}
