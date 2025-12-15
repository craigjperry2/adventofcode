use crate::core::Solution;
use crate::parsers::{parse_day_with_preview, positive_ints};
use color_eyre::eyre::Result;

pub struct Day02;

impl Solution for Day02 {
    fn part1(&self, input: &str) -> Result<String> {
        let products =
            parse_day_with_preview(2, positive_ints, |text| text.split(",").collect(), 5)?;
        let id_ranges: Vec<(u64, u64)> = products.iter().map(|p| (p[0], p[1])).collect();
        Ok(invalids(&id_ranges).iter().sum::<u64>().to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        todo!()
    }
}

use std::collections::HashSet;

fn invalids_in_range(lo: u64, hi: u64) -> impl Iterator<Item = u64> {
    assert!(lo <= hi);

    let lo_s = lo.to_string();
    let half_len = (lo_s.len() / 2).max(1);
    let first_half: u64 = lo_s[..half_len].parse().unwrap();

    (first_half..)
        .map(|i| {
            let s = i.to_string();
            let doubled = format!("{s}{s}");
            doubled.parse::<u64>().unwrap()
        })
        .take_while(move |&id| id <= hi)
        .filter(move |&id| id >= lo)
}

fn invalids(id_ranges: &[(u64, u64)]) -> HashSet<u64> {
    id_ranges
        .iter()
        .flat_map(|&(lo, hi)| invalids_in_range(lo, hi))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalids_smoke_test() {
        assert_eq!(invalids(&[(11, 22)]), [11u64, 22u64].into_iter().collect());
    }
}
