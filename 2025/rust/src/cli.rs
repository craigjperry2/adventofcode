use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre::{bail, eyre, Result};

use crate::core;

#[derive(Parser, Debug)]
#[command(name = "aoc25", version, about = "Advent of Code 2025 runner")] 
pub struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run a solution for a day
    Run {
        /// Day number (1..=25)
        day: u8,
        /// Part to run (1 or 2). If omitted, runs both if implemented.
        #[arg(value_enum)]
        part: Option<Part>,
        /// Submit the computed answer to AoC (requires --features online)
        #[arg(long)]
        submit: bool,
    },
    /// Fetch and cache the puzzle input for a day
    Fetch {
        /// Day number (1..=25)
        day: u8,
        /// Overwrite any existing cached input
        #[arg(long)]
        force: bool,
    },
    /// Print the path to the input file for a day
    InputPath { day: u8 },
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
enum Part {
    P1,
    P2,
}

impl From<Part> for u8 {
    fn from(p: Part) -> Self {
        match p {
            Part::P1 => 1,
            Part::P2 => 2,
        }
    }
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Run { day, part, submit } => run_day(day, part, submit),
        Command::Fetch { day, force } => fetch(day, force),
        Command::InputPath { day } => {
            println!("{}", core::input_path(day).display());
            Ok(())
        }
    }
}

fn run_day(day: u8, part: Option<Part>, submit: bool) -> Result<()> {
    if !(1..=25).contains(&day) {
        bail!("day must be in 1..=25 (got {day})");
    }

    let solution: &'static dyn core::Solution = match core::solution_for(day) {
        Some(s) => s,
        None => bail!("No solution registered for day {day} yet. Create one and register it in core::solution_for."),
    };

    let input = core::read_or_fetch_input(day)?;

    let run_part = |p: Part| -> Result<String> {
        match p {
            Part::P1 => solution.part1(&input),
            Part::P2 => solution.part2(&input),
        }
    };

    match part {
        Some(p) => {
            let ans = run_part(p)?;
            println!("Day {day} Part {}: {ans}", u8::from(p));
            if submit {
                submit_answer(day, p, &ans)?;
            }
        }
        None => {
            // Try both parts
            if let Ok(ans) = run_part(Part::P1) {
                println!("Day {day} Part 1: {ans}");
                if submit {
                    submit_answer(day, Part::P1, &ans)?;
                }
            }
            if let Ok(ans) = run_part(Part::P2) {
                println!("Day {day} Part 2: {ans}");
                if submit {
                    submit_answer(day, Part::P2, &ans)?;
                }
            }
        }
    }

    Ok(())
}

fn fetch(day: u8, force: bool) -> Result<()> {
    if !(1..=25).contains(&day) { bail!("day must be in 1..=25"); }
    core::ensure_inputs_dir()?;
    let path = core::input_path(day);
    if path.exists() && !force { bail!("Input already exists at {} (use --force to overwrite)", path.display()); }
    let contents = core::fetch_input(day)?;
    std::fs::write(&path, contents)?;
    eprintln!("Saved input to {}", path.display());
    Ok(())
}

fn submit_answer(day: u8, part: Part, answer: &str) -> Result<()> {
    #[cfg(feature = "online")]
    {
        let resp = core::submit(day, u8::from(part), answer)?;
        println!("Submission response: {resp}");
        Ok(())
    }
    #[cfg(not(feature = "online"))]
    {
        Err(eyre!("Submission requires building with --features online"))
    }
}
