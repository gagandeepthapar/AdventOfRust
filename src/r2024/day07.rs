// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

fn num_in_base(num: usize, base: usize) -> String {
    let mut rev_string: String = String::new();
    let (mut quot, mut rem) = (num, 0);

    (0..32).rev().for_each(|idx| {
        (quot, rem) = (quot / base, quot % base);
        rev_string.push_str(&rem.to_string());
    });

    let fwd_string = rev_string.chars().rev().collect::<String>();
    fwd_string.to_string()
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let calsum = reader.lines().fold(0, |runtot, line| {
        let cal_line = line.unwrap();
        let (total, compstr) = cal_line.split_once(": ").unwrap();
        let total = total.parse::<usize>().unwrap();
        let components: Vec<usize> = compstr
            .split(" ")
            .map(|numstr| numstr.parse::<usize>().unwrap())
            .collect();

        let n_opts = (2 as usize).pow(components.len() as u32 - 1);
        let valid_option = (0..n_opts).fold(false, |curr_valid, n| {
            let st = format!("{:032b}", n);
            let (_, relstr) = st.split_at(32 - components.len() + 1);
            let chs = relstr.chars().collect::<Vec<char>>();

            let check_sum =
                components[1..]
                    .iter()
                    .zip(chs.iter())
                    .fold(components[0], |runtot, (comp, ch)| match ch {
                        '0' => runtot + comp,
                        '1' => runtot * comp,
                        _ => panic!("INVALID"),
                    });

            curr_valid | (check_sum == total)
        });

        runtot + (total * valid_option as usize)
    });

    Ok(calsum)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let calsum: u128 = reader.lines().fold(0, |runtot, line| {
        let cal_line = line.unwrap();
        let (total, compstr) = cal_line.split_once(": ").unwrap();
        let total = total.parse::<usize>().unwrap();
        let components: Vec<usize> = compstr
            .split(" ")
            .map(|numstr| numstr.parse::<usize>().unwrap())
            .collect();

        let n_opts = (3 as usize).pow(components.len() as u32 - 1);
        let valid_option = (0..n_opts).fold(false, |curr_valid, n| {
            let st = num_in_base(n, 3);
            let (_, relstr) = st.split_at(32 - components.len() + 1);

            let chs = relstr.chars().collect::<Vec<char>>();

            let check_sum =
                components[1..]
                    .iter()
                    .zip(chs.iter())
                    .fold(components[0], |runtot, (comp, ch)| match ch {
                        '0' => runtot + comp,
                        '1' => runtot * comp,
                        '2' => {
                            // let bdigs = ((*comp as f64).log10()).ceil() as u32;
                            // runtot * (10 as usize).pow(bdigs) + comp
                            let mut current = runtot.to_string();
                            current.push_str(&comp.to_string());
                            current.parse::<usize>().unwrap()
                        }
                        _ => panic!("INVALID"),
                    });

            curr_valid | (check_sum == total)
        });

        runtot + (total * valid_option as usize) as u128
    });

    Ok(calsum as usize)
}
