fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let _sum = add_fn(1, 1);

    let _add = |a: i32, b: i32| -> i32 {
        a + b
    };

    let add = |a, b| a + b;

    let _sum = add(1, 1);
}