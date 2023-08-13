// video 62
//
// Topic: Result
//
// Requirements:
// * Create an structure named 'Adult' that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using 'derive
// * Implement a 'new' function for the 'Adult' structure that returns a Result:
//   * The ok variant should contain the initialized structure, but only 
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why 
//     the structure could not be created
// * Instantiate two 'Adult' structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use 'match' to print out a message for each 'Adult':
//   * For the ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        match age {
            age if age < 21 => Err("should be at least 21Y old"),
            age =>  Ok(Self {
                name: name.to_owned(),
                age: age,
            })
        }
    }
}

fn main() {
    let res_adults = vec![
        Adult::new("Gene", 21), 
        Adult::new("Ievgen", 18)
    ];

    for res in res_adults {
        match res {
            Ok(_) => println!("created adult"),
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }

}