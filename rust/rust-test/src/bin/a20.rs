// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::{io, fmt, error::Error};

use State::*;

enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn str_2_state(str: &str) -> Option<State> {
    match str.to_lowercase().as_str() {
        "off" => Some(Off),
        "sleep" => Some(Sleep),
        "reboot" => Some(Reboot),
        "shutdown" => Some(Shutdown),
        "hibernate" => Some(Hibernate),
        _ => None,
    }
}

fn print_state(state: State) {
    let msg = match state {
        Off => "porwering off",
        Sleep => "sleeping",
        Reboot => "rebooting in 29 sec",
        Shutdown => "shutting down now",
        Hibernate => "hibernate will start in 5 min"
    };
    println!("{}", msg);
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

#[derive(Debug, Clone)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

// Implement the Error trait for the custom error type
impl Error for MyError {}

fn main() {
    _ = get_input()
        .map_err(|err| Box::new(err) as Box<dyn Error>)
        .and_then(|input|
            match str_2_state(input.as_str()) {
                Some(state) => Ok(state),
                None => Err(Box::new(MyError("cannot parse state".to_owned()))),
            }
        )
        .map(|state| print_state(state));
}
