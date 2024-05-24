// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker_assignment: Option<i32>
}

fn main() {
    // compiler helped for below `Some()`
    let student = Student{name: String::from("Lavishq"), locker_assignment: Some(47)};

    println!("student: {}", student.name);

    // cheated below for 2s
    match student.locker_assignment {
        Some(n) => println!("locker number: {}", n),
        None => println!("no locker assigned"),
    }
}
