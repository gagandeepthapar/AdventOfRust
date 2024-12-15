use clap::Parser;
use utils::AOCResult;
pub mod aoc_utils;
pub mod r2022;
pub mod r2023;
pub mod r2024;
pub mod utils;

// INPUTS
pub use r2024::day14 as today;

fn main() -> AOCResult<()> {
    // Parse Arguments
    let aoc_args = utils::AOCArgs::parse();

    // Process Arguments
    aoc_args.process()?;

    // END //
    Ok(())
}
