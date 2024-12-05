// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

// OPTIONAL

// fn sat(val: i64, lims: (i64, i64)) -> i64 {
//     max(min(val, lims.1), lims.0)
// }

// fn get_neighbors(c_row: i64, c_col: i64, max_row: i64, max_col: i64) -> HashSet<(i64, i64)> {
//     let mut neighbors: HashSet<(i64, i64)> = HashSet::new();
//     let row_lims = (0, max_row - 1);
//     let col_lims = (0, max_col - 1);

//     for ii in -1..=1 {
//         for jj in -1..=1 {
//             let coord = (sat(c_row + ii, row_lims), sat(c_col + jj, col_lims));
//             neighbors.insert(coord);
//         }
//     }

//     neighbors
// }

pub fn part1<R: BufRead>(_reader: R) -> AOCResult<usize> {
    Err(AOCError)
}

pub fn part2<R: BufRead>(_reader: R) -> AOCResult<usize> {
    Err(AOCError)
}
