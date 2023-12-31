use std::{thread::{self, JoinHandle}, time::Duration};

fn main() {
    let value: JoinHandle<usize> = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        42
    });

    println!("Waiting on thread");

    match value.join() {
        Ok(n) => println!("value: {}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}