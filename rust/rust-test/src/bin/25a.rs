// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

use std::fmt::Debug;

trait Perimeter {
    fn perimeter(&self) -> f32;
}

#[derive(Debug)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

#[derive(Debug)]
struct Square {
    a: f32
}


impl Triangle {
    fn new(a: f32, b: f32, c: f32) -> Self {
        Triangle { a, b, c}
    }
}

impl Perimeter for Triangle {
    fn perimeter(&self) -> f32 {
       self.a + self.b + self.c
    }
}

impl Square {
    fn new(a: f32) -> Self {
        Square { a }
    }
}

impl Perimeter for Square {
    fn perimeter(&self) -> f32 {
        self.a * 4f32
    }
}

fn print_perimeter<T>(figure: &T) 
where T: Perimeter + Debug {
    println!("shape: {:?}, perimeter: {}", figure, figure.perimeter());
}

fn main() {
    print_perimeter(&Square::new(1f32));
    print_perimeter(&Triangle::new(1f32, 2f32, 3f32));
}
