// REQUIRED
use crate::utils::{AOCError, AOCResult};
use std::{io::BufRead, usize};

// OPTIONAL
use const_format::concatcp;
use image::{ImageBuffer, Luma, RgbImage};
use regex::Regex;

const GRID_WIDTH: i64 = 101;
const GRID_HEIGHT: i64 = 103;
const P1_TIME: i64 = 100;

fn reader_to_state<R: BufRead>(reader: R) -> Vec<[(i64, i64); 2]> {
    let num_re = Regex::new(r"(\-?\d+),(\-?\d+) v=(\-?\d+),(\-?\d+)").unwrap();
    let robot_states: Vec<[(i64, i64); 2]> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let caps = num_re.captures(&line).unwrap();

            [
                (
                    caps[1].parse::<i64>().unwrap(),
                    caps[2].parse::<i64>().unwrap(),
                ),
                (
                    caps[3].parse::<i64>().unwrap(),
                    caps[4].parse::<i64>().unwrap(),
                ),
            ]
        })
        .collect::<Vec<[(i64, i64); 2]>>();

    robot_states
}

pub fn part1<R: BufRead>(reader: R) -> AOCResult<usize> {
    let robot_states = reader_to_state(reader);

    let xbar = GRID_WIDTH / 2;
    let ybar = GRID_HEIGHT / 2;

    let qs = robot_states.iter().fold([0, 0, 0, 0], |qs, robot| {
        let xf = robot[0].0 + robot[1].0 * P1_TIME;
        let xf = (xf % GRID_WIDTH) + (GRID_WIDTH * (xf % GRID_WIDTH < 0) as i64);
        let yf = robot[0].1 + robot[1].1 * P1_TIME;
        let yf = (yf % GRID_HEIGHT) + (GRID_HEIGHT * (yf % GRID_HEIGHT < 0) as i64);

        let nx = xf < xbar;
        let px = xf > xbar;
        let ny = yf < ybar;
        let py = yf > ybar;

        [
            qs[0] + (nx && ny) as usize,
            qs[1] + (px && ny) as usize,
            qs[2] + (nx && py) as usize,
            qs[3] + (px && py) as usize,
        ]
    });

    let safety_fac = qs.iter().product();

    Ok(safety_fac)
}

pub fn part2<R: BufRead>(reader: R) -> AOCResult<usize> {
    let robots = reader_to_state(reader);
    let christmas_time = 6493;
    let img_dir = std::env::current_dir()
        .unwrap()
        .join(concatcp!(crate::utils::INPT_LOC, "2024/day14imgs/"));

    // let t_start = 6000;
    // let t_max = t_start + 1000;

    // for ii in t_start..=t_max {
    //     let mut img: RgbImage = ImageBuffer::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);

    //     robots.iter().for_each(|robot| {
    //         let xf = robot[0].0 + robot[1].0 * ii;
    //         let xf = (xf % GRID_WIDTH) + (GRID_WIDTH * (xf % GRID_WIDTH < 0) as i64);
    //         let yf = robot[0].1 + robot[1].1 * ii;
    //         let yf = (yf % GRID_HEIGHT) + (GRID_HEIGHT * (yf % GRID_HEIGHT < 0) as i64);

    //         img.put_pixel(xf as u32, yf as u32, image::Rgb([255, 255, 255]));
    //     });

    //     img.save(img_dir.join(format!("{}_time.png", ii))).unwrap();
    // }

    let mut img: RgbImage = ImageBuffer::new(GRID_WIDTH as u32, GRID_HEIGHT as u32);

    robots.iter().for_each(|robot| {
        let xf = robot[0].0 + robot[1].0 * christmas_time;
        let xf = (xf % GRID_WIDTH) + (GRID_WIDTH * (xf % GRID_WIDTH < 0) as i64);
        let yf = robot[0].1 + robot[1].1 * christmas_time;
        let yf = (yf % GRID_HEIGHT) + (GRID_HEIGHT * (yf % GRID_HEIGHT < 0) as i64);

        img.put_pixel(xf as u32, yf as u32, image::Rgb([255, 255, 255]));
    });

    img.save(img_dir.join(format!("{}_time.png", christmas_time)))
        .unwrap();

    Ok(christmas_time as usize)
}
