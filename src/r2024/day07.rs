// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

fn check_ops(target: usize, comps: &[usize], cat_flag: bool) -> bool {
    let mut opstack: Vec<(usize, usize)> = Vec::new();

    // Initial stack option
    opstack.push((comps.len() - 1, target));

    while let Some((ii, target)) = opstack.pop() {
        let num = comps[ii];

        // Only single option
        if ii == 0 {
            if num == target {
                return true;
            }
            continue;
        }

        // Num greater than taret
        if num > target {
            continue;
        }

        // Add
        opstack.push((ii - 1, target - num));

        // Mul
        if target % num == 0 {
            opstack.push((ii - 1, target / num));
        }

        // Cat
        if cat_flag {
            let targetstr = target.to_string();
            let numstr = num.to_string();
            if targetstr.ends_with(&numstr) {
                opstack.push((ii - 1, target / 10usize.pow(numstr.len() as u32)))
            }
        }
    }

    false
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let calsum = reader.lines().fold(0, |runtot, line| {
        let line = line.unwrap();
        let (target, compstr) = line.split_once(": ").unwrap();
        let target = target.parse::<usize>().unwrap();
        let components = compstr
            .split_whitespace()
            .map(|cs| cs.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        runtot + (target * check_ops(target, &components, false) as usize)
    });
    Ok(calsum)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let calsum = reader.lines().fold(0, |runtot, line| {
        let line = line.unwrap();
        let (target, compstr) = line.split_once(": ").unwrap();
        let target = target.parse::<usize>().unwrap();
        let components = compstr
            .split_whitespace()
            .map(|cs| cs.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        runtot + (target * check_ops(target, &components, true) as usize)
    });

    Ok(calsum)
}
