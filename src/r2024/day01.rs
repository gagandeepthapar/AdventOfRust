// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use std::collections::HashMap;

fn list_to_vecs<R: BufRead>(reader: R) -> (Vec<usize>, Vec<usize>) {
    let mut vecs = [Vec::<usize>::new(), Vec::<usize>::new()];
    reader.lines().for_each(|line| {
        line.unwrap()
            .split_whitespace()
            .enumerate()
            .for_each(|(ii, val)| {
                let v = val.parse::<usize>().unwrap();
                vecs[ii].push(v);
            });
    });

    (vecs[0].clone(), vecs[1].clone())
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let (mut v1, mut v2) = list_to_vecs(reader);
    v1.sort();
    v2.sort();

    let tot_dist = v1
        .iter()
        .zip(v2.iter())
        .fold(0, |tot, (&a, &b)| tot + a.abs_diff(b));

    Ok(tot_dist)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let (v1, v2) = list_to_vecs(reader);
    let mut sim_list = HashMap::<usize, usize>::new();

    v2.iter().for_each(|&val| {
        let curr = sim_list.get(&val);
        sim_list.insert(val, {
            match curr {
                Some(curval) => curval + 1,
                None => 1,
            }
        });
    });

    let tot_sim = v1.iter().fold(0, |runtot, &val| {
        runtot + {
            match sim_list.get(&val) {
                Some(curval) => val * curval,
                None => 0,
            }
        }
    });

    Ok(tot_sim)
}
