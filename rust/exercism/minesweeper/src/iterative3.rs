use std::char;
use std::cmp;
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // grab dimensions of array
    let h = minefield.len();
    if h == 0 { return Vec::new(); } // empty minefield
    let w = minefield[0].len(); // assuming every str has the same length
    if w == 0 {
        let mut ans = Vec::with_capacity(h);
        ans.resize(h,String::new());
        return ans;
    }
    // copy mines into a more easily indexable form
    let grid : Vec<Vec<char>> = minefield.iter().map(|row| row.chars().collect()).collect();
    // count adjacent mines
    let count_mines = | x : usize, y : usize | -> char {
        let x0 = match x { 0 => 0, n => n - 1 };
        let y0 = match y { 0 => 0, n => n - 1 };
        let x1 = cmp::min(x+1,w-1);
        let y1 = cmp::min(y+1,h-1);
        let mut count = 0;
        for c in x0 .. x1+1 {
            for r in y0 .. y1+1 {
                count += match grid[r][c] { '*' => 1, _ => 0 };
            }
        }
        match char::from_digit(count,10) {
            None => panic!("Couldn't express count as single digit!"),
            Some('0') => ' ',
            Some(d) => d
        }
    };
    // build the output
    minefield
        .iter().enumerate()
        .map(|(y,&row)|
             row.char_indices()
             .map(|(x,ch)| match ch { '*' => '*', _ => count_mines(x,y) })
             .collect())
        .collect()
}
