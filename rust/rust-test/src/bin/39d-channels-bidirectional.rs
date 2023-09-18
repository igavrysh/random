use std::thread;
use crossbeam_channel::unbounded;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumReult(i64),
    WorkerQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();
    
    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => {
                match msg {
                    WorkerMsg::PrintData(d)
                        => println!("Worker: {}", d),
                    WorkerMsg::Sum(lhs, rhs) => {
                        println!("Worker: summing...");
                        _ = main_tx.send(MainMsg::SumReult(lhs + rhs));
                        ()
                    },
                    WorkerMsg::Quit => {
                        println!("Worker: terminating...");
                        _ = main_tx.send(MainMsg::WorkerQuit);
                        break;
                    }
                }
            },
            Err(e) => {
                println!("Worker: disconnected, {e}");
                _ = main_tx.try_send(MainMsg::WorkerQuit);
                break;
            }
        }
    });

    let _ = worker_tx.send(WorkerMsg::PrintData("Hello from main!".to_owned()));
    let _ = worker_tx.send(WorkerMsg::Sum(10,10));
    let _ = worker_tx.send(WorkerMsg::Quit);


    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::SumReult(answer) => println!("Main: answer = {}", answer),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

    let _ = worker.join();
}