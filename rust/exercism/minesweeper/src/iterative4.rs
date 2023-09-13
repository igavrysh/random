pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // function to update the adj vec for a mine at the passed location, note that
    // 9 is used to encode a mine
    fn mine_found(av: &mut Vec<Vec<u8>>, row: usize, col: usize) {
        for (r, mrow) in av.iter_mut().enumerate().skip(row).take(3) {
            for (c, mcol) in mrow.iter_mut().enumerate().skip(col).take(3) {
                if r == row + 1 && c == col + 1 {
                    *mcol = 9;
                } else if *mcol != 9 {
                    *mcol += 1;
                }
            }
        }
    }
    // create a adjacency-counting vector large enough so that you will never annotate over the edge
    let rows = minefield.len();
    let cols = if rows > 0 { minefield[0].len() } else { 0 };
    let mut adjvec = vec![vec![0; cols + 2]; rows + 2];
    // go through the input minefield setting adjvec as mines are found
    for (rn, mfrow) in minefield.iter().enumerate().take(rows) {
        for (cn, ch) in mfrow.chars().enumerate() {
            if ch == '*' {
                mine_found(&mut adjvec, rn, cn);
            }
        }
    }
    // go through adjvec creating the output
    adjvec[1..=rows]
        .iter()
        .map(|r| {
            r[1..=cols]
                .iter()
                .map(|n| match n {
                    0 => ' ',
                    9 => '*',
                    x => (x + b'0') as char,
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}