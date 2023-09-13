pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
    let board = MinesweeperBoard::parse(minefield);
    if let Ok(..) = board {
        board.unwrap().to_vec()
    } else {
        Vec::new()
    }
}
// two dimensional minecraft board whose width is the characters in any given line
// and whose height is the number of lines
// origin 0,0 is at upper-left
use std::{collections::HashSet, fmt};
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Mine {
    x: u8,
    y: u8,
}
impl Mine {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}
#[derive(Debug, PartialEq)]
pub struct MinesweeperBoard {
    pub width: u8,
    pub height: u8,
    pub mines: HashSet<Mine>,
}
#[derive(Debug, Clone)]
pub struct MinesweeperBoardError;
impl std::error::Error for MinesweeperBoardError {}
impl fmt::Display for MinesweeperBoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid board dimensions")
    }
}
impl MinesweeperBoard {
    /// Turns an array of string slices into a minesweeper board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use minesweeper::decoupled2::{MinesweeperBoard, Mine};
    /// # use std::collections::HashSet;
    /// let minefield = ["·*·*·", "··*··", "··*··", "·····"];
    /// let mut mines: HashSet<Mine> = HashSet::new();
    /// mines.insert(Mine::new(1,0));
    /// mines.insert(Mine::new(3,0));
    /// mines.insert(Mine::new(2,1));
    /// mines.insert(Mine::new(2,2));
    /// let board = MinesweeperBoard::parse(&minefield).unwrap();
    /// assert_eq!(board, MinesweeperBoard { width: 5u8, height: 4u8, mines });
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if one of the u8 casts fails.
    pub fn parse(minefield: &[&str]) -> Result<MinesweeperBoard, impl std::error::Error> {
        if minefield.is_empty() {
            return Err(MinesweeperBoardError);
        }
        let width: u8 = minefield[0].chars().count().try_into().unwrap();
        let height: u8 = minefield.len().try_into().unwrap();
        if height < 1 {
            return Err(MinesweeperBoardError);
        }
        let mut mines: HashSet<Mine> = HashSet::new();
        for (y, line) in minefield.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '*' {
                    mines.insert(Mine::new(x as u8, y as u8));
                }
            }
        }
        Ok(MinesweeperBoard {
            width,
            height,
            mines,
        })
    }
    /// WARNING: This will always return a number, *even if a mine is in that spot*
    /// Check for mines before using get_cell_value
    ///
    /// ```
    /// # use minesweeper::decoupled2::{MinesweeperBoard, Mine};
    /// # use std::collections::HashSet;
    /// # let minefield = ["·*·*·", "··*··", "··*··", "·····"];
    /// # let mut mines: HashSet<Mine> = HashSet::new();
    /// # mines.insert(Mine::new(1,0));
    /// # mines.insert(Mine::new(3,0));
    /// # mines.insert(Mine::new(2,1));
    /// # mines.insert(Mine::new(2,2));
    /// # let board = MinesweeperBoard::parse(&minefield).unwrap();
    /// assert_eq!(board.get_cell_value(0,0).unwrap(), 1u8);
    /// assert_eq!(board.get_cell_value(2,0).unwrap(), 3u8);
    /// assert_eq!(board.get_cell_value(3,2).unwrap(), 2u8);
    /// assert_eq!(board.get_cell_value(4,2).unwrap(), 0u8);
    /// ```
    pub fn get_cell_value(&self, x: u8, y: u8) -> Result<u8, impl std::error::Error> {
        self.mines
            .iter()
            .filter(|m| {
                (m.x.saturating_sub(1) == x || m.x == x || m.x.saturating_add(1) == x)
                    && (m.y.saturating_sub(1) == y || m.y == y || m.y.saturating_add(1) == y)
            })
            .count()
            .try_into()
    }
    /// Returns the output vector the spec expects.
    /// 
    /// ```
    /// # use minesweeper::decoupled2::{MinesweeperBoard, Mine};
    /// # use std::collections::HashSet;
    /// # let minefield = ["·*·*·", "··*··", "··*··", "·····"];
    /// # let mut mines: HashSet<Mine> = HashSet::new();
    /// # mines.insert(Mine::new(1,0));
    /// # mines.insert(Mine::new(3,0));
    /// # mines.insert(Mine::new(2,1));
    /// # mines.insert(Mine::new(2,2));
    /// # let board = MinesweeperBoard::parse(&minefield).unwrap();
    /// assert_eq!(board.to_vec(), vec!["1*3*1","13*31"," 2*2 ", " 111 "])
    /// ```
    pub fn to_vec(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for y in 0..self.height {
            let mut line: Vec<String> = Vec::new();
            for x in 0..self.width {
                if self.mines.contains(&Mine::new(x, y)) {
                    line.push("*".to_string());
                } else {
                    let num = self.get_cell_value(x, y).unwrap();
                    line.push(match num {
                        0 => " ".to_string(),
                        n @ 1..=8 => format!("{}", n),
                        _ => panic!("impossible cell value"),
                    });
                }
            }
            output.push(line.join(""))
        }
        output
    }
}
impl std::fmt::Display for MinesweeperBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.to_vec();
        let text = board.join("\n");
        write!(f, "{}", text)
    }
}