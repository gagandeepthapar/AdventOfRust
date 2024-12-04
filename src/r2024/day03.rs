// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use regex::Regex;

fn mulstr_to_product(mulstr: &str) -> usize {
    let nums = mulstr.strip_prefix("mul(").unwrap();
    let nums = nums.strip_suffix(")").unwrap();
    let (num1, num2) = nums.split_once(",").unwrap();
    num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap()
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let mul_regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();

    let mul_val = reader.lines().fold(0, |runtot, line| {
        let line = line.unwrap();

        let caps: Vec<&str> = mul_regex
            .captures_iter(&line)
            .filter_map(|caps| caps.get(1).map(|m| m.as_str()))
            .collect();

        let linesum = caps
            .iter()
            .fold(0, |runtot, mulstr| runtot + mulstr_to_product(mulstr));

        runtot + linesum
    });

    Ok(mul_val)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let mul_regex = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let do_regex = Regex::new(r"(do\(\))").unwrap();
    let dont_regex = Regex::new(r"(don\'t\(\))").unwrap();

    let tot_regex =
        Regex::new(r"(?<mulstr>mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don\'t\(\))").unwrap();

    let mut valid_mul = true;
    let mul_val = reader.lines().fold(0, |runtot, line| {
        let line = line.unwrap();

        let mut line_sum = 0;
        tot_regex.captures_iter(&line).for_each(|cap| {
            let cap_str = cap.get(0).unwrap().as_str();

            if do_regex.is_match(cap_str) {
                valid_mul = true;
            } else if dont_regex.is_match(cap_str) {
                valid_mul = false;
            } else if mul_regex.is_match(cap_str) {
                line_sum += mulstr_to_product(cap_str) * (valid_mul as usize);
            }
        });
        runtot + line_sum
    });

    Ok(mul_val)
}
