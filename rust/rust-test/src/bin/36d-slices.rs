
fn main() {
    let chars = vec!['A', 'B', 'C', 'D'];
    match chars.as_slice() {
        [_first, .., _last] => (),
        [_single] => (),
        [] => (),
    }

    let chars = vec!['A', 'B', 'C', 'D'];
    match chars.as_slice() {
        [_one, _two, ..] => (),
        [.., _last] => (),
        [] => (),
    }

    let nums = vec![7, 8, 9];
    match nums.as_slice() {
        [_first @ 1..=3, _rest @ ..] => {
            // 'first' is always 1, 2 or 3
            // 'rest' is the remaining slice
        },
        [single] if single == &5 || single == &6 => (),
        [_a, _b] => (),
        [..] => (),
        //[] => (),
    }
}