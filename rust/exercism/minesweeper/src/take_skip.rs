#[allow(dead_code)]
pub fn annotate(input: &[&str]) -> Vec<String> {
    if input.is_empty() {
        return vec![];
    }
    if input[0].is_empty() {
        return vec![String::new()];
    }
    let width = input[0].len();
    let mut result = Vec::new();
    for (y, &line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '*' {
                result.push('*');
                continue;
            }
            let neighbor_mines = input
                .iter()
                .take(y + 2)
                .skip((y as isize - 1).max(0) as usize)
                .flat_map(|&line| {
                    line.chars()
                        .take(x + 2)
                        .skip((x as isize - 1).max(0) as usize)
                })
                .filter(|&chr| chr == '*')
                .count();
            if neighbor_mines > 0 {
                result.push(format!("{}", neighbor_mines).chars().nth(0).unwrap());
            } else {
                result.push(' ');
            }
        }
    }
    result.chunks(width).map(String::from_iter).collect()
}

