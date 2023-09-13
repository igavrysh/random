#[allow(dead_code)]
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let  rows = minefield.len();
    let mut res: Vec<String> = Vec::with_capacity(minefield.len());
    if rows == 0 {
        return res
    }
    let cols = minefield[0].len();
    if cols == 0 {
        res.push(String::from(""));
        return res;
    } 

    let mut line: &[u8] = minefield[0].as_bytes();
    let mut prev_line: &[u8] = &[];
    let mut next_line: &[u8]= &[];
    if rows > 1 { next_line = minefield[1].as_bytes() };

    let dirs = [
        [-1,-1],[-1,0],[-1,1], 
        [0,-1],[0, 1],
        [1,-1],[1,0],[1,1]
    ];

    const MINE_CH: char = '*';
    const MINE: u8 = MINE_CH as u8;

    for i in 0..minefield.len() {
        let mut res_line = String::from("");
        for j in 0..line.len() {
            if line[j] == MINE {
                res_line += &MINE_CH.to_string();
            } else {
                let mut mines = 0;
                for d in dirs {
                    let new_i = i as i32 + d[0];
                    let new_j = j as i32 + d[1];
                    if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
                        mines = mines + 
                        if new_i == i as i32 -1 {
                            if prev_line[new_j as usize] == MINE { 1 } else { 0 }
                        } else if new_i == i as i32  {
                            if line[new_j as usize] == MINE { 1 } else { 0 }
                        } else if new_i == i as i32 + 1 {
                            if next_line[new_j as usize] == MINE { 1 } else { 0 }
                        } else {
                            0
                        }
                    }
                }
                if mines > 0 {
                    res_line += &mines.to_string();
                } else {
                    res_line += &String::from(" ")
                }
             }
        }

        res.push(res_line);

        prev_line = line;
        line = next_line;
        if i+2 < rows {
            next_line = minefield[i + 2].as_bytes()
        }
    }

    res
}