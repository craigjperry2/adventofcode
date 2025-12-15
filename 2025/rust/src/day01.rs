use crate::core::Solution;
use crate::parsers::{lines, parse_day, parse_day_with_preview};
use color_eyre::eyre::{Result, bail};

pub struct Day01;

impl Solution for Day01 {
    fn part1(&self, input: &str) -> Result<String> {
        let rotations = parse_day_with_preview(1, parse_rotation, lines, 5)?;
        let pointing_at = rotations.iter().scan(50, |state, x| {
            *state += x;
            Some(state.rem_euclid(100))
        });
        let count_zero = pointing_at.filter(|&x| x == 0).count();
        Ok(count_zero.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let rotations = parse_day(1, parse_rotation)?;
        let mut zeros = 0;
        let mut position = 50;
        let dial = 100;
        for r in rotations {
            let (full_circles, remainder) = (r.abs() / dial, r.abs() % dial);
            let distance_to_0 = if r > 0 || position == 0 { dial - position } else { position };
            zeros += full_circles + if remainder >= distance_to_0 { 1 } else { 0 };
            position = (position + r).rem_euclid(dial);
        }
        Ok(zeros.to_string())
    }
}

fn parse_rotation(line: &str) -> i32 {
    let n: i32 = line[1..line.len()].parse().unwrap(); // strip leading L/R and trailing '\n'
    if line[0..1] == *"L" { -n } else { n }
}
