
fn main() {
    let chars = vec!['A', 'B', 'C', 'D'];
    match chars.as_slice() {
        [first, .., last] => (),
        [single] => (),
        [] => (),
    }

    let chars = vec!['A', 'B', 'C', 'D'];
    match chars.as_slice() {
        [one, two, ..] => (),
        [.., last] => (),
        [] => (),
    }

    let nums = vec![7, 8, 9];
    match nums.as_slice() {
        [first @ 1..=3, rest @ ..] => {
            // 'first' is always 1, 2 or 3
            // 'rest' is the remaining slice
        },
        [single] if single == &5 || single == &6 => (),
        [a, b] => (),
        [..] => (),
        [] => (),
    }
}