// REQUIRED
use crate::{
    aoc_utils::sat,
    utils::{AOCError, AOCResult},
};
use std::{env::consts::ARCH, io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::reader2vecs;
use std::collections::{HashMap, HashSet};

fn get_freq_loc(puzzle: Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut antenna_loc: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    puzzle.iter().enumerate().for_each(|(rowidx, row)| {
        row.iter().enumerate().for_each(|(colidx, ch)| {
            if *ch != '.' {
                antenna_loc
                    .entry(*ch)
                    .or_insert_with(HashSet::new)
                    .insert((rowidx, colidx));
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
    println!("{:?}", antenna_loc);

    let (mut newnode, nodeflag) = ((0, 0), true);
    let antinode_list: HashSet<(usize, usize)> = HashSet::new();
    antenna_loc.keys().map(|key| {
        let vals = antenna_loc.get(&key).unwrap();
        vals.iter().for_each(|val1| {
            vals.iter().for_each(|val2| {
                if (val1.0 != val2.0) && (val1.1 != val2.1) {
                    let dr = val2.0 - val1.0;
                    let dc = val2.1 - val1.1;

                    let (nndode1_r,_) = sat(val2.0 + dr, (0, max_row));
                    let nndode1_c = sat(val2.1 + dc, (0, max_col));
                    let nndode2_r = sat(val1.0 - dr, (0, max_row));
                    let nndode2_c = sat(val1.1 - dc, (0, max_col));

                    antinode_list.insert((nnode1_r, nnode1_c));
                    antinode_list.insert((nnode2_r, nnode2_c));
                }
            });
        })
    });

    Err(AOCError)
}

pub fn part2<R: BufRead>(_reader: R) -> AOCResult<usize> {
    Err(AOCError)
}
