use std::thread;

fn main() {
    let data = vec!['a', 'b', 'c'];
    let caps = thread::spawn(move || {
        let data: Vec<char> = data
            .iter()
            .map(|c| c.to_ascii_uppercase())
            .collect();
        data
    });

    println!("Waiting for value...");

    match caps.join() {
        Ok(n) => println!("value: {:?}", n),
        Err(e) => println!("error joining thread: {:?}", e),
    }
}