// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for.. in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i8,
    name: String,
    color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    let people = vec![
        Person {
            age: 1,
            name: "Charlie".to_owned(),
            color: "red".to_owned(),
        },
        Person {
            age: 35,
            name: String::from("Gene"),
            color: "green".to_owned(),
        },
        Person {
            age: 18,
            name: String::from("Ievgen"),
            color: "soft".to_owned(),
        }
    ];

    for person in people {
        if person.age <= 10 {
            print(&person.name);
            print(&person.color);
        }
    }
}