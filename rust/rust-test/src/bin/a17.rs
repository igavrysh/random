// Topic: Browsing standard library documentation
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
// * Try searching for: to_uppercase, to_Lowercase

fn main() {
    let s = "AbC".to_owned();
    let lower = s.to_lowercase();
    let upper: String = s.to_uppercase();
    println!("string {:?} lowercased {:?}", s, lower);
    println!("string {:?} uppercased {:?}", s, upper);
}