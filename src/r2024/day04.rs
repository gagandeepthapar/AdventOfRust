// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use crate::aoc_utils::{reader2vecs, wordsearch, DIRECTION};
use std::collections::HashSet;

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let found_list = wordsearch(
        &puzzle,
        "XMAS".to_string(),
        &DIRECTION::all_options().to_vec(),
    );
    Ok(found_list.len())
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let puzzle = reader2vecs(reader);
    let found_list = wordsearch(&puzzle, "MAS".to_string(), &DIRECTION::diags().to_vec());

    let mut center_map: HashSet<(usize, usize)> = HashSet::new();
    let mas_count = found_list.iter().fold(0, |runtot, found| {
        let (center, _) = found.1.travel(found.0, (puzzle.len(), puzzle[0].len()));
        let new_val = center_map.insert(center);
        runtot + (!new_val as usize)
    });

    Ok(mas_count)
}
