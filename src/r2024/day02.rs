// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use ndarray::{s, Array1};
const MIN_DIFF: i64 = 1;
const MAX_DIFF: i64 = 3;

fn get_diff_row(abs_arr: &Array1<i64>) -> Array1<i64> {
    &abs_arr.slice(s![1..]) - &abs_arr.slice(s![..-1])
}

fn text_to_array(text: String) -> Array1<i64> {
    let rep_arr: Array1<i64> =
        Array1::from_iter(text.split_whitespace().map(|ch| ch.parse::<i64>().unwrap()));

    rep_arr
}

fn reader_to_diffs<R: BufRead>(reader: R) -> Vec<Array1<i64>> {
    let diffs: Vec<Array1<i64>> = reader
        .lines()
        .map(|line| {
            let rep_arr = text_to_array(line.unwrap());
            get_diff_row(&rep_arr)
        })
        .collect();
    diffs
}

fn check_row(diff_arr: &Array1<i64>) -> bool {
    let mut mono_inc = true;
    let mut mono_dec = true;

    let valid = diff_arr.iter().fold(true, |curr_flag, &diff_val| {
        let sign = diff_val.signum();

        mono_inc = mono_inc && (sign == 1);
        mono_dec = mono_dec && (sign == -1);
        let monotonic = mono_inc ^ mono_dec;

        let bound_flag = (MIN_DIFF <= (diff_val * sign)) && ((diff_val * sign) <= MAX_DIFF);

        // Current Flag && increasing ^ decreasng && !0 && within bounds
        curr_flag && monotonic && bound_flag && (sign != 0)
    });

    valid
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let diff_vec = reader_to_diffs(reader);

    let valid_rep_count = diff_vec.iter().fold(0, |runtot, diff_line| {
        let valid_rep = check_row(diff_line);
        runtot + (valid_rep as usize)
    });

    Ok(valid_rep_count)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let valid_rep_count = reader.lines().fold(0, |runtot, line| {
        let arr = text_to_array(line.unwrap());
        let diff_line = get_diff_row(&arr);
        let check_valid = check_row(&diff_line);

        let valid_rep =
            {
                if check_valid {
                    check_valid
                } else {
                    (0..arr.len()).into_iter().fold(false, |curr_flag, ii| {
                        let mod_arr = Array1::from_iter(arr.iter().enumerate().filter_map(
                            |(idx, &diff_val)| if idx == ii { None } else { Some(diff_val) },
                        ));
                        let mod_diff_line = get_diff_row(&mod_arr);
                        curr_flag || check_row(&mod_diff_line.to_owned())
                    })
                }
            };

        runtot + (valid_rep as usize)
    });

    Ok(valid_rep_count)
}
