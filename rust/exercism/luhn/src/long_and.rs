pub fn is_valid(serial: &str) -> bool {
    // Only valid digits
    serial.chars().all(|c| c.is_digit(10) || c.is_whitespace()) &&
    // More than one digit
    serial.chars().filter(|c| c.is_digit(10)).count() > 1 &&
    // Has valid checksum
    serial
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)| if i % 2 == 0 { n } else { 2 * n } )
        .map(|n| if n > 9 { n - 9 } else { n } )
        .sum::<u32>() % 10 == 0
}
