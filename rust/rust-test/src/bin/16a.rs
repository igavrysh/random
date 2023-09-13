// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Gene".to_owned(),
            locker: None,
        },
        Student {
            name: String::from("Ievgen"),
            locker: Some(42),
        }
    ];
    
    for student in students {
        println!("name: {:?}", student.name);
        match student.locker {
            Some(id) => println!("locker assigned @ {:?}", id),
            None => println!("no locker assigned"),
        }
    }
}