use color_eyre::eyre::{bail, eyre, Result};
use once_cell::sync::Lazy;
use std::fs;
use std::path::{Path, PathBuf};

pub trait Solution: Sync + Send {
    fn part1(&self, _input: &str) -> Result<String> {
        bail!("Part 1 not implemented for this day")
    }
    fn part2(&self, _input: &str) -> Result<String> {
        bail!("Part 2 not implemented for this day")
    }
}

use crate::{day01, day02, day03, day04, day05, day06};
static DAY01: day01::Day01 = day01::Day01;
static DAY02: day02::Day02 = day02::Day02;
static DAY03: day03::Day03 = day03::Day03;
static DAY04: day04::Day04 = day04::Day04;
static DAY05: day05::Day05 = day05::Day05;
static DAY06: day06::Day06 = day06::Day06;

pub fn solution_for(day: u8) -> Option<&'static dyn Solution> {
    match day {
        1 => Some(&DAY01),
        2 => Some(&DAY02),
        3 => Some(&DAY03),
        4 => Some(&DAY04),
        5 => Some(&DAY05),
        6 => Some(&DAY06),
        _ => None,
    }
}

pub fn year() -> u16 {
    2025
}

static INPUTS_DIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("inputs"));

pub fn ensure_inputs_dir() -> Result<()> {
    fs::create_dir_all(&*INPUTS_DIR)?;
    Ok(())
}

pub fn input_path(day: u8) -> PathBuf {
    let fname = format!("day{day:02}.txt");
    INPUTS_DIR.join(fname)
}

pub fn read_or_fetch_input(day: u8) -> Result<String> {
    let path = input_path(day);
    if path.exists() {
        Ok(fs::read_to_string(path)?)
    } else {
        #[cfg(feature = "online")]
        {
            ensure_inputs_dir()?;
            let contents = fetch_input(day)?;
            fs::write(&path, &contents)?;
            Ok(contents)
        }
        #[cfg(not(feature = "online"))]
        {
            bail!(
                "Input file not found at {}. Run `aoc25 fetch {day}` or build with `--features online` to auto-fetch.",
                path.display()
            )
        }
    }
}

#[cfg(feature = "online")]
fn session_cookie() -> Result<String> {
    let sess = std::env::var("AOC_SESSION").map_err(|_| eyre!(
        "AOC_SESSION env var not set. Copy your 'session' cookie from https://adventofcode.com in your browser and set it, e.g.\n  export AOC_SESSION=...\nConsider using direnv to manage it locally."
    ))?;
    Ok(format!("session={sess}"))
}

#[cfg(feature = "online")]
fn client() -> Result<reqwest::blocking::Client> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("aoc25-rust (https://github.com/craigjperry2)")
        .build()?;
    Ok(client)
}

#[cfg(feature = "online")]
pub fn fetch_input(day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year(), day);
    let resp = client()?
        .get(url)
        .header(reqwest::header::COOKIE, session_cookie()?)
        .send()?;
    if !resp.status().is_success() {
        bail!("Failed to fetch input: HTTP {}", resp.status());
    }
    Ok(resp.text()?)
}

#[cfg(not(feature = "online"))]
pub fn fetch_input(_day: u8) -> Result<String> {
    bail!("Online fetch is disabled. Rebuild with --features online")
}

#[cfg(feature = "online")]
pub fn submit(day: u8, part: u8, answer: &str) -> Result<String> {
    if part != 1 && part != 2 {
        bail!("part must be 1 or 2");
    }
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year(), day);
    let resp = client()?
        .post(url)
        .header(reqwest::header::COOKIE, session_cookie()?)
        .form(&[("level", part.to_string()), ("answer", answer.to_string())])
        .send()?;
    if !resp.status().is_success() {
        bail!("Failed to submit: HTTP {}", resp.status());
    }
    Ok(resp.text()?)
}

#[cfg(not(feature = "online"))]
pub fn submit(_day: u8, _part: u8, _answer: &str) -> Result<String> {
    bail!("Online submission is disabled. Rebuild with --features online")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_for_day_formats() {
        assert_eq!(input_path(1), Path::new("inputs/day01.txt"));
        assert_eq!(input_path(12), Path::new("inputs/day12.txt"));
    }

    #[test]
    fn day1_is_registered() {
        assert!(solution_for(1).is_some());
    }
}
