fn adj<'a>(map: &'a Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    // This isn't the prettiest function.  Oh well.
    let mut count = 0;
    let x_min = if row == 0 { 0 } else {row - 1 };
    let y_min = if col == 0 { 0 } else {col - 1 };
    let x_max = if row + 1 == map.len() { row } else { row + 1 };
    let y_max = if col + 1 == map[0].len() { col } else { col + 1 };
    for r in x_min..x_max+1 {
        for c in y_min..y_max+1 {
            if map[r][c] == '*'  && (r, c) != (row, col) {
                count = count + 1;
            }
        }
    }
    count
}

pub fn annotate<'a>(map: &'a Vec<&str>) -> Vec<String> {
    let mut marked : Vec<Vec<char>> = Vec::new();
    for s in map {
        marked.push(s.chars().collect());
    }
    for row in 0..marked.len() {
        for col in 0..marked[0].len() {
            if marked[row][col] != '*' {
                let c = std::char::from_digit(adj(&marked, row, col), 10)
                    .expect("adj returned None");
                match c {
                    '0' => marked[row][col] = ' ',
                    _ => marked[row][col] = c
                };
            }
        }
    }

    marked.iter().map(|v| {
        (*v).iter().map(|c| *c).collect::<String>()
    }).collect::<Vec<String>>()
}