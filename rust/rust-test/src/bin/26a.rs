// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::offset::Utc;

fn main() {
    let now = Utc::now();
    println!("{}", now);
    println!("{}", now.format("%b %-d, %-I:%M").to_string());
}
