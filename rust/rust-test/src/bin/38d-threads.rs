use std::thread;

fn main() {
    let iterations = 10;
    let a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("A:{}", i);
        }
    });

    let b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("\t\tB:{}", i);
        }
    });

    _ = a.join();
    _ = b.join();
}