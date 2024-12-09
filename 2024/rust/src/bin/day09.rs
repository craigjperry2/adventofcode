use aoc24::read_day_input;
use std::iter::repeat;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(9);
    let file_map: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let checksum = part1(&file_map);
    println!(
        "Part 1: '{checksum}' took {}ms",
        sw_part1.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(file_map: &Vec<usize>) -> usize {
    let mut expanded_file_map: Vec<Option<usize>> = file_map
        .iter()
        .enumerate()
        .flat_map(|(id, &size)| repeat(if id % 2 == 0 { Some(id / 2) } else { None }).take(size))
        .collect();
    // println!("{:?}", expanded_file_map);

    let mut block = expanded_file_map.len() - 1;
    let mut space = 0;
    while space < block {
        while space < block && expanded_file_map[space].is_some() {
            space += 1;
        }
        while block > space && expanded_file_map[block].is_none() {
            block -= 1;
        }
        expanded_file_map.swap(space, block);
        space += 1;
        block -= 1;
    }
    // println!("{:?}", expanded_file_map);

    expanded_file_map
        .iter()
        .enumerate()
        .map(|(pos, &id)| pos * id.unwrap_or(0))
        .sum()
}
