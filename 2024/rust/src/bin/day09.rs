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

    let sw_part2 = std::time::Instant::now();
    let checksum = part2(&file_map);
    println!(
        "Part 2: '{checksum}' took {}ms",
        sw_part2.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(file_map: &[usize]) -> usize {
    let mut expanded_file_map = expand(file_map);
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

fn expand(file_map: &[usize]) -> Vec<Option<usize>> {
    file_map
        .iter()
        .enumerate()
        .flat_map(|(id, &size)| repeat(if id % 2 == 0 { Some(id / 2) } else { None }).take(size))
        .collect()
}

// -------------------- PART 2 --------------------

fn part2(file_map: &[usize]) -> usize {
    let mut expanded_file_map: Vec<Option<usize>> = expand(file_map);
    // println!("{:?}", expanded_file_map);

    for (file_start, file_len) in find_files(&expanded_file_map) {
        let mut space_start = 0;
        while space_start < file_start {
            let next_space = find_next_space(&expanded_file_map, space_start, file_start);

            if next_space.is_some() {
                space_start = next_space.unwrap().0;
                let space_len = next_space.unwrap().1;
                if space_len >= file_len {
                    for i in 0..file_len {
                        expanded_file_map.swap(space_start + i, file_start + i);
                    }
                    break;
                }
            } else {
                break;
            }
            
            space_start += 1; // should really jump by space_len
        }
    }
    // println!("{:?}", expanded_file_map);

    expanded_file_map
        .iter()
        .enumerate()
        .map(|(pos, &id)| pos * id.unwrap_or(0))
        .sum()
}

fn find_next_space(
    file_map: &[Option<usize>],
    starting_from: usize,
    ending: usize,
) -> Option<(usize, usize)> {
    let mut start = starting_from;
    while file_map[start].is_some() {
        start += 1;
    }
    if start >= ending {
        return None;
    }
    let mut end = start + 1;
    while end < file_map.len() && file_map[end].is_none() {
        end += 1;
    }
    if end > ending+1 {
        return None;
    }
    Some((start, end - start))
}

type FileStart = usize;
type FileLen = usize;

fn find_files(file_map: &[Option<usize>]) -> Vec<(FileStart, FileLen)> {
    let mut files: Vec<(FileStart, FileLen)> = Vec::new();
    let mut end = file_map.len() - 1;

    loop {
        while end > 0 && file_map[end].is_none() {
            end -= 1;
        }
        if end == 0 {
            return files;
        }
        let id = file_map[end];
        let mut start: isize = isize::try_from(end).unwrap() - 1;
        while start >= 0 && file_map[usize::try_from(start).unwrap()] == id {
            start -= 1;
        }
        start += 1; // search overshoots by 1
        let ustart: usize = start.try_into().unwrap();
        files.push((ustart, end - ustart + 1)); // get size from 0-index
        end = ustart;

        if end == 0 {
            break;
        }
        end -= 1;
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Option<usize>> {
        vec![
            Some(0),
            Some(0),
            None,
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            Some(3),
            Some(3),
            None,
            Some(4),
            Some(4),
            None,
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(6),
            Some(6),
            Some(6),
            Some(6),
            None,
            Some(7),
            Some(7),
            Some(7),
            None,
            Some(8),
            Some(8),
            Some(8),
            Some(8),
            Some(9),
            Some(9),
        ]
    }

    #[test]
    fn find_next_space_test() {
        let result = find_next_space(&example(), 5, 10);
        assert_eq!(result, Some((8, 3)));
    }

    #[test]
    fn find_next_space_too_short() {
        let result = find_next_space(&example(), 5, 9);
        assert_eq!(result, None);
    }

    #[test]
    fn find_next_space_test_partial() {
        let result = find_next_space(&example(), 3, 10);
        assert_eq!(result, Some((3, 2)));
    }

    #[test]
    fn find_all_files() {
        let result = find_files(&example());
        assert_eq!(
            result,
            vec![
                (40, 2),
                (36, 4),
                (32, 3),
                (27, 4),
                (22, 4),
                (19, 2),
                (15, 3),
                (11, 1),
                (5, 3),
                (0, 2)
            ]
        );
    }
}
