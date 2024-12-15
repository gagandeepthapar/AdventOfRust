// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::{reader2vecs, DIRECTION};
use std::collections::HashSet;

fn flood_fill_dfs(
    current: (usize, usize),
    puzzle: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    remaining: &mut HashSet<(usize, usize)>,
    target: char,
) {
    let states = DIRECTION::cardinals();
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;

    visited.insert(current);
    remaining.remove(&current);
    for path in states {
        let (newstate, valid_step) = path.travel(current, (max_row, max_col));
        if valid_step && !visited.contains(&newstate) && puzzle[newstate.0][newstate.1] == target {
            flood_fill_dfs(newstate, puzzle, visited, remaining, target);
        }
    }
}

fn get_crops(puzzle: &Vec<Vec<char>>) -> Vec<HashSet<(usize, usize)>> {
    let max_row = puzzle.len();
    let max_col = puzzle[0].len();

    let mut remaining: HashSet<(usize, usize)> = (0..max_row)
        .flat_map(|rid| (0..max_col).map(move |cid| (rid, cid)))
        .collect();

    let mut crop_set: Vec<HashSet<(usize, usize)>> = vec![HashSet::new()];
    let mut crop_id = 0;

    puzzle.iter().enumerate().for_each(|(rid, row)| {
        row.iter().enumerate().for_each(|(cid, ch)| {
            if remaining.contains(&(rid, cid)) {
                crop_set.push(HashSet::new());
                crop_id += 1;
                flood_fill_dfs(
                    (rid, cid),
                    &puzzle,
                    &mut crop_set[crop_id],
                    &mut remaining,
                    *ch,
                );
            }
        })
    });

    crop_set[1..].to_vec()
}

fn get_edges(
    puzzle: &Vec<Vec<char>>,
    crop_field: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;
    let edges: HashSet<(usize, usize)> =
        HashSet::from_iter(crop_field.iter().filter_map(|&crop_pos| {
            let mut edge_coord = false;
            for step in DIRECTION::all_options() {
                let (newstep, validstep) = step.travel(crop_pos, (max_row, max_col));

                edge_coord = edge_coord
                    || (puzzle[newstep.0][newstep.1] != puzzle[crop_pos.0][crop_pos.1]
                        || !validstep)
            }

            if edge_coord {
                Some(crop_pos)
            } else {
                None
            }
        }));

    edges
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let crop_set = get_crops(&puzzle); // [(Perim, Area)]
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;

    let neighbors = DIRECTION::cardinals();
    let fence_params: Vec<(usize, usize)> = Vec::from_iter(crop_set.iter().map(|crop| {
        // Perimter = 4 * area minus number of shared sides
        // Compute # of shared sides
        let shared_sides = crop.iter().fold(0, |runtot, &single_crop| {
            let mut addend = 0;
            neighbors.iter().for_each(|step| {
                let (newstep, valid_step) = step.travel(single_crop, (max_row, max_col));
                addend += (valid_step && crop.contains(&newstep)) as usize
            });
            runtot + addend
        });

        // Store perim and area
        (4 * crop.len() - shared_sides, crop.len())
    }));

    let fence_cost = fence_params
        .iter()
        .fold(0, |runtot, crop_fence| runtot + crop_fence.0 * crop_fence.1);

    Ok(fence_cost)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let crop_set = get_crops(&puzzle); // [(Perim, Area)]
    let max_row = puzzle.len() - 1;
    let max_col = puzzle[0].len() - 1;

    for idx in 0..crop_set.len() {
        let edges0 = get_edges(&puzzle, &crop_set[idx]);
        let blankrow = (0..=max_col).map(|_| '.').collect::<Vec<char>>();
        let mut blankslate = (0..=max_row)
            .map(|_| blankrow.clone())
            .collect::<Vec<Vec<char>>>();

        for edge in edges0 {
            blankslate[edge.0][edge.1] = puzzle[edge.0][edge.1];
        }
        for row in blankslate {
            println!("{:?}", row);
        }
        println!("\n");
    }
    // println!("{:?}", edges0.len());
    // println!("{:?}", crop_set[0].len());

    // let neighbors = DIRECTION::cardinals();
    // // Number of Sides == Number of Corners (definition)
    // let fence_params: Vec<(usize, usize)> = Vec::from_iter(crop_set.iter().map(|cropfield| {
    //     let a = 1;
    //     let b = 2;
    //     (a, b)
    // }));

    // let fence_cost = fence_params
    //     .iter()
    //     .fold(0, |runtot, crop_fence| runtot + crop_fence.0 * crop_fence.1);

    // Ok(fence_cost)
    // println!("{:?}", fence_cost);

    Err(AOCError)
}
