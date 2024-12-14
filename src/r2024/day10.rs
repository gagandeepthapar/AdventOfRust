// REQUIRED
use crate::utils::AOCResult;
use std::{collections::HashSet, io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::{reader2numvecs, DIRECTION};

pub fn dfs_peak(
    puzzle: &Vec<Vec<usize>>,
    coord: (usize, usize),
    val: usize,
    visited: &mut HashSet<(usize, usize)>,
    unique: bool,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    // Base Case
    if puzzle[coord.0][coord.1] == 9 {
        result.push(coord);
    }

    if unique {
        visited.insert(coord);
    }

    let neighbors = DIRECTION::cardinals();
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;

    let (mut nstep, mut valid_step) = (coord, true);
    neighbors.iter().for_each(|step| {
        (nstep, valid_step) = step.travel(coord, (max_row, max_col));
        if valid_step
            && (puzzle[nstep.0][nstep.1] as i64 - val as i64 == 1)
            && !visited.contains(&nstep)
        {
            let mut res = dfs_peak(puzzle, nstep, val + 1, visited, unique);
            result.append(&mut res);
        }
    });

    result
}

fn get_trailheads(puzzle: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let trailheads: Vec<(usize, usize)> = puzzle
        .iter()
        .enumerate() // Get row index and row reference
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate() // Get column index and value
                .filter_map(move |(col_idx, &val)| {
                    if val == 0 {
                        Some((row_idx, col_idx))
                    } else {
                        None
                    }
                })
        })
        .collect();

    trailheads
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2numvecs(reader);
    let trailheads = get_trailheads(&puzzle);
    let trail_score = trailheads.iter().fold(0, |runtot, &thead| {
        let mut hs = HashSet::new();
        let score = dfs_peak(&puzzle, thead, 0, &mut hs, true).len();
        runtot + score
    });

    Ok(trail_score)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2numvecs(reader);
    let trailheads = get_trailheads(&puzzle);
    let trail_score = trailheads.iter().fold(0, |runtot, &thead| {
        let mut hs = HashSet::new();
        let score = dfs_peak(&puzzle, thead, 0, &mut hs, false).len();
        runtot + score
    });

    Ok(trail_score)
}
