use std::mem::MaybeUninit;

use std::{string, thread};

fn test_multithreading_naive() {
    let mut counter = 0;

    // thread::scope(|s| {
    //     // Add 50000 to the counter in a background thread
    //     let t1 = s.spawn(|| {
    //         for _ in 0..50000 {
    //             counter += 1;
    //         }
    //     });

    //     // Add 50000 to the counter in a background thread
    //     let t2 = s.spawn(|| {
    //         for _ in 0..50000 {
    //             counter += 1;
    //         }
    //     });

    //     // Wait for both threads to finish
    //     t1.join().unwrap();
    //     t2.join().unwrap();

    //     //println!("Result = {counter}");
    // });

}

fn call_closure<C: FnOnce()>(c: C) {
    c();
}

fn test_fn_once_move() {
    let i = "42".to_string();
    let mut strings = Vec::new();
    //capture_i is still FnOnce
    let capture_i = || {
        strings.push(i);
    };

    call_closure(capture_i);

    println!("strings: {:?}", strings);
}

fn main() {
    println!("test_fn_once_move");
    test_fn_once_move();

    println!("Hello, world!");
    let v: Vec<i32> = vec![1,2,3];
    let boxed_slice: Box<[i32]> = vec![1,2,3].into_boxed_slice();
    let array_on_heap: Box<[i32;3]> = vec![1,2,3].into_boxed_slice().try_into().unwrap();
    println!("vec: {:?}, boxed_slice: {:?}, array_on_heap: {:?}", v, boxed_slice, array_on_heap);

    // Allocates space for [i32; 4] directly on the heap
    let mut boxed_maybe: Box<[MaybeUninit<i32>]> = Box::new_uninit_slice(4);
    // Safely initialize the elements on the heap
    let slice = unsafe {
        boxed_maybe[0].as_mut_ptr().write(1);
        boxed_maybe[1].as_mut_ptr().write(2);
        boxed_maybe[2].as_mut_ptr().write(3);
        boxed_maybe[3].as_mut_ptr().write(4);
        boxed_maybe.assume_init()
    };
    println!("slice: {:?}", slice);
}
