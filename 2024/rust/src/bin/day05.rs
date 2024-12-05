use aoc24::read_day_input;
use std::iter::Map;
use std::str::Lines;
use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

fn main() {
    let input = read_day_input(5);
    let (rules, updates) = input.split_once("\n\n").expect("malformed input");

    let parsed_rules: HashMap<&str, HashSet<&str>> = rules
        .lines()
        .map(|l| {
            l.split_once('|')
                .unwrap_or_else(|| panic!("failed to parse rule {l}"))
        })
        .fold(HashMap::new(), |mut map, (b, a)| {
            map.entry(b).or_default().insert(a);
            map
        });

    run_part(1, part1(updates, &parsed_rules));
    run_part(2, part2(updates, &parsed_rules));
}

fn run_part<'a>(day: i32, filtered: Vec<Vec<&str>>) {
    let result: i32 = filtered
        .iter()
        .map(|u| {
            u[u.len().div(2)]
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("Failed to parse {e} as i32"))
        })
        .sum();
    println!("Part {day}: {result}");
}

fn part1<'a>(updates: &'a str, parsed_rules: &HashMap<&str, HashSet<&str>>) -> Vec<Vec<&'a str>> {
    parse_updates(updates)
        .filter(|u| in_order(parsed_rules, u))
        .collect()
}

fn parse_updates(updates: &str) -> Map<Lines, fn(&str) -> Vec<&str>> {
    updates.lines().map(|l| l.split(',').collect())
}

fn in_order(rules: &HashMap<&str, HashSet<&str>>, update: &[&str]) -> bool {
    for i in 1..update.len() {
        for j in 0..i {
            if let Some(matched_rules) = rules.get(&update[i]) {
                if matched_rules.contains(&update[j]) {
                    return false;
                }
            }
        }
    }
    true
}

fn part2<'a>(updates: &'a str, parsed_rules: &HashMap<&str, HashSet<&str>>) -> Vec<Vec<&'a str>> {
    parse_updates(updates)
        .filter(|u| !in_order(parsed_rules, u))
        .map(|u| put_in_order(parsed_rules, &u))
        .collect()
}

fn put_in_order<'a>(rules: &HashMap<&str, HashSet<&str>>, update: &Vec<&'a str>) -> Vec<&'a str> {
    (0..update.len()).fold(update.clone(), |mut acc, i| {
        let earlier = target_position(rules, update, &acc, i);

        if let Some(e) = earlier {
            acc.remove(i);
            acc.insert(e, update[i]);
        }

        acc
    })
}

fn target_position(
    rules: &HashMap<&str, HashSet<&str>>,
    update: &[&str],
    acc: &[&str],
    i: usize,
) -> Option<usize> {
    rules
        .get(update[i])
        .iter()
        .flat_map(|h| h.iter())
        .filter_map(|r| acc.iter().position(|u| u == r))
        .min()
        .filter(|e| *e < i)
}
