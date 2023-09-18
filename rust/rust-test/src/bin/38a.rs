// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let threads = vec![
        thread::spawn(|| { msg_hello() }),
        thread::spawn(|| { msg_thread() }),
        thread::spawn(|| { msg_excited() }),
    ];

    let mut strs: Vec<&'static str> = vec![];
    for thread in threads {
        match thread.join() {
            Ok(str) => strs.push(str),
            Err(e) => println!("error: {:?}", e), 
        }
    }

    for i in 0..strs.len() {
        print!("{}", strs[i]);
        if i == strs.len()-1 {
            print!("\n");
        }
    }
}
