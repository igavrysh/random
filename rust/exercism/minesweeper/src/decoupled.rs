#[derive(Debug, Clone)]
enum Field {
    Mine,
    Wall,
    Empty(u32),
}

struct Row {
    data: Vec<Field>,
}

struct Map {
    data: Vec<Row>,
}

impl Row {
    fn new(size: usize, wall: bool) -> Self {
        if wall {
            Row { data: vec![Field::Wall; size] }
        } else {
            let mut r = Row { data: vec![Field::Empty(0); size] };
            r.data[0] = Field::Wall;
            r.data[size-1] = Field::Wall;
            r
        }
    }

    fn inc(&mut self, at: usize) {
        if let Field::Empty(x) = self.data[at] {
            self.data[at] = Field::Empty(x+1);
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 1..self.data.len()-1 {
            match self.data[i] {
                Field::Mine => s.push('*'),
                Field::Wall => {},
                Field::Empty(0) => s.push(' '),
                Field::Empty(x) => s.push_str(&x.to_string()),
            };
        }
        s
    }

    fn mark_mine(&mut self, at: usize) {
        self.data[at] = Field::Mine;
    }
}

impl Map {
    // minefield has outer walls to not have to deal with bounds checking for
    // edge mines, we can just check if its a wall and not increment it.
    fn new(rows: usize, cols: usize) -> Self {
        let cols = cols+2;
        let mut data : Vec<Row> = Vec::new();
        data.push(Row::new(cols, true));
        for _ in 0..rows {
            data.push(Row::new(cols, false))
        }
        data.push(Row::new(cols, true));
        Map {
            data: data,
        }
    }

    // increment all neighbours of the specified field
    fn inc_neighbours(&mut self, row: usize, col: usize) {
        // account for the extra edges
        let row = row+1;
        let col = col+1;
        self.data[row].mark_mine(col);
        for r in row-1..=row+1 {
            for c in col-1..=col+1 {
                self.data[r].inc(c);
            }
        }
    }

    fn to_printable(&self) -> Vec<String> {
        let mut v = Vec::new();
        // need to skip the top and bottom wall row
        for r in 1..self.data.len()-1 {
            v.push(self.data[r].to_string());
        }
        v
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = if rows == 0 { 0 } else { minefield[0].len() };
    let mut map = Map::new(rows, cols);
    for r in 0..rows {
        minefield[r]
            .chars()
            .enumerate()
            .for_each(|(idx, c)|
                if c == '*' {
                    map.inc_neighbours(r, idx)
                }
            );
    }

    map.to_printable()
}