// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{collections::HashMap, io::BufRead, u128};

const BLINK_P1: u128 = 25;
const BLINK_P2: u128 = 75;

fn get_start_pos<R: BufRead>(reader: R) -> Vec<u128> {
    let nline = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    nline
}

fn even_digits(num: u128) -> bool {
    num.to_string().len() % 2 == 0
}

fn split_digits(num: u128) -> (u128, u128) {
    let num = num.to_string();
    let (a, b) = num.split_at(num.to_string().len() / 2);
    (a.parse::<u128>().unwrap(), b.parse::<u128>().unwrap())
}

fn dfs_apply_rules(
    val0: u128,
    blink_count: u128,
    blink_cap: u128,
    cache: &mut HashMap<(u128, u128), u128>,
) -> u128 {
    // Returns # of stones
    if blink_count == blink_cap {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(val0, blink_count)) {
        return cached_result;
    }

    let result = match val0 {
        0 => dfs_apply_rules(1, blink_count + 1, blink_cap, cache),
        num if even_digits(num) => {
            let (left, right) = split_digits(num);
            dfs_apply_rules(left, blink_count + 1, blink_cap, cache)
                + dfs_apply_rules(right, blink_count + 1, blink_cap, cache)
        }
        _ => dfs_apply_rules(val0 * 2024, blink_count + 1, blink_cap, cache),
    };

    cache.insert((val0, blink_count), result);
    result
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let nline = get_start_pos(reader);
    let total = nline.iter().fold(0, |runtot, &stone_num| {
        let mut cache: HashMap<(u128, u128), u128> = HashMap::new();
        runtot + dfs_apply_rules(stone_num, 0, BLINK_P1, &mut cache)
    });

    Ok(total as usize)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let nline = get_start_pos(reader);
    let total = nline.iter().fold(0, |runtot, &stone_num| {
        let mut cache: HashMap<(u128, u128), u128> = HashMap::new();
        runtot + dfs_apply_rules(stone_num, 0, BLINK_P2, &mut cache)
    });

    Ok(total as usize)
}
