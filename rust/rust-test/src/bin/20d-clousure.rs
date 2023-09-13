fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_fn(1, 1);

    let add = |a: i32, b: i32| -> i32 {
        a + b
    };

    let add = |a, b| a + b;

    let sum = add(1, 1);
}