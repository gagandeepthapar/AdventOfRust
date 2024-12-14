// REQUIRED
use crate::utils::AOCResult;
use std::{io::BufRead, usize};

// OPTIONAL
use regex::Regex;

const A_COST: i64 = 3;
const B_COST: i64 = 1;
const TGT_BIAS: i64 = 10000000000000;

fn reader_to_arcade<R: BufRead>(reader: R) -> Vec<[(i64, i64); 3]> {
    let mut arcade_vec: Vec<[(i64, i64); 3]> = Vec::new();
    let mut machine_opts: [(i64, i64); 3] = [(0, 0); 3];
    let button_re =
        Regex::new(r"(Button|Prize)(\:| A\:| B\:) X(\+|\=)(\d+), Y(\+|\=)(\d+)").unwrap();

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        if let Some(capdata) = button_re.captures(&line) {
            let x_opt = capdata[4].parse::<i64>().unwrap();
            let y_opt = capdata[6].parse::<i64>().unwrap();
            let cidx = match &capdata[2] {
                " A:" => 0,
                " B:" => 1,
                ":" => 2,
                _ => panic!("STOP"),
            };

            machine_opts[cidx] = (x_opt, y_opt);
        } else {
            arcade_vec.push(machine_opts.clone());
        }
    });
    arcade_vec.push(machine_opts.clone());

    arcade_vec
}

fn solveaxequalsb(A: &[i64; 4], B: &[i64; 2]) -> ([i64; 2], bool) {
    // 2x2 integer linear system of equations
    let mut def = ([0 as i64; 2], false);
    let [a, b, c, d] = A;
    let [e, f] = B;

    // Compute determinant for inverse
    let det = a * d - b * c;
    def.1 = det == 0;
    if def.1 {
        // Early return for no solution
        return def;
    }

    let ainvb = [d * e - b * f, a * f - c * e];
    def.1 = (ainvb[0] % det != 0) || (ainvb[1] % det != 0);
    if def.1 {
        // Early return if non-integer solution
        return def;
    }

    // Solve
    def.0 = [ainvb[0] / det, ainvb[1] / det];
    def
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let arcade_vec = reader_to_arcade(reader);
    let pot_win = arcade_vec.iter().fold(0, |runtot, &claw| {
        let [(a, c), (b, d), (e, f)] = claw;
        let ([a_count, b_count], _) = solveaxequalsb(&[a, b, c, d], &[e, f]);

        runtot + (A_COST * a_count + B_COST * b_count)
    });

    Ok(pot_win as usize)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let arcade_vec = reader_to_arcade(reader);
    let pot_win = arcade_vec.iter().fold(0, |runtot, &claw| {
        let [(a, c), (b, d), (e, f)] = claw;
        let ([a_count, b_count], _) = solveaxequalsb(&[a, b, c, d], &[e + TGT_BIAS, f + TGT_BIAS]);

        runtot + (A_COST * a_count + B_COST * b_count)
    });

    Ok(pot_win as usize)
}
