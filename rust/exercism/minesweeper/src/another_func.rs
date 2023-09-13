#[inline]
fn is_mine(board: &[&str], i: usize, j: usize) -> bool {
    // let _s: &str = &((*board[i])[j..(j+1)]);
    &(*board[i])[j..(j+1)] == "*"
}

static COEFS: &'static [(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 0), (0, 1),
    (1, -1), (1, 0), (1, 1)];

pub fn annotate(board: &[&str]) -> Vec<String> {
    let mut results: Vec<String> = board.iter().map(|r| r.to_string()).collect();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if is_mine(board, i, j) { continue };
            let c = COEFS.iter()
                .map(|&(x, y)| (x+(i as isize), y+(j as isize)))
                .filter(|&(x, y)| x>=0 && y>=0 && x<board.len() as isize && y<board[i].len() as isize)
                .filter(|&(x, y)| is_mine(board, x as usize, y as usize))
                .count() as u8;
            if c != 0 {
                unsafe {
                    results[i].as_mut_vec()[j] = '0' as u8 + c;
                }
            }
        }
    }
    results
}