// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::{reader2vecs, sat, DIRECTION};
use std::collections::{HashMap, HashSet};

fn get_guard_orders() -> [DIRECTION; 4] {
    [
        DIRECTION::NORTH,
        DIRECTION::EAST,
        DIRECTION::SOUTH,
        DIRECTION::WEST,
    ]
}

fn read_puzzle(puzzle: Vec<Vec<char>>) -> ((usize, usize), HashSet<(usize, usize)>) {
    let mut guard_pos: (usize, usize) = (0, 0);
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    puzzle.iter().enumerate().for_each(|(rowidx, row)| {
        row.iter().enumerate().for_each(|(colidx, &ch)| {
            if ch == '#' {
                obstacles.insert((rowidx, colidx));
            } else if ch == '^' {
                guard_pos = (rowidx, colidx);
            }
        });
    });

    (guard_pos, obstacles)
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let max_row = puzzle.len();
    let max_col = puzzle[0].len();

    let (mut guard_pos, obstacles) = read_puzzle(puzzle);
    let guard_orders = get_guard_orders();

    let mut traveled: HashSet<(usize, usize)> = HashSet::new();
    traveled.insert(guard_pos);

    let mut dir_idx = 0;
    let mut valid_step = true;
    while valid_step {
        (guard_pos, valid_step) = guard_orders[dir_idx].travel(guard_pos, (max_row, max_col));
        if obstacles.contains(&guard_pos) {
            (guard_pos, _) = guard_orders[dir_idx]
                .opposite()
                .travel(guard_pos, (max_row, max_col));
            dir_idx += 1;
            dir_idx = dir_idx % guard_orders.len();
        }

        traveled.insert(guard_pos);
    }

    Ok(traveled.len() - 1) // subtract one to discount final step exiting maze
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;

    let (mut guard_pos, mut obstacles) = read_puzzle(puzzle);
    let guard_orders = get_guard_orders();

    /*
    A-----------B
    |           |
    |           |
    D-----------C
    */

    let (mut invalid_row, mut invalid_col) = (true, true);
    let (mut corner_a, mut corner_b, mut corner_c) = ((0, 0), (0, 0), (0, 0));

    let infloop_counter = obstacles.iter().fold(0, |runtot, corner_d| {
        (corner_c.0, invalid_row) = sat(corner_d.0, (0, max_row));
        (corner_a.1, invalid_col) = sat(corner_d.1, (0, max_col));

        let rcount = (0..corner_d.0 - 1).collect::<Vec<usize>>();
        let ccount = (corner_d.1 + 1..max_col).collect::<Vec<usize>>();

        0
    });

    println!("{:?}", infloop_counter);
    Err(AOCError)
}
