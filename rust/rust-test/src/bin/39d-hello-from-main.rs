use crossbeam_channel::unbounded;
use std::thread;

fn main() {
    let (s, r1) = unbounded();
    let r2 = r1.clone();


    let handle1 = thread::spawn(move || {
        match r1.recv() {
            Ok(msg) => println!("Thread 1: {}", msg),
            Err(e) => println!("{:?}", e),
        }
    });

    let handle2 = thread::spawn(move || {
        match r2.recv() {
            Ok(msg) => println!("Thread 2: {}", msg),
            Err(e) => println!("{:?}", e),
        }
    });

    let res = s.send("Hello from main!");
    if res.is_err() {
        println!("error while sending message to child thread")
    }

    let res = s.send("Hello from main!");
    if res.is_err() {
        println!("error while sending message to child thread")
    }

    let res = handle1.join();
    if res.is_err() {
        println!("error while joining child thread")
    }

    let res = handle2.join();
    if res.is_err() {
        println!("error while joining child thread")
    }
}