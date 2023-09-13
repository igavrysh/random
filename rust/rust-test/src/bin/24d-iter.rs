fn main() {
    let numbers = vec![1,2,3,4,5];

    let mut plus_one = vec![];
    for num in &numbers {
        plus_one.push(num+1);
    }

    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num+1)
        .collect();
    println!("plus_one: {:?}", plus_one);

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| *(*num) <= 3)
        .collect();
    println!("new_numbers: {:?}", new_numbers);

    let find_me: Vec<_> = numbers
        .iter()
        .filter(|num| *(*num) <= 3)
        .collect();
    println!("find_me: {:?}", find_me);

    let count = numbers
        .iter()
        .count();
    println!("count: {:?}", count);

    let last = numbers
        .iter()
        .map(|num| *num )
        .last();
    println!("last: {:?}", last);

    let min = numbers
        .iter()
        .min();
    println!("min: {:?}", min);

    let numbers = vec![1,2,3,4,5];

    // let take: Vec<i32> = 
    let take: Vec<i32> = numbers
        .iter()
        .map(|num| *num)
        .take(3)
        .collect();
    println!("take: {:?}", take);

    /*
    let find_me: Option<i32> = numbers
        .iter()
        .find(|num| num == 40);
    */
}