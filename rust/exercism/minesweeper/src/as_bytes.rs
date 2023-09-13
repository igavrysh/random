use std::collections::HashSet;

const BOMB: u8 = '*' as u8;

pub fn row_to_string(bombs: &HashSet<(usize, usize)>, y: usize, cells: &Vec<u8>) -> String {
    let mut row = String::new();
    for (x, &count) in cells.iter().enumerate() {
        if bombs.contains(&(y, x)) {
            row.push(char::from(BOMB));
        } else {
            if count > 0 {
                row.push(char::from_digit(count as u32, 10).unwrap());
            } else {
                row.push(' ');
            }
        }
    }
    row
}

pub fn display_annotated_minefield(
    bombs: &HashSet<(usize, usize)>,
    counts_field: Vec<Vec<u8>>,
) -> Vec<String> {
    return counts_field
        .iter()
        .enumerate()
        .map(|(y, cells)| row_to_string(bombs, y, cells))
        .collect::<Vec<String>>();
}

/**
 * Annotate each square of the given minefield with the number of mines
 * that surround said square (blank if there are no surrounding mines): minefield
 */
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut row_bytes: &[u8];
    let mut counts_field = minefield
        .iter()
        .map(|row| vec!(0u8; row.as_bytes().len()))
        .collect::<Vec<Vec<u8>>>();
    let mut bombs: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in minefield.iter().enumerate() {
        row_bytes = row.as_bytes();
        for (x, cell) in row_bytes.iter().enumerate() {
            if *cell == BOMB {
                bombs.insert((y, x));
                if y > 0 {
                    if x > 0 {
                        counts_field[y - 1][x - 1] += 1;
                    }
                    if x + 1 < counts_field[y - 1].len() {
                        counts_field[y - 1][x + 1] += 1;
                    }
                    counts_field[y - 1][x] += 1;
                }
                if y + 1 < minefield.len() {
                    if x > 0 {
                        counts_field[y + 1][x - 1] += 1;
                    }
                    if x + 1 < counts_field[y + 1].len() {
                        counts_field[y + 1][x + 1] += 1;
                    }
                    counts_field[y + 1][x] += 1;
                }
                if y + 1 > 0 && y < minefield.len() {
                    if x > 0 {
                        counts_field[y][x - 1] += 1;
                    }
                    if x + 1 < counts_field[y].len() {
                        counts_field[y][x + 1] += 1;
                    }
                }
            }
        }
    }
    display_annotated_minefield(&bombs, counts_field)
}