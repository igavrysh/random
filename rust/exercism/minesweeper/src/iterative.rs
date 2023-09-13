use std::cmp::min;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut output = Vec::new();
    let rows_count = minefield.len();
    for y in 0..rows_count {
        let mut line = String::new();
        let rows_slice = &minefield[y.saturating_sub(1) ..= min(y + 1, rows_count -1)];
        let row = minefield[y];
        let cells_count = row.len();
        for x in 0..cells_count {
            if &row[x..x+1] == "*" {
                line += "*";
                continue;
            }
            let num = rows_slice.iter().fold(0, |num, &line| {
                num + line[x.saturating_sub(1) ..= min(x + 1, cells_count -1)]
                    .match_indices("*")
                    .count()
            });
            if num > 0 {
                line += num.to_string().as_str();
            } else {
                line += " ";
            }
        }
        output.push(line);
    }
    output
}
