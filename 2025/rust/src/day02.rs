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
        let products =
            parse_day_with_preview(2, positive_ints, |text| text.split(",").collect(), 5)?;
        let id_ranges: Vec<(u64, u64)> = products.iter().map(|p| (p[0], p[1])).collect();
        Ok(all_invalids(&id_ranges).iter().sum::<u64>().to_string())
    }
}

use std::collections::HashSet;

fn invalids_in_range(lo: u64, hi: u64, repeat: usize) -> impl Iterator<Item = u64> {
    assert!(lo <= hi);
    assert!(repeat >= 1);

    let lo_s = lo.to_string();
    let prefix_len = lo_s.len() / repeat;

    let first_half: u64 = if prefix_len == 0 {
        1
    } else {
        lo_s[..prefix_len].parse().unwrap()
    };

    (first_half..)
        .map(move |i| {
            let s = i.to_string();
            let mut repeated = String::with_capacity(s.len() * repeat);
            for _ in 0..repeat {
                repeated.push_str(&s);
            }
            repeated.parse::<u64>().unwrap()
        })
        .take_while(move |&id| id <= hi)
        .filter(move |&id| id >= lo)
}

fn invalids(id_ranges: &[(u64, u64)]) -> HashSet<u64> {
    id_ranges
        .iter()
        .flat_map(|&(lo, hi)| invalids_in_range(lo, hi, 2))
        .collect()
}

fn all_invalids(id_ranges: &[(u64, u64)]) -> HashSet<u64> {
    id_ranges
        .iter()
        .flat_map(|&(lo, hi)| {
            let max_repeat = hi.to_string().len();
            (2..=max_repeat).flat_map(move |repeat| invalids_in_range(lo, hi, repeat))
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalids_smoke_test() {
        assert_eq!(invalids(&[(11, 22)]), [11u64, 22u64].into_iter().collect());
    }

    #[test]
    fn invalids_in_range_repeat_5_example() {
        let got: Vec<u64> = invalids_in_range(2_121_212_118, 2_121_212_124, 5).collect();
        assert_eq!(got, vec![2_121_212_121]);
    }

    #[test]
    fn all_invalids_example() {
        let got = all_invalids(&[(11, 22), (95, 115)]);
        let expected: std::collections::HashSet<u64> = [11u64, 22, 99, 111].into_iter().collect();
        assert_eq!(got, expected);
    }
}
