// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

// OPTIONAL
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

fn sat(val: i64, lims: (i64, i64)) -> i64 {
    max(min(val, lims.1), lims.0)
}

fn get_neighbors(c_row: i64, c_col: i64, max_row: i64, max_col: i64) -> HashSet<(i64, i64)> {
    let mut neighbors: HashSet<(i64, i64)> = HashSet::new();
    let row_lims = (0, max_row - 1);
    let col_lims = (0, max_col - 1);

    for ii in -1..=1 {
        for jj in -1..=1 {
            let coord = (sat(c_row + ii, row_lims), sat(c_col + jj, col_lims));
            neighbors.insert(coord);
        }
    }

    neighbors
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let num_regex = Regex::new(r"([0-9]*)*").unwrap();
    let invalid_syms: HashSet<char> =
        HashSet::from(['.', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);

    let nums: HashSet<char> = HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']);

    reader.lines().enumerate().for_each(|(rownum, line)| {});

    Err(AOCError)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    Err(AOCError)
}
