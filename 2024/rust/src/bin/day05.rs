use aoc24::read_day_input;
use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

fn main() {
    let input = read_day_input(5);
    let (rules, updates) = input.split_once("\n\n").expect("malformed input");

    let parsed_rules: HashMap<String, HashSet<String>> = rules
        .lines()
        .map(|l| {
            l.split_once('|')
                .unwrap_or_else(|| panic!("failed to parse rule {}", l))
        })
        .fold(HashMap::new(), |mut map, (b, a)| {
            map.entry(b.into()).or_default().insert(a.into());
            map
        });

    let result: i32 = updates
        .lines()
        .map(|l| l.split(',').map(str::to_string).collect())
        .filter(|u| in_order(&parsed_rules, u))
        .map(|u| {
            u[u.len().div(2)]
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("Failed to parse {} as i32", e))
        })
        .sum();

    println!("{:?}", result);
}

fn in_order(rules: &HashMap<String, HashSet<String>>, update: &Vec<String>) -> bool {
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
