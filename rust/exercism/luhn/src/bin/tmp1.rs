use std::collections::HashMap;

pub fn tmp() {
    let mut v = vec![1,2,3];
    for i in &mut v {
        *i += 40;
    }

    for i in &v {
        println!("{i}");
    }
}

fn main() {
    let mut list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // line below does not work due to mutable borrow in closure and 
    // imutable borrow in println! 
    //println!("abc list: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    tmp()
}