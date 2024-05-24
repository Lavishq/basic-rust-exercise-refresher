// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let a_tuple:(i32, i32) = (8, 7);
    let (x, y) = cartesian_co(a_tuple);

    if y > 5 {
        println!("gt");
    } else if y <5 {
        println!("lt");
    } else {
        println!("eq");
    }
}

fn cartesian_co(t: (i32, i32)) -> (i32, i32) {
    (t.0, t.1)
}
