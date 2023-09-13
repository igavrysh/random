use std::{collections::HashMap, sync::{Mutex, Arc}, thread};

pub fn my_solution_frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input_len = input.len();
    let acc = Arc::new(Mutex::new(HashMap::<char, usize>::new()));

    let counter: Arc<Mutex<usize>> = Arc::new(Mutex::new(input_len));

    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    let vec_input: Vec<String> = input.iter().map(|s| s.to_string()).collect();

    let shared_strings =  Arc::new(vec_input);

    for _ in 0..worker_count {
        let counter = Arc::clone(&counter);
        let acc: Arc<Mutex<HashMap<char, usize>>> = Arc::clone(&acc);
        let shared_strings = Arc::clone(&shared_strings);
   
        let handle = thread::spawn(move || {
            let mut d = HashMap::<char, usize>::new();
            loop {
                let target_idx: usize;
                {
                    let mut i = counter.lock().unwrap();
                    if *i == 0 {
                        // merge d and acc hashmaps together
                        let mut acc = acc.lock().unwrap();
                        for (c, fq) in &d {
                            let fq_acc: &mut usize = acc.entry(*c).or_insert(0);
                            *fq_acc += fq;
                        }
                        return;
                    }
                    *i -= 1;
                    target_idx = *i;
                }

                let input_string = shared_strings.get(target_idx).unwrap();
                
                // populate d based on input string
                for c in input_string.chars() {
                    if !c.is_alphabetic() {
                        continue;
                    }
                    let key = c.to_ascii_lowercase();
                    let fq: &mut usize = d.entry(key).or_insert(0);
                    *fq += 1;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = acc.lock().unwrap().clone();
    result
}
