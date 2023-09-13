pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let offset:Vec<_> = [-1,-1,-1,0,0,1,1,1].iter().zip([-1,0,1,-1,1,-1,0,1]).collect();
    let width = if minefield.len() > 0 { minefield[0].len() as i32 } else { 0 };
    let height = minefield.len() as i32;
    minefield.iter().enumerate().map(|(i, r)| {
        r.char_indices().map(|(j, c)| { match c {
            '*' => c,
             _ => match offset.iter().map(|&(n, m)| (i as i32 + n, j as i32 + m))
                .filter(|&(n, m)| n >= 0 && n < height && m >= 0 && m < width)
                .filter(|&(n, m)| minefield[n as usize].as_bytes()[m as usize] == b'*')
                .count() {
                    0 => ' ',
                    mines => (mines as u8 + '0' as u8) as char,
                }
        }}).collect()
    }).collect()
}
