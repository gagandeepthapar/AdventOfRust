// REQUIRED
use crate::utils::AOCResult;
use std::cmp;
use std::{io::BufRead, usize};

// OPTIONAL
const MAX_VALS: [usize; 3] = [12, 13, 14];

fn game_list_to_min_cubes<R: BufRead>(reader: R) -> Vec<[usize; 3]> {
    let min_cubes: Vec<[usize; 3]> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            // SPLIT ID (ID | SET 1; SET 2; SET 3)
            let (_, game) = line.split_once(": ").unwrap();

            // SPLIT SETS (SET 1 | SET 2 | SET 3)
            let mut picked_colors: [usize; 3] = [0, 0, 0];
            game.split("; ").for_each(|set| {
                // SPLIT INTO COLORS
                set.split(", ").for_each(|color_combo| {
                    let (qty, color) = color_combo.split_once(" ").unwrap();

                    // PARSE QTY
                    let qty = qty.parse::<usize>().unwrap();

                    // ASSIGN COLOR TO QTY
                    match color {
                        "red" => picked_colors[0] = cmp::max(picked_colors[0], qty),
                        "green" => picked_colors[1] = cmp::max(picked_colors[1], qty),
                        "blue" => picked_colors[2] = cmp::max(picked_colors[2], qty),
                        _ => panic!("Invalid Color"),
                    }
                });
            });
            picked_colors
        })
        .collect();
    min_cubes
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let min_cubes = game_list_to_min_cubes(reader);
    let bad_id_sum = min_cubes
        .iter()
        .enumerate()
        .fold(0, |runtot, (idx, color_list)| {
            let valid_set = color_list
                .iter()
                .zip(MAX_VALS.iter())
                .fold(true, |curr_valid, (picked, max)| {
                    curr_valid && (picked <= max)
                });

            runtot + (valid_set as usize) * (idx + 1)
        });

    Ok(bad_id_sum)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let min_cubes = game_list_to_min_cubes(reader);
    let cube_power = min_cubes.iter().fold(0, |runtot, color_list| {
        runtot + (color_list[0] * color_list[1] * color_list[2])
    });

    Ok(cube_power)
}
