// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct Score {
    powerups: Vec<Powerup>,
    max: i32,
    min: i32,

    current: i32,

    idx: usize,
    idx_powerups: usize,
    prev_idx_powerup_delta_inc: usize,
    delta: i32, 
}

impl Score {
    fn new(powerups: Vec<Powerup>, min: i32, max: i32) -> Self {
        Score {
            powerups,
            min,
            max,

            current: 0,
            idx: 0,
            idx_powerups: 0,
            prev_idx_powerup_delta_inc: 0,
            delta: 1,
        }
    }
}

struct Powerup {
    delta_idx: usize,
    powerup: i32,
}

impl Powerup {
    fn new(delta_idx: usize, powerup: i32) -> Self {
        Powerup { delta_idx, powerup }
    }
}

impl Iterator for Score {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.powerups.get(self.idx_powerups) {
                Some(pu) => {
                    if self.prev_idx_powerup_delta_inc + pu.delta_idx == self.idx {
                        self.delta += pu.powerup;
                        self.prev_idx_powerup_delta_inc = self.idx;
                        self.idx_powerups += 1;
                    } else {
                        break;
                    }
                },
                None => {
                    break;
                }
            };
        }

        self.current += self.delta;

        if self.current < self.min || self.current > self.max  {
            return None
        }

        self.idx += 1;

        Some(self.current)
    }
}

fn test1() {
    /*
    pups        = [ 0,  0, +1,  +3]
                           +1
                           +5
    total pups  = [0,   0, +7,  +3]
    totaldelta  = [+1, +1, +8, +11]
    res         = [1,   2, 10,  21]
     */
    let powerups = vec![
        Powerup::new(2, 1),
        Powerup::new(0, 1),
        Powerup::new(0, 5),
        Powerup::new(1, 3),
    ];
    let min = 1;
    let max = 28;

    let s = Score::new(powerups, min, max);

    for val in s {
        print!("{val} ");
    }

}

fn main() {
    test1()
}
