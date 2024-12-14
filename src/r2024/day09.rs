// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

fn get_taken_free<R: BufRead>(
    reader: R,
) -> (Vec<(usize, usize, usize)>, Vec<(usize, usize, usize)>) {
    let (taken, free): (Vec<(usize, usize, usize)>, Vec<(usize, usize, usize)>) = reader
        .lines()
        .flat_map(|line| {
            line.unwrap()
                .char_indices()
                .map(|(id, ch)| (id / 2, ch.to_digit(10).unwrap() as usize, id))
                .collect::<Vec<(usize, usize, usize)>>()
        })
        .partition(|&(_, _, id)| id % 2 == 0);

    (taken, free)
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    // [(file_id, block_space); (free_space_id, free_space_size)]
    let (mut taken, free) = get_taken_free(reader);
    let mut disk_vec: Vec<usize> = Vec::from_iter((0..taken[0].1).map(|_| taken[0].0));
    let mut backptr = taken.len() - 1;
    for fwdptr in 1..taken.len() {
        // Replace free space
        for _ in 0..free[fwdptr - 1].1 {
            if backptr < fwdptr {
                continue;
            }
            disk_vec.push(taken[backptr].0);
            taken[backptr].1 -= 1;
            if taken[backptr].1 == 0 {
                backptr -= 1;
            }
        }

        // Add in taken space
        for _ in 0..taken[fwdptr].1 {
            disk_vec.push(taken[fwdptr].0);
        }
    }

    let disk_sum = disk_vec
        .iter()
        .enumerate()
        .fold(0, |runtot, (idx, disk_space)| runtot + idx * disk_space);

    Ok(disk_sum)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    // [(file_id, block_space); (free_space_id, free_space_size)]
    let (mut taken, mut free) = get_taken_free(reader);
    let mut disk_vec: Vec<usize> = vec![taken[0].0; taken[0].1];

    free.push((0, 0, 0));
    let mut backptr: usize;
    let mut fwdptr = 1;
    // println!("{:?}", free);
    for freedx in 0..free.len() {
        backptr = taken.len() - 1;
        while free[freedx].1 > 0 && backptr > 0 {
            if taken[backptr].1 <= free[freedx].1 {
                disk_vec.append(&mut vec![taken[backptr].0; taken[backptr].1]);
                free[freedx].1 -= taken[backptr].1;
                // free[backptr - 1].1 += taken[backptr].1 + free[backptr].1;
                // free[backptr].1 = 0;
                taken[backptr].1 = 0;
            }
            backptr -= 1;
        }
        disk_vec.append(&mut vec![0; free[freedx].1]);
        disk_vec.append(&mut vec![taken[fwdptr].0; taken[fwdptr].1]);
        taken[fwdptr].1 = 0;
        fwdptr += 1;
    }

    let disk_sum = disk_vec
        .iter()
        .enumerate()
        .fold(0, |runtot, (idx, dval)| runtot + idx * dval);

    Ok(disk_sum)
}
