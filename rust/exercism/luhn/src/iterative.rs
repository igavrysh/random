fn double(n: u32) -> u32 {
    if n * 2 > 9 {
        n * 2 - 9
    } else {
        n * 2
    }
}
pub fn is_valid(input: &str) -> bool {
    let trimmed = input.replace(" ", "");
    if trimmed.len() < 2 || !trimmed.chars().all(char::is_numeric) {
        return false
    }
    trimmed
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)|
            if i % 2 == 1 {
                double(n)
            } else {
                n
            }
        )
        .sum::<u32>() % 10 == 0
}