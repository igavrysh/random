pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.char_indices()
                .map(|(col_index, x)| match x {
                    '*' => '*',
                    _ => count_bombs(minefield, row_index, col_index),
                })
                .collect::<String>()
        })
        .collect()
}

fn count_bombs(minefield: &[&str], row_index: usize, col_index: usize) -> char {
    let mut count = 0;
    let row_checking_range = checking_range(row_index, minefield.len() - 1);
    let col_checking_range = checking_range(col_index, minefield[0].len() - 1);
    for row in minefield.get(row_checking_range).unwrap() {
        for x in row.get(col_checking_range.clone()).unwrap().chars() {
            if x == '*' {
                count += 1;
            }
        }
    }
    match count {
        0 => ' ',
        _ => std::char::from_digit(count, 10).unwrap(),
    }
}

fn checking_range(index: usize, limit: usize) -> std::ops::RangeInclusive<usize> {
    index.saturating_sub(1)..=std::cmp::min(index + 1, limit)
}
