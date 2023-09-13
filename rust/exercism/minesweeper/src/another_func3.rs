pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(nrow, row)| {
            row.chars()
                .enumerate()
                .map(|(ncol, ch)| match ch {
                    '*' => '*',
                    _ => annotate_mines(minefield, nrow, ncol),
                })
                .collect()
        })
        .collect()
}

fn annotate_mines(minefield: &[&str], nrow: usize, ncol: usize) -> char {
    let digit = (nrow.saturating_sub(1)..nrow + 2)
        .flat_map(|r| (ncol.saturating_sub(1)..ncol + 2).map(move |c| (r, c)))
        .filter_map(|(r, c)| minefield.get(r).and_then(|row| row.chars().nth(c)))
        .filter(|&ch| ch == '*')
        .count() as u8;
    match digit {
        0 => ' ',
        _ => char::from(digit + '0' as u8),
    }
}
