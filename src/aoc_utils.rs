use std::array;
use std::cmp::{max, min};
use std::{io::BufRead, usize};

#[derive(Debug, Clone, Copy)]
pub enum DIRECTION {
    NORTH,
    NORTHEAST,
    EAST,
    SOUTHEAST,
    SOUTH,
    SOUTHWEST,
    WEST,
    NORTHWEST,
}

impl DIRECTION {
    pub fn all_options() -> [DIRECTION; 8] {
        [
            DIRECTION::NORTH,
            DIRECTION::NORTHEAST,
            DIRECTION::EAST,
            DIRECTION::SOUTHEAST,
            DIRECTION::SOUTH,
            DIRECTION::SOUTHWEST,
            DIRECTION::WEST,
            DIRECTION::NORTHWEST,
        ]
    }

    pub fn cardinals() -> [DIRECTION; 4] {
        [
            DIRECTION::NORTH,
            DIRECTION::EAST,
            DIRECTION::WEST,
            DIRECTION::SOUTH,
        ]
    }

    pub fn diags() -> [DIRECTION; 4] {
        [
            DIRECTION::NORTHEAST,
            DIRECTION::SOUTHEAST,
            DIRECTION::SOUTHWEST,
            DIRECTION::NORTHWEST,
        ]
    }

    pub fn opposite(&self) -> DIRECTION {
        match self {
            DIRECTION::NORTH => DIRECTION::SOUTH,
            DIRECTION::NORTHEAST => DIRECTION::SOUTHWEST,
            DIRECTION::EAST => DIRECTION::WEST,
            DIRECTION::SOUTHEAST => DIRECTION::NORTHWEST,
            DIRECTION::SOUTH => DIRECTION::NORTH,
            DIRECTION::SOUTHWEST => DIRECTION::NORTHEAST,
            DIRECTION::WEST => DIRECTION::EAST,
            DIRECTION::NORTHWEST => DIRECTION::SOUTHEAST,
        }
    }

    pub fn travel(
        &self,
        current: (usize, usize),
        bottom_right: (usize, usize),
    ) -> ((usize, usize), bool) {
        let vert_trav: i64 = {
            match self {
                Self::NORTH | Self::NORTHEAST | Self::NORTHWEST => -1,
                Self::EAST | Self::WEST => 0,
                Self::SOUTH | Self::SOUTHEAST | Self::SOUTHWEST => 1,
            }
        };

        let horiz_trav: i64 = {
            match self {
                Self::NORTHWEST | Self::WEST | Self::SOUTHWEST => -1,
                Self::NORTH | Self::SOUTH => 0,
                Self::NORTHEAST | Self::EAST | Self::SOUTHEAST => 1,
            }
        };

        let (nrow, rb) = sat((current.0 as i64 + vert_trav) as usize, (0, bottom_right.0));
        let (ncol, cb) = sat(
            (current.1 as i64 + horiz_trav) as usize,
            (0, bottom_right.1),
        );

        // TRUE -> VALID STEP
        // FALSE -> INVALID STEP
        ((nrow, ncol), !(rb | cb))
    }
}

pub fn reader2vecs<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    let puzzle: Vec<Vec<char>> = reader
        .lines()
        .map(|line_result| {
            let line_result = line_result.unwrap();
            line_result.chars().collect::<Vec<char>>() // Convert each line to Vec<char>
        })
        .collect();

    puzzle
}
pub fn sat(val: usize, lims: (usize, usize)) -> (usize, bool) {
    let nval = min(max(val, lims.0), lims.1);

    // TRUE -> SATURATED
    // FALSE -> ORIGINAL
    (nval, nval != val)
}

pub fn wordsearch(
    puzzle: &Vec<Vec<char>>,
    word: String,
    valid_dir: &Vec<DIRECTION>,
) -> Vec<((usize, usize), DIRECTION)> {
    let mut found_words: Vec<((usize, usize), DIRECTION)> = Vec::new();
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;
    let wordlen = word.len();
    let wordchars = word.chars().collect::<Vec<char>>();

    puzzle.iter().enumerate().for_each(|(rowidx, row)| {
        row.iter().enumerate().for_each(|(colidx, &ch)| {
            if ch == wordchars[0] {
                for step in valid_dir {
                    let mut valid_match = true;
                    let (mut newstep, mut valid_step) = ((rowidx, colidx), true);
                    for wordidx in 1..wordlen {
                        if valid_match {
                            (newstep, valid_step) = step.travel(newstep, (max_row, max_col));
                            valid_match = valid_match
                                && valid_step
                                && puzzle[newstep.0][newstep.1] == wordchars[wordidx];
                        }
                    }
                    if valid_match {
                        found_words.push(((rowidx, colidx), *step));
                    }
                }
            }
        });
    });

    found_words
}
