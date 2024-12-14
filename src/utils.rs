use crate::today;
use chrono::Datelike;
use clap::{ArgAction, Parser};
use core::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::{env, fs};

// CONSTS
const TEST_LOC: &str = "TESTS/";
const INPT_LOC: &str = "INPUTS/";

const PART1: &str = "part_1.txt";
const PART2: &str = "part_2.txt";
const TSOLN: &str = "test_solns.txt";
const TEMPLATE: &str = "src/template.rs";

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

    pub fn part1(&self, day: usize, test_solution: Option<usize>) -> AOCResult<usize> {
        // TEST
        let reader = AOCChallenge::read_file(&self.test_file_a);
        let (test, test_dur) = AOCChallenge::timeit(reader, today::part1);
        let test = test?;

        AOCChallenge::disp_sol(&day.to_string(), "1 (TEST)", test, test_dur);
        assert_eq!(test, test_solution.unwrap_or(self.test_soln[0]));

        // ACTUAL
        let reader = AOCChallenge::read_file(&self.input_file);
        let (p1, actual_dur) = AOCChallenge::timeit(reader, today::part1);
        let p1 = p1?;

        AOCChallenge::disp_sol(&day.to_string(), "1", p1, actual_dur);

        Ok(p1)
    }

    pub fn part2(&self, day: usize, test_solution: Option<usize>) -> AOCResult<usize> {
        // TEST
        let reader = AOCChallenge::read_file(&self.test_file_b);
        let (test, test_dur) = AOCChallenge::timeit(reader, today::part2);
        let test = test?;

        AOCChallenge::disp_sol(&day.to_string(), "2 (TEST)", test, test_dur);
        assert_eq!(test, test_solution.unwrap_or(self.test_soln[1]));

        // ACTUAL
        let reader = AOCChallenge::read_file(&self.input_file);
        let (p2, actual_dur) = AOCChallenge::timeit(reader, today::part2);
        let p2 = p2?;

        AOCChallenge::disp_sol(&day.to_string(), "2", p2, actual_dur);

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

/// Simple Args to Create or Run AOC
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AOCArgs {
    /// Relevant Year for Advent of Code. Defaults to current year.
    #[arg(short = 'y', long)]
    year: Option<usize>,

    /// Relevant Year for Advent of Code. Defaults to current 1.
    #[arg(short = 'd', long)]
    day: Option<usize>,

    /// Create specfied Advent of Code Snippet
    #[arg(long,action=ArgAction::SetTrue)]
    create: bool,

    /// Run specified Advent of Code Snippet
    #[arg(long,action=ArgAction::SetTrue)]
    run: bool,
}

impl AOCArgs {
    pub fn process(&self) -> AOCResult<()> {
        // Set default day/year
        let current_date = chrono::Local::now();
        let _yr = self.year.unwrap_or(current_date.year() as usize);
        let _day = self.day.unwrap_or(1);

        // Welcome Splash
        println!("\n>>>>>> Advent of Code {} - Day {} <<<<<<", _yr, _day);

        // Process Args
        if self.create {
            println!("Creating structure for Year {} - Day {:02}\n", _yr, _day);
            self.create(_yr, _day)?
        }

        if self.run && !self.create {
            println!("Running Year {} - Day {:02}\n", _yr, _day);
            self.run(_yr, _day)?
        }
        Ok(())
    }

    fn create(&self, year: usize, day: usize) -> AOCResult<()> {
        // Get Cargo level
        let pwd = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // Test and Input directories
        let new_dirs = [
            pwd.join(TEST_LOC).join(format!("{}/day{:02}", year, day)), // Test
            pwd.join(INPT_LOC).join(format!("{}", year)),               // Inputs
        ];

        // Create new dirs
        new_dirs
            .iter()
            .for_each(|dir| fs::create_dir_all(&dir).unwrap());

        // Create new files
        let new_files = [
            &new_dirs[0].join(PART1),
            &new_dirs[0].join(PART2),
            &new_dirs[0].join(TSOLN),
            &new_dirs[1].join(format!("day{:02}.txt", day)),
        ];
        new_files.iter().for_each(|path| {
            if !path.exists() {
                fs::File::create(path).unwrap();
            }
        });

        // Create new rs file
        let code_path = pwd.join("src").join(format!("r{}", year));
        if !code_path.exists() {
            fs::create_dir_all(&code_path).unwrap();
        }

        // Update mod.rs file
        let mod_path = code_path.join("mod.rs");

        // Create if doesn't exist
        if !mod_path.exists() {
            fs::File::create(&mod_path).unwrap();
        }
        let code_path = code_path.join(format!("day{:02}.rs", day));

        // Copy template data to new file
        if !code_path.exists() {
            fs::File::create(&code_path).unwrap();
            fs::copy(pwd.join(TEMPLATE), code_path).unwrap();

            // Append new file to mod.rs file
            let mut mod_file = OpenOptions::new().append(true).open(mod_path).unwrap();
            let newline = format!("pub mod day{:02};", day);
            if let Err(e) = writeln!(mod_file, "{}", newline) {
                eprintln!("Couldn't write to modrs file: {}", e);
            }
        }

        Ok(())
    }

    fn run(&self, year: usize, day: usize) -> AOCResult<()> {
        // Get relevant dirs and files
        let pwd = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let aoc_dirs = [
            pwd.join(TEST_LOC).join(format!("{}/day{:02}", year, day)), // Test
            pwd.join(INPT_LOC).join(format!("{}", year)),               // Inputs
        ];

        // Extract relevant files
        let tf1 = aoc_dirs[0].join(PART1);
        let tf2 = aoc_dirs[0].join(PART2);
        let tsol = aoc_dirs[0].join(TSOLN);
        let iptf = aoc_dirs[1].join(format!("day{:02}.txt", day));

        let aoc = AOCChallenge::new(
            tf1.to_str().unwrap(),
            tf2.to_str().unwrap(),
            iptf.to_str().unwrap(),
            tsol.to_str().unwrap(),
        );

        // PART 1 //
        let _ = aoc.part1(day, None)?;

        // PART 2 //
        let _ = aoc.part2(day, None)?;

        Ok(())
    }
}
