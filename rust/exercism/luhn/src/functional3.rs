pub fn is_valid(number: &str) -> bool {
    number.matches(char::is_numeric).count() > 1 &&
    number.chars()
        .rev()
        .filter(|&c| c != ' ')
        .map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, d)| {
            d.map(|d| {
                let r = d * ((i as u32 % 2) + 1);
                if r > 9 { r - 9 } else { r }
            })
        })
        .fold(Some(0), |acc, x| acc.and_then(|acc| x.map(|x| acc + x)))
        .map(|x| x % 10 == 0)
        .unwrap_or(false)
}