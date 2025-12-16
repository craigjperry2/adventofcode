use crate::core::Solution;
use crate::parsers::{lines, parse_text_with_preview, positive_ints};
use color_eyre::eyre::Result;

pub struct Day05;

impl Solution for Day05 {
    fn part1(&self, input: &str) -> Result<String> {
        let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();
        let fresh_ranges = parse_text_with_preview(fresh_ranges, positive_ints, lines, 5)
            .iter()
            .map(|r| (r[0], r[1]))
            .collect::<Vec<(u64, u64)>>();
        let ingredients: Vec<u64> = parse_text_with_preview(ingredients, positive_ints, lines, 5)
            .iter()
            .flatten()
            .copied()
            .collect();
        let answer = count_fresh_ingredients(fresh_ranges.as_slice(), ingredients.as_slice());
        Ok(answer.to_string())
    }

    fn part2(&self, input: &str) -> Result<String> {
        let (fresh_ranges, _) = input.split_once("\n\n").unwrap();
        let fresh_ranges = parse_text_with_preview(fresh_ranges, positive_ints, lines, 5)
            .iter()
            .map(|r| (r[0], r[1]))
            .collect::<Vec<(u64, u64)>>();
        let answer = count_fresh_ids(fresh_ranges);
        Ok(answer.to_string())
    }
}

/// How many of the available ingredient IDs are in one of the fresh ranges?
pub fn count_fresh_ingredients(
    fresh_ranges: &[(u64, u64)],
    available_ingredient_ids: &[u64],
) -> usize {
    available_ingredient_ids
        .iter()
        .filter(|&&id| fresh_ranges.iter().any(|&(lo, hi)| lo <= id && id <= hi))
        .count()
}

pub fn count_fresh_ids(mut fresh_ranges: Vec<(u64, u64)>) -> u64 {
    // How many IDs are contained in the union of the ranges?
    fresh_ranges.sort_unstable_by_key(|&(lo, _hi)| lo);

    let mut fresh_count: u64 = 0; // The number of fresh IDs found so far
    let mut unexplored: u64 = 0; // The highest ID number that we haven't considered yet

    for (lo, hi) in fresh_ranges {
        let start = unexplored.max(lo);

        if start <= hi {
            // inclusive range length = hi - start + 1
            fresh_count += hi - start + 1;
        }

        // next unexplored is one past the end of what we've now covered
        unexplored = unexplored.max(hi.saturating_add(1));
    }

    fresh_count
}
