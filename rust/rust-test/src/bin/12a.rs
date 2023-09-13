// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// Use a struct to encapsulate the box characteristics
// Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct ShippingBox {
    dims: Dims,
    weight: f64,
    color: Color,
}

struct Dims { 
    width: f64,
    depth: f64,
    height: f64,
}

impl Dims {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("depth: {:?}", self.depth);
        println!("height: {:?}", self.height);
    }
}

enum Color {
    Red,
    Blue,
}

impl Color {
    fn print(&self) {
        match self {
            Self::Red => println!("red"),
            Self::Blue => println!("blue"),
        }
    }
}

impl ShippingBox {
    fn new(weight: f64, color: Color, dims: Dims) -> Self {
        ShippingBox { weight: 1.0, color: Color::Red, dims: dims }
    }

    fn display(&self) {
        self.color.print();
        self.dims.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let shiping_box = ShippingBox::new(2.1, Color::Red, 
        Dims { width: 1.0, depth: 2.0, height: 3.0 });
    shiping_box.display();
}