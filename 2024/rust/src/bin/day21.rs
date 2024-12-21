use itertools::Itertools;
use std::collections::HashMap;
use std::iter::{once, repeat};

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = include_str!("../../../data/day21.txt");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let complexities = part1(input);
    println!(
        "Part 1: '{complexities}' took {}s",
        sw_part1.elapsed().as_secs()
    );
}

// -------------------- PART 1 --------------------

fn part1(codes: &str) -> usize {
    codes
        .lines()
        // .inspect(|a| println!("{a:?}"))
        .map(|line| {
            once('A') // Start at A
                .chain(line.chars())
                .tuple_windows()
                // .inspect(|a| println!("{a:?}"))
                .map(|(from, to)| xy_difference(from, to))
                // .inspect(|a| println!("{a:?}"))
                .map(|(x, y)| to_commands(x, y))
                // .inspect(|a| println!("{a:?}"))
                .collect::<String>()
        })
        // .inspect(|a: &String| println!("{a:?}"))
        .map(|buttons| {
            once('A')
                .chain(buttons.chars())
                .tuple_windows()
                .map(|(from, to)| xy_difference2(from, to))
                .map(|(x, y)| to_commands(x, y))
                .collect::<String>()
        })
        // .inspect(|a: &String| println!("{a:?}"))
        .map(|buttons| {
            once('A')
                .chain(buttons.chars())
                .tuple_windows()
                .map(|(from, to)| xy_difference2(from, to))
                .map(|(x, y)| to_commands(x, y))
                .collect::<String>()
        })
        .inspect(|a: &String| println!("{a:?}"))
        .zip(codes.lines().map(|l2| {
            l2.chars()
                .filter(|c2| c2.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        }))
        .inspect(|(buttons, code)| println!("b{} c{}", buttons.len(), code))
        .map(|(buttons, code)| buttons.len() * code)
        .sum()
}

// 165062 too high
// Problem - not choosing the shortest path on test case 5
// there's a sequence that should be <A,v<A but mine goes v<<A,>^A
// functionally identical but 2 keys longer

// <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// v<<A>>^A<A>AvA<^AA>A<vAAA>^A
// <A^A>^^AvvvA
// 029A

// 3, -1 -> >>>^
fn to_commands(x: isize, y: isize) -> String {
    let h = ["<", "", ">"][(x.signum() + 1) as usize].repeat(x.unsigned_abs());
    let v = ["^", "", "v"][(y.signum() + 1) as usize].repeat(y.unsigned_abs());
    // to avoid the gap, if x < 0, do it last otherwise do it first
    if x < 0 {
        format!("{v}{h}A")
    } else {
        format!("{h}{v}A")
    }
}

// 'A', '8' -> (-1, -3)
fn xy_difference(from: char, to: char) -> (isize, isize) {
    let lookup: HashMap<char, (isize, isize)> = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [' ', '0', 'A'],
    ]
    .iter()
    .enumerate()
    .flat_map(|(y, xs)| {
        xs.iter().enumerate().zip(repeat(y)).map(|((x, &key), y)| {
            (
                key,
                (isize::try_from(x).unwrap(), isize::try_from(y).unwrap()),
            )
        })
    })
    .collect();
    let (from_x, from_y) = lookup.get(&from).unwrap();
    let (to_x, to_y) = lookup.get(&to).unwrap();
    (to_x - from_x, to_y - from_y)
}

// 'A', '<' -> (-2, 1)
fn xy_difference2(from: char, to: char) -> (isize, isize) {
    let lookup: HashMap<char, (isize, isize)> = [
        [' ', '^', 'A'],
        ['<', 'v', '>']
    ].iter()
        .enumerate()
        .flat_map(|(y, xs)| {
            xs.iter().enumerate().zip(repeat(y)).map(|((x, &key), y)| {
                (
                    key,
                    (isize::try_from(x).unwrap(), isize::try_from(y).unwrap()),
                )
            })
        })
        .collect();
    let (from_x, from_y) = lookup.get(&from).unwrap();
    let (to_x, to_y) = lookup.get(&to).unwrap();
    (to_x - from_x, to_y - from_y)
}
