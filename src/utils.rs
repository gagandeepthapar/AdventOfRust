use crate::today;
use core::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

pub type AOCResult<T> = Result<T, AOCError>;

#[derive(Debug, Clone)]
pub struct AOCError;
impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed AOC Challenge")
    }
}

pub struct AOCChallenge {
    test_file_a: String,
    test_file_b: String,
    input_file: String,
    test_soln: [usize; 2],
}

impl AOCChallenge {
    pub fn new(test_file_a: &str, test_file_b: &str, input_file: &str, test_soln: &str) -> Self {
        println!(
            "\n>>>>>> Advent of Code {} - Day {} <<<<<<\n",
            super::YEAR,
            super::DAY
        );

        let reader = AOCChallenge::read_file(test_soln);
        let mut solns = [0, 0];
        reader
            .lines()
            .enumerate()
            .for_each(|(idx, sol)| solns[idx] = sol.unwrap().parse::<usize>().unwrap());

        AOCChallenge {
            test_file_a: String::from(test_file_a),
            test_file_b: String::from(test_file_b),
            input_file: String::from(input_file),
            test_soln: solns,
        }
    }

    pub fn part1(&self, test_solution: Option<usize>) -> AOCResult<usize> {
        // TEST
        let reader = AOCChallenge::read_file(&self.test_file_a);
        let (test, test_dur) = AOCChallenge::timeit(reader, today::part1);
        let test = test?;

        AOCChallenge::disp_sol(super::DAY, "1 (TEST)", test, test_dur);
        assert_eq!(test, test_solution.unwrap_or(self.test_soln[0]));

        // ACTUAL
        let reader = AOCChallenge::read_file(&self.input_file);
        let (p1, actual_dur) = AOCChallenge::timeit(reader, today::part1);
        let p1 = p1?;

        AOCChallenge::disp_sol(super::DAY, "1", p1, actual_dur);

        Ok(p1)
    }

    pub fn part2(&self, test_solution: Option<usize>) -> AOCResult<usize> {
        // TEST
        let reader = AOCChallenge::read_file(&self.test_file_b);
        let (test, test_dur) = AOCChallenge::timeit(reader, today::part2);
        let test = test?;

        AOCChallenge::disp_sol(super::DAY, "2 (TEST)", test, test_dur);
        assert_eq!(test, test_solution.unwrap_or(self.test_soln[1]));

        // ACTUAL
        let reader = AOCChallenge::read_file(&self.input_file);
        let (p2, actual_dur) = AOCChallenge::timeit(reader, today::part2);
        let p2 = p2?;

        AOCChallenge::disp_sol(super::DAY, "2", p2, actual_dur);

        Ok(p2)
    }

    fn timeit<R: BufRead>(
        reader: R,
        func: fn(R) -> AOCResult<usize>,
    ) -> (AOCResult<usize>, Duration) {
        let start = Instant::now();
        let sol = func(reader);
        let end = Instant::now();

        (sol, end.duration_since(start))
    }

    fn read_file(fname: &str) -> BufReader<File> {
        BufReader::new(File::open(fname).unwrap())
    }

    fn disp_sol(day: &str, part: &str, sol: usize, time: Duration) {
        println!(
            ">>> Day {} Part {} Solution ({} us):\n{}\n",
            day,
            part,
            time.as_micros(),
            sol,
        );
    }
}
