// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::{reader2vecs, DIRECTION};
use std::collections::{HashMap, HashSet};

fn get_freq_loc(puzzle: Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antenna_loc: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    puzzle.iter().enumerate().for_each(|(rowidx, row)| {
        row.iter().enumerate().for_each(|(colidx, ch)| {
            if *ch != '.' {
                antenna_loc
                    .entry(*ch)
                    .or_insert_with(Vec::new)
                    .insert(0, (rowidx, colidx));
            }
        });
    });

    antenna_loc
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;
    let antenna_loc = get_freq_loc(puzzle);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    antenna_loc.iter().for_each(|(_, v)| {
        for ii in 0..v.len() {
            for jj in 0..v.len() {
                // Skip same antennas
                if ii == jj {
                    continue;
                }

                // Compute path to antenna
                let dx = v[jj].0 as i64 - v[ii].0 as i64;
                let dy = v[jj].1 as i64 - v[ii].1 as i64;
                let path = DIRECTION::delta_coord_to_dirs((dx, dy));

                // Travel down path to antinode position
                let mut start = v[jj];
                let mut v_step: bool = true;
                let valid_antinode = path.iter().fold(true, |valid_path, dir| {
                    (start, v_step) = dir.travel(start, (max_row, max_col));
                    valid_path && v_step
                });

                // Insert if valid path taken
                if valid_antinode {
                    antinodes.insert(start);
                }
            }
        }
    });

    Ok(antinodes.len())
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;
    let antenna_loc = get_freq_loc(puzzle);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    antenna_loc.iter().for_each(|(_, v)| {
        for ii in 0..v.len() {
            for jj in 0..v.len() {
                // Skip same antennas
                if ii == jj {
                    continue;
                }

                // Compute path to antenna
                let dx = v[jj].0 as i64 - v[ii].0 as i64;
                let dy = v[jj].1 as i64 - v[ii].1 as i64;
                let path = DIRECTION::delta_coord_to_dirs((dx, dy));

                // Travel down path to antinode position
                let mut start = v[jj];
                antinodes.insert(start);

                let mut v_step: bool = true;
                let mut valid_antinode = true;
                while valid_antinode {
                    valid_antinode = path.iter().fold(true, |valid_path, dir| {
                        (start, v_step) = dir.travel(start, (max_row, max_col));
                        valid_path && v_step
                    });

                    // Insert if valid path taken
                    if valid_antinode {
                        antinodes.insert(start);
                    }
                }
            }
        }
    });

    Ok(antinodes.len())
}
