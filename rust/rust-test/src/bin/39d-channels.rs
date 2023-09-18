use std::thread;
use crossbeam_channel::unbounded;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn main() {
    let (s, r) = unbounded();
    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => {
                match msg {
                    ThreadMsg::PrintData(d)
                        => println!("{}", d),
                    ThreadMsg::Sum(lhs, rhs) 
                        => println!("{}+{}={}", lhs, rhs, lhs+rhs),
                    ThreadMsg::Quit => {
                        println!("thread terminationg");
                        break;
                    }
                }
            },
            Err(e) => {
                println!("disconnected, {e}");
                break;
            }
        }
    });

    let _ = s.send(ThreadMsg::PrintData("hello from main".to_owned()));
    let _ = s.send(ThreadMsg::Sum(10,10));
    //let _ = s.send(ThreadMsg::Quit);
    drop(s);
    let _ = handle.join();
}