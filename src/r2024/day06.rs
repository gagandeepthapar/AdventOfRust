// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::{reader2vecs, DIRECTION};
use std::collections::HashSet;

fn get_guard_orders() -> [DIRECTION; 4] {
    [
        DIRECTION::NORTH,
        DIRECTION::EAST,
        DIRECTION::SOUTH,
        DIRECTION::WEST,
    ]
}

fn read_puzzle(puzzle: &Vec<Vec<char>>) -> ((usize, usize), HashSet<(usize, usize)>) {
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

fn get_unique_path(
    puzzle: &Vec<Vec<char>>,
    guard_pos: &(usize, usize),
    obstacles: &HashSet<(usize, usize)>,
) -> (HashSet<(usize, usize, DIRECTION)>, bool) {
    let guard_orders = get_guard_orders();
    let mut guard_pos = guard_pos.clone();
    let max_row = puzzle.len();
    let max_col = puzzle[0].len();

    let mut traveled: HashSet<(usize, usize, DIRECTION)> = HashSet::new();
    traveled.insert((guard_pos.0, guard_pos.1, DIRECTION::NORTH));

    let mut dir_idx = 0;
    let mut valid_step = true;
    let mut cyc_flag = false;
    let mut test_step;

    while valid_step {
        // Check next step
        (test_step, valid_step) = guard_orders[dir_idx].travel(guard_pos, (max_row, max_col));
        if obstacles.contains(&test_step) {
            dir_idx = (dir_idx + 1) % guard_orders.len();
        }

        // Return if next step exits maze
        if !valid_step {
            return (traveled, cyc_flag);
        }

        // Update Guard Position
        (guard_pos, valid_step) = guard_orders[dir_idx].travel(guard_pos, (max_row, max_col));
        let checkflag = traveled.insert((guard_pos.0, guard_pos.1, guard_orders[dir_idx]));
        cyc_flag = cyc_flag | (!checkflag);

        if cyc_flag {
            return (traveled, cyc_flag);
        }
    }

    // HashSet; Cycle Flag
    (traveled, cyc_flag)
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);

    let (guard_pos, obstacles) = read_puzzle(&puzzle);
    let (traveled, cycflag) = get_unique_path(&puzzle, &guard_pos, &obstacles);
    if cycflag {
        panic!("CYCLE FOUND!");
    }

    let unique_blocks: HashSet<(usize, usize)> = HashSet::from_iter(
        traveled
            .iter()
            .map(|&(coord_x, coord_y, _)| (coord_x, coord_y)),
    );

    Ok(unique_blocks.len() - 1) // subtract one to discount final step exiting maze
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    // Get puzzle in vec format
    let puzzle = reader2vecs(reader);

    // get guard and obstacles
    let (guard_pos, mut obstacles) = read_puzzle(&puzzle);

    // get baseline guard path
    let (guard_path, mut cycle_flag) = get_unique_path(&puzzle, &guard_pos, &obstacles);
    if cycle_flag {
        panic!("CYCLE FOUND IN BASE MAZE")
    }

    let mut unique_path: HashSet<(usize, usize)> = HashSet::from_iter(
        guard_path
            .iter()
            .map(|&(coord_x, coord_y, _)| (coord_x, coord_y)),
    );

    unique_path.remove(&guard_pos);
    let count = unique_path.iter().fold(0, |runopts, &gridpt| {
        obstacles.insert(gridpt);
        (_, cycle_flag) = get_unique_path(&puzzle, &guard_pos, &obstacles);
        obstacles.remove(&gridpt);

        runopts + (cycle_flag as usize)
    });

    Ok(count)
}
