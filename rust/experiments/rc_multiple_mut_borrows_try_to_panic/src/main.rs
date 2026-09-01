use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
fn cause_panic() {
    // 1. Create the shared data wrapped in a RefCell and Rc
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // 2. Clone the Rc pointer. 
    // pointer_a and pointer_b now point to the exact same heap data.
    let pointer_a = Rc::clone(&shared_data);
    let pointer_b = Rc::clone(&shared_data);

    // 3. First mutable borrow (Pointer A)
    // We use .borrow_mut() to get a mutable reference to the vector.
    let mut borrow_a = pointer_a.borrow_mut();
    borrow_a.push(4);
    println!("Pointer A successfully borrowed and modified the data.");

    // 4. Second concurrent mutable borrow (Pointer B)
    // CRASH! borrow_a is still alive in this scope. 
    // pointer_b tries to borrow the exact same data mutably at the same time.
    let mut borrow_b = pointer_b.borrow_mut();
    borrow_b.push(5); 

    // (This line will never be reached)
    println!("Pointer B successfully borrowed the data."); 
}

#[allow(dead_code)]
fn solve2() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let pointer_a = Rc::clone(&shared_data);
    let pointer_b = Rc::clone(&shared_data);
    
    let mut borrow_a = pointer_a.borrow_mut();
    borrow_a.push(4);
    println!("Pointer A successfully borrowed and modified the data.");
    drop(borrow_a);

    let mut borrow_b = pointer_b.borrow_mut();
    borrow_b.push(5); 

    println!("Pointer B successfully borrowed the data."); 
}

#[allow(dead_code)]
fn solve1() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let pointer_a = Rc::clone(&shared_data);
    let pointer_b = Rc::clone(&shared_data);

    {
        let mut borrow_a = pointer_a.borrow_mut();
        borrow_a.push(4);
        println!("Pointer A successfully borrowed and modified the data.");
    }

    let mut borrow_b = pointer_b.borrow_mut();
    borrow_b.push(5); 

    println!("Pointer B successfully borrowed the data."); 
}

fn main() {
    solve2();
    // solve1();
    // cause_panic();
}