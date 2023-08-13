fn maybe_num() -> Option<i32> {
    Some(42)
}

fn maybe_word() -> Option<String> {
    Some("Hello World".to_owned())
}

fn main() {
    let plus_one = match maybe_num() {
        Some(num) => Some(num+1),
        None => None,
    };

    let plus_one = maybe_num().map(|a| a+1);

    let word_length = maybe_word()
        .map(|word| word.len())
        .map(|len| len * 2);
}