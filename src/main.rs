use utils::{AOCChallenge, AOCResult};
pub mod aoc_utils;
pub mod r2022;
pub mod r2023;
pub mod r2024;
pub mod utils;

// INPUTS
const YEAR: &str = "2024";
const DAY: &str = "05";
pub use r2024::day05 as today;

// FILES
const TEST1_FILE: &str = const_format::concatcp!("TESTS/", YEAR, "/day", DAY, "/part_1.txt");
const TEST2_FILE: &str = const_format::concatcp!("TESTS/", YEAR, "/day", DAY, "/part_2.txt");
const TEST_SOLN: &str = const_format::concatcp!("TESTS/", YEAR, "/day", DAY, "/test_solns.txt");
const INPT_FILE: &str = const_format::concatcp!("INPUTS/", YEAR, "/day", DAY, ".txt");

// SOLUTIONS
const TEST1_SOLN: Option<usize> = None;
const TEST2_SOLN: Option<usize> = None;

fn main() -> AOCResult<()> {
    let aoc = AOCChallenge::new(TEST1_FILE, TEST2_FILE, INPT_FILE, TEST_SOLN);

    // PART 1 //
    let _ = aoc.part1(TEST1_SOLN)?;

    // PART 2 //
    let _ = aoc.part2(TEST2_SOLN)?;

    // END //
    Ok(())
}
