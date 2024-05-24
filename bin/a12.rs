// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


// i thought of dim as matrix dim (3,2) etc but i watched video since impl instanciation was w/ new keyword and i was erroring 
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64
}

// cheated the idea of impl
impl Dimensions {
    fn print(&self) {
        println!("following are the width, height and depth:{}{}{}",self.width, self.height, self.depth);
    }
}

enum Color {
    Red,
    Green
}

// cheated since i didnt think enums can have implementations
impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("the color is 'Red'"),
            Color::Green => println!("the color is 'Green'")
        }
    }
}

struct Box {
    dimensions : Dimensions,
    weight:  u32,
    color: Color
}

impl Box {
    fn new(dimensions:Dimensions, weight:u32, color: Color) -> Self {
        Self {dimensions: dimensions, weight: weight, color: color}
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight is {}", self.weight);
    }
}

fn main() {
    let temp_dim = Dimensions {
        height: 1.0, width: 2.0, depth: 4.0
    };

    let boxing = Box::new(temp_dim, 47, Color::Red);
    boxing.print();
}
// fought with compiler for a while and won after many loses
// also learned that src/bin can have a rs file and run using `cargo run  --bin a12`