// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use regex::Regex;
use std::collections::HashMap;

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let digit_regex = Regex::new(r"[0-9]").unwrap();
    let caldoc = reader.lines().fold(0, |runtot, line| {
        let line = line.unwrap();
        let tens_val = digit_regex.captures(&line).unwrap();

        let bline: String = line.chars().rev().collect();
        let ones_val = digit_regex.captures(&bline).unwrap();

        runtot
            + ((&tens_val[0]).parse::<usize>().unwrap() * 10)
            + ((&ones_val[0]).parse::<usize>().unwrap())
    });

    Ok(caldoc)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let fwd_digit_names = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let back_digit_names = [
        "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
    ];

    let fwd_regex = Regex::new(
        r"(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(zero)|([0-9])",
    )
    .unwrap();

    let back_regex = Regex::new(
        r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)|(orez)|([0-9])",
    )
    .unwrap();

    let mut fwd_map: HashMap<String, usize> = HashMap::from_iter(
        fwd_digit_names
            .iter()
            .enumerate()
            .map(|(idx, &name)| (String::from(name), idx)),
    );

    let mut back_map: HashMap<String, usize> = HashMap::from_iter(
        back_digit_names
            .iter()
            .enumerate()
            .map(|(idx, &name)| (String::from(name), idx)),
    );

    let dig_range = 0..=9;
    dig_range.into_iter().for_each(|val| {
        let strval: String = val.to_string();
        fwd_map.insert(strval.clone(), val);
        back_map.insert(strval, val);
    });

    let caldoc = reader.lines().fold(0, |runtot, line| {
        let line = line.unwrap();
        let fwd_line = line.clone();
        let back_line: String = line.clone().chars().rev().collect();

        let fwd_cap = fwd_regex.captures(&fwd_line).unwrap();
        let back_cap = back_regex.captures(&back_line).unwrap();

        runtot + (fwd_map.get(&fwd_cap[0]).unwrap() * 10) + (back_map.get(&back_cap[0]).unwrap())
    });

    Ok(caldoc)
}
