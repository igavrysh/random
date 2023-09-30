#[allow(dead_code)]
enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age: usize,
    species: Species,
}

fn main() {
    let hawk = Bird {
        age: 13, 
        species: Species::Hawk,
    };

    match hawk {
        Bird { age: 4, .. } 
            => println!("4Y old bird"),
        Bird { age: 4..=10 | 15..=20, .. } 
            => println!("4-10Y or 15-20Y old"),
        Bird { species: Species::Finch, .. } 
            => println!("finch!"),
        Bird { .. }
             => println!("other bird"),
    }
} 