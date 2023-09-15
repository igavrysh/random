use std::collections::HashMap;

// treat mod as individual files
mod greet {
    use std::collections::HashMap;

    pub fn hello(){
        println!("hello")
    }

    pub fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    //greet::hello();
    //use greet::*;
    use greet::hello;

    hello();
    greet::goodbye();

    math::add(1,2);
    math::sub(3, 4);
}