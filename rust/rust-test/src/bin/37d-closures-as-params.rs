
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
} 

fn main() {
    let name = "Gene";
    let add 
        = Box::new(move |a, b| {
            println!("adding a number for {}!", name);
            a + b
        });
    let sub 
        = Box::new(|a: i32, b: i32| a - b);
    let mult 
        = Box::new(|a: i32, b: i32| a * b);

    println!("{}", math(2, 2, add));
    println!("{}", math(2, 2, sub));
    println!("{}", math(2, 2, mult));

}