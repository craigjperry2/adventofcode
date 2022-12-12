use ferris_says::say;
use std::{io::{stdout, BufWriter, Error, Write}, collections::{HashMap, HashSet}, str::FromStr, ops::{Sub, Add, Mul}};
use itertools::{Itertools};
use regex::Regex;

fn main() -> Result<(), Error> {
    let stdout = stdout();
    let message = String::from("Craig's Advent of Code Runner 2022");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer)?;
    writer.flush()?;

    println!("Day 1 Part 1, max calories: {}", day1p1(load_input(1)));
    println!( "Day 1 Part 2, total calories from top 3 elves: {}", day1p2(load_input(1)));

    println!("Day 2 Part 1, total score: {}", day2p1(load_input(2)));
    println!("Day 2 Part 2, total score: {}", day2p2(load_input(2)));

    println!("Day 3 Part 1, total score: {}", day3p1(load_input(3)));
    println!("Day 3 Part 2, total score: {}", day3p2(load_input(3)));

    println!("Day 4 Part 1, total pairs: {}", day4p1(load_input(4)));
    println!("Day 4 Part 2, total pairs: {}", day4p2(load_input(4)));

    println!("Day 5 Part 1, top crates: {}", day5p1(load_input(5)));
    println!("Day 5 Part 2, top crates: {}", day5p2(load_input(5)));

    println!("Day 6 Part 1, Start marker at: {}", day6p1(load_input(6)));
    println!("Day 6 Part 2, Start marker at: {}", day6p2(load_input(6)));
    
    println!("Day 7 Part 1, total size: {}", day7p1(load_input(7)));
    println!("Day 7 Part 2, total size: {}", day7p2(load_input(7)));

    println!("Day 8 Part 1, visible trees: {}", day8p1(load_input(8)));
    println!("Day 8 Part 2, highest score: {}", day8p2(load_input(8)));

    println!("Day 9 Part 1, positions visited: {}", day9p1(load_input(9)));
    println!("Day 9 Part 2, positions visited: {}", day9p2(load_input(9)));

    println!("Day 10 Part 1, sum of signals: {}", day10p1(load_input(10)));
    println!("Day 10 Part 2, letters: ");
    day10p2(load_input(10));

    println!("Day 11 Part 1, level of monkey business: {}", day11p1(load_input(11)));
    println!("Day 11 Part 2, level of monkey business: {}", day11p2(load_input(11)));

    println!("Day 12 Part 1, fewest steps: {}", day12p1(load_input(12)));
    println!("Day 12 Part 2, fewest steps: {}", day12p2(load_input(12)));

    Ok(())
}

fn load_input(day: u8) -> String {
    let path = format!("resources/day{}.txt", day);
    std::fs::read_to_string(path).expect("Unable to read file")
}

fn day1p1(input: String) -> i32 {
    input
        .split("\n\n")
        .map(|batch|
            batch
                .lines()
                .map(|num| num.parse::<i32>().unwrap())
                .sum()
        )
        .max()
        .expect("No batches parsed")
}

fn day1p2(input: String) -> i32 {
    input
        .split("\n\n")
        .map(|batch|
            batch
                .lines()
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
        )
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum()
}

fn day2p1(input: String) -> i32 {
    let mut lookup = HashMap::new();
    lookup.insert("A X", 4);
    lookup.insert("A Y", 8);
    lookup.insert("A Z", 3);
    lookup.insert("B X", 1);
    lookup.insert("B Y", 5);
    lookup.insert("B Z", 9);
    lookup.insert("C X", 7);
    lookup.insert("C Y", 2);
    lookup.insert("C Z", 6);

    score(&input, &lookup)
}

fn day2p2(input: String) -> i32 {
    let mut lookup = HashMap::new();
    lookup.insert("A X", 3);
    lookup.insert("A Y", 4);
    lookup.insert("A Z", 8);
    lookup.insert("B X", 1);
    lookup.insert("B Y", 5);
    lookup.insert("B Z", 9);
    lookup.insert("C X", 2);
    lookup.insert("C Y", 6);
    lookup.insert("C Z", 7);

    score(&input, &lookup)
}

fn score(input: &String, lookup: &HashMap<&str, i32>) -> i32 {
    input
        .split_terminator("\n")
        .map(|game| {
            lookup.get(&game).expect(format!("No score for {}", game).as_str())
        })
        .sum()             
}

fn priority(item: char) -> i32 {
    (" abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(item))
        .expect("unexpected priority")
        .try_into()
        .expect("priority overflow")
}

fn day3p1(input: String) -> i32 {
    input
        .split_terminator("\n")
        .map(|rucksack| {
            let (left, right) = rucksack.split_at(rucksack.len() / 2);
            let left_set: HashSet<char> = left.chars().collect();
            right
                .chars()
                .unique()
                .filter(|c| left_set.contains(c))
                .map(priority)
                .exactly_one()
                .unwrap()
        })
        .sum()
}

fn day3p2(input: String) -> i32 {
    input
        .split_terminator("\n")
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|group|
            group
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .expect("No common item in group")
                .into_iter()
                .exactly_one()
                .expect("Multiple common items in group")
        )
        .map(priority)
        .sum()
}

fn split_and_parse<T>(input: &str, separator: &str) -> (T, T)
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug
{
    let comma = input.find(separator).expect(format!("No {} in input", separator).as_str());
    let left = &input[..comma];
    let right = &input[comma+1..];

    let left = left.parse::<T>().expect(format!("Left is not a {}", std::any::type_name::<T>()).as_str());
    let right = right.parse::<T>().expect(format!("Right is not a {}", std::any::type_name::<T>()).as_str());

    (left, right)
}

fn day4(input: String) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    input
        .split_terminator("\n")
        .map(|assignments| {
            let (left, right) = split_and_parse::<String>(&assignments, ",");

            let (left_from, left_to) = split_and_parse::<i32>(&left, "-");
            let left_set: HashSet<i32> = (left_from..=left_to).collect();

            let (right_from, right_to) = split_and_parse::<i32>(&right, "-");
            let right_set: HashSet<i32> = (right_from..=right_to).collect();
            (left_set, right_set)
        })
        .collect::<Vec<(HashSet<i32>, HashSet<i32>)>>()
}

fn day4p1(input: String) -> i32 {
    day4(input)
        .iter()
        .map(|(l, r)| (l.is_superset(&r) || r.is_superset(&l)) as i32) 
        .sum()
}

fn day4p2(input: String) -> i32 {
    day4(input)
        .iter()
        .map(|(l, r)| l.intersection(&r).count() + r.intersection(&l).count() > 0)
        .map(|x| x as i32)
        .sum()
}


fn day5(input: String, part1: bool) -> String {
    let (starting_state, instructions) = input.split_terminator("\n\n").collect_tuple().expect("Failed to parse state and instructions");

    let mut state = starting_state
        .lines()
        // discard the last row which contains the column numbers
        .take(8)
        // normalise column spacing to allow striding over every 4th char starting from the 0th position
        .map(|row|
            row.chars()
                .skip(1)
                .collect::<String>()
        )
        // stride over every 4th char to parse the given state into a 2d array
        .map(|row|
            row.chars()
                .enumerate()
                .filter(|(i, _)| i % 4 == 0)
                .map(|(_, c)| c)
                .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // transpose the state matrix
    state = (0..state[0].len())
        .map(|i|
            state.iter()
                .map(|row| row[i])
                .filter(|c| *c != ' ')
                .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // parse and execute each instruction
    instructions
        .lines()
        .map(|instruction| {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("Failed to compile regex");
            let captures = re.captures(instruction).expect("Failed to parse instruction");
            // parse into a 3-tuple of (count, from, to)
            return (captures[1].parse::<i32>().expect("Failed to parse count"), captures[2].parse::<i32>().expect("Failed to parse from"), captures[3].parse::<i32>().expect("Failed to parse to"));
        })
        .for_each(|(count, from, to)| {
            // NB: 0 indexed vector but 1 indexed instructions, hence the -1
            let mut x = state[(from - 1) as usize].drain(..count as usize).collect::<Vec<char>>();
            if part1 {
                // part 1 requires us to place the items in reverse order just as if we moved 1 box at a time
                x.reverse();
            }
            state[(to - 1) as usize].splice(0..0, x);
        });

    // The top crate of each stack is the final answer
    state
        .iter()
        .map(|row| row[0])
        .collect::<String>()

}

fn day5p1(input: String) -> String {
    day5(input, true)
}

fn day5p2(input: String) -> String {
    day5(input, false)
}

fn day6p1(input: String) -> i32 {
    // fold over the input keeping track of the last 4 seen chars and the index
    input
        .chars()
        .enumerate()
        .map(|(j, x)| (j+1, x)) // is there a better way to enumerate starting at 1 instead of 0 in rust?
        .fold((false, '\0', '\0', '\0', '\0', 0_usize), |acc, (j, x)| { // use the \0 null char as a null placeholder
            if acc.0 {
                // haven't found a neat way to short circuit the fold in rust when i found the answer, hence the flag field and ignoring the remaining input
                // did mess around with try_fold and ControlFlow but that needed a nightly rust compiler rather than stable to opt into a beta feature
                return acc
            }
            let s = HashSet::from([acc.2, acc.3, acc.4, x]);
            if j > 3 && s.len() == 4 { // a false positive occurs when the first 3 chars consumed are unique thanks to the init value of \0 being unique
                return (true, acc.2, acc.3, acc.4, x, j)
            }
            (false, acc.2, acc.3, acc.4, x, j)
        }).5 as i32

        // having done this, i'm not sure reducing over the input was the best approach, i suspect a for loop with a fixed size ring buffer would have been better but would have entailed spending on another dependency crate rather than stdlib (or book-keeping my use of a vecdeque from stdlib)
}

fn day6p2(input: String) -> i32 {
    // iterate the input looking for the first 14 contiguous chars that are unique and return the index of the 14th char
    input
        .chars()
        .enumerate()
        .skip(13)
        .filter(|(j, x)| {
            // this is less code but way less efficient than my ring buffer idea at the end of part 1
            let mut s : HashSet<char> = HashSet::from_iter(input.chars().skip(j-13).take(13));
            s.insert(*x);
            s.len() == 14
        })
        .map(|(j, _)| j+1) // convert to 1-indexed
        .next()
        .expect("Failed to find a match")
        as i32
}

fn day7(input: String) -> HashMap<String, i64> {
    let mut dirpath_size: HashMap<String, i64> = HashMap::new();
    let mut dirpath: Vec<String> = Vec::new();

    input
        .lines()
        .for_each(|line| {
            match line {
                line if line.starts_with("dir") || line.starts_with("$ ls") => {
                    // discard the dir and $ ls lines
                    return;
                }
                line if line.starts_with("$ cd") => {
                    // keep track of the current dirpath
                    let dir = line
                        .split_whitespace()
                        .nth(2).expect("Invalid cd command format");
                    if dir == ".." {
                        dirpath.pop();
                    } else {
                        dirpath.push(dir.to_string());
                    }
                }
                line if line.starts_with(char::is_numeric) => {
                    let size = line
                        .split_whitespace()
                        .next().expect("Invalid line format")
                        .parse::<i64>().expect("Invalid number format");
                    
                    // increment the running total for this dir and each parent dir in the path
                    let mut dirpath_local = dirpath.clone();
                    while dirpath_local.len() > 0 {
                        dirpath_size
                            .entry(dirpath_local.join("/"))
                            .and_modify(|count| *count += size)
                            .or_insert(size);
                        dirpath_local.pop();
                    }
                }
                line => {
                    panic!("Invalid line format: {}", line);
                }
            }
        });

        dirpath_size
}

fn day7p1(input: String) -> i64 {
    // iterate all the values of dirpath_size, filter out values > 100000, sum the remaining values
    day7(input)
            .iter()
            .filter(|(_, size)| **size <= 100000)
            .map(|(_, size)| size)
            .sum::<i64>()
}

fn day7p2(input: String) -> i64 {
    let dirpath_size = day7(input);
    let total_space = 70000000;
    let current_used_space = dirpath_size.get("/").expect("Failed to find root dir");
    let min_free_space = 30000000;
    let needed_space = min_free_space - (total_space - current_used_space);

    *dirpath_size
            .iter()
            .filter(|(_, size)| **size >= needed_space)
            .map(|(_, size)| size)
            .min().expect("No dirs large enough were identified")
}

fn day8p1(input: String) -> i32 {
    // split input into lines, collecting in a vector of vectors of i32
    let tree_grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).expect("Invalid input") as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut visible: HashSet<(i32, i32)> = HashSet::new();
    let ylen = tree_grid.len();
    let xlen = tree_grid[0].len();

    // assess each row from west to east, keeping track of the visible trees
    for y in 0..ylen {
        let mut tallest_tree_so_far = 0;

        for x in 0..xlen {
            let t = tree_grid[y][x];
            if x == 0 || t > tallest_tree_so_far {
                visible.insert((x as i32, y as i32));
                tallest_tree_so_far = t;
            }
        }
    }

    // assess each column from north to south, keeping track of the visible trees
    for x in 0..xlen {
        let mut tallest_tree_so_far = 0;

        for y in 0..ylen {
            let t = tree_grid[y][x];
            if y == 0 || t > tallest_tree_so_far {
                visible.insert((x as i32, y as i32));
                tallest_tree_so_far = t;
            }
        }
    }

    // assess each row from east to west, keeping track of the visible trees
    for y in 0..ylen {
        let mut tallest_tree_so_far = 0;

        for x in (0..xlen).rev() {
            let t = tree_grid[y][x];
            if x == xlen-1 || t > tallest_tree_so_far {
                visible.insert((x as i32, y as i32));
                tallest_tree_so_far = t;
            }
        }
    }

    // assess each column from south to north, keeping track of the visible trees
    for x in 0..xlen {
        let mut tallest_tree_so_far = 0;

        for y in (0..ylen).rev() {
            let t = tree_grid[y][x];
            if y == ylen-1 || t > tallest_tree_so_far {
                visible.insert((x as i32, y as i32));
                tallest_tree_so_far = t;
            }
        }
    }

    visible.len() as i32
}

// vec of all visible trees north of this point
fn look_north(grid: &Vec<Vec<i32>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let mut visible: Vec<(usize, usize)> = Vec::new();
    let max_height = grid[coords.1][coords.0];

    for y in (0..coords.1).rev() {
        let t = grid[y][coords.0];
        visible.push((coords.0, y));
        if t >= max_height {
            break;
        }
    }

    visible
}

// vec of all visible trees south of this point
fn look_south(grid: &Vec<Vec<i32>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let ylen = grid.len();
    let mut visible: Vec<(usize, usize)> = Vec::new();
    let max_height = grid[coords.1][coords.0];

    for y in coords.1+1..ylen {
        let t = grid[y][coords.0];
        visible.push((coords.0, y));
        if t >= max_height {
            break;
        }
    }

    visible
}

// vec of all visible trees east of this point
fn look_east(grid: &Vec<Vec<i32>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let xlen = grid[0].len();
    let mut visible: Vec<(usize, usize)> = Vec::new();
    let max_height = grid[coords.1][coords.0];

    for x in coords.0+1..xlen {
        let t = grid[coords.1][x];
        visible.push((x, coords.1));
        if t >= max_height {
            break;
        }
    }

    visible
}

// vec of all visible trees west of this point
fn look_west(grid: &Vec<Vec<i32>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    let mut visible: Vec<(usize, usize)> = Vec::new();
    let max_height = grid[coords.1][coords.0];

    for x in (0..coords.0).rev() {
        let t = grid[coords.1][x];
        visible.push((x, coords.1));
        if t >= max_height {
            break;
        }
    }

    visible
}

fn day8p2(input: String) -> i32 {
    let tree_grid = input
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).expect("Invalid input") as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    
    tree_grid
        .iter()
        .enumerate()
        .map(|(y, row)|
                row.iter()
                .enumerate()
                .map(|(x, _)|
                    look_north(&tree_grid, (x, y)).len() *
                    look_west(&tree_grid, (x, y)).len() *
                    look_south(&tree_grid, (x, y)).len() *
                    look_east(&tree_grid, (x, y)).len()
                )
                .max().unwrap() as i32
        )
        .max().unwrap()
}

#[derive(Debug, PartialEq)]
enum DirectionDistanceInstruction {
    U(i32),
    D(i32),
    L(i32),
    R(i32),
}

impl FromStr for DirectionDistanceInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().nth(0).expect("Invalid direction");
        let dist = s[2..].parse::<i32>().expect("Invalid distance");
        match dir {
            'U' => Ok(DirectionDistanceInstruction::U(dist)),
            'D' => Ok(DirectionDistanceInstruction::D(dist)),
            'L' => Ok(DirectionDistanceInstruction::L(dist)),
            'R' => Ok(DirectionDistanceInstruction::R(dist)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point(i32, i32);

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point(self.0 - other.0, self.1 - other.1)
    }
}

fn move_tail(head: Point, tail: Point) -> Point {
    match head - tail {
        // Move diagonally up+left
        Point(-2, 1) | Point(-2, 2) | Point(-1, 2) => Point(tail.0 - 1, tail.1 + 1),
        // Move diagonally up+right
        Point(1, 2) | Point(2, 2) | Point(2, 1) => Point(tail.0 + 1, tail.1 + 1),
        // Move diagonally down+right
        Point(2, -1) | Point(2, -2) | Point(1, -2) => Point(tail.0 + 1, tail.1 - 1),
        // Move diagonally down+left
        Point(-1, -2) | Point(-2, -2) | Point(-2, -1) => Point(tail.0 - 1, tail.1 - 1),
        // Move up
        Point(0, 2) => Point(tail.0, tail.1 + 1),
        // Move right
        Point(2, 0) => Point(tail.0 + 1, tail.1),
        // Move down
        Point(0, -2) => Point(tail.0, tail.1 - 1),
        // Move left
        Point(-2, 0) => Point(tail.0 - 1, tail.1),
        _ => tail
    }
}

fn day9p1(input: String) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);
    visited.insert(tail);  // the 's' position

    input
        .lines()
        .for_each(|line| {
            line
                .split(",")
                .map(|s| {
                    s.parse().expect("Invalid instruction")
                })
                .for_each(|instruction| {
                    match instruction {
                        DirectionDistanceInstruction::U(dist) => {
                            for _ in 0..dist {
                                head.1 += 1;
                                tail = move_tail(head, tail);
                                visited.insert(tail);
                            }
                        }
                        DirectionDistanceInstruction::D(dist) => {
                            for _ in 0..dist {
                                head.1 -= 1;
                                tail = move_tail(head, tail);
                                visited.insert(tail);
                            }
                        }
                        DirectionDistanceInstruction::L(dist) => {
                            for _ in 0..dist {
                                head.0 -= 1;
                                tail = move_tail(head, tail);
                                visited.insert(tail);
                            }
                        }
                        DirectionDistanceInstruction::R(dist) => {
                            for _ in 0..dist {
                                head.0 += 1;
                                tail = move_tail(head, tail);
                                visited.insert(tail);
                            }
                        }
                    }
                });
        });
    visited.len() as i32
}

fn update_knots(knots: &mut Vec<Point>, dist: i32, visited: &mut HashSet<Point>, f: impl Fn(&mut Point)) {
    for _ in 0..dist {
        f(&mut knots[0]);
        (1..10).for_each(|i| {
            knots[i] = move_tail(knots[i-1], knots[i]);
        });
        visited.insert(knots[9]);
    }
}

fn day9p2(input: String) -> i32 {
    let mut knots = vec![
        Point(0, 0), // 0 = head
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0),
        Point(0, 0), // 9 = tail
    ];
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(knots[9]);  // the 's' position

    input
        .lines()
        .for_each(|line| {
            line
                .split(",")
                .map(|s| {
                    s.parse().expect("Invalid instruction")
                })
                .for_each(|instruction| {
                    match instruction {
                        DirectionDistanceInstruction::U(dist) => {
                            update_knots(&mut knots, dist, &mut visited, |p| p.1 += 1);
                        }
                        DirectionDistanceInstruction::D(dist) => {
                            update_knots(&mut knots, dist, &mut visited, |p| p.1 -= 1);
                        }
                        DirectionDistanceInstruction::L(dist) => {
                            update_knots(&mut knots, dist, &mut visited, |p| p.0 -= 1);
                        }
                        DirectionDistanceInstruction::R(dist) => {
                            update_knots(&mut knots, dist, &mut visited, |p| p.0 += 1);
                        }
                    }
                });
        });
    visited.len() as i32
}

fn day10p1(input: String) -> i32 {
    let trace: HashMap<i32, i32> = trace_program(input);
    (20..=220).step_by(40)
        .map(|i| {
            i * trace.get(&(i-1)).expect(format!("Invalid clock: {}", i).as_str())
        }).sum()
}

fn trace_program(input: String) -> HashMap<i32, i32>{
    let mut clock = 0;
    let mut last_value = 1;
    let mut trace: HashMap<i32, i32> = HashMap::new();
    trace.insert(clock, last_value);

    input
        .lines()
        .for_each(|line| {
            let instruction = &line[0..4];
            if instruction == "noop" {
                clock += 1;
                trace.insert(clock, last_value);
            } else {
                clock += 1;
                trace.insert(clock, last_value);
                clock += 1;
                last_value += line[5..].parse::<i32>().expect("Invalid value");
                trace.insert(clock, last_value);
            };
        });
    trace
}

fn day10p2(input: String) {
    let trace: HashMap<i32, i32> = trace_program(input);

    (0..=239)
        .map(|pos| {
            let v = *trace.get(&pos).expect(format!("Failed to unwrap trace value for clock {}", pos).as_str());
            if v-1 <= (pos%40) && v+1 >= (pos%40) { "#" } else { "." }
        })
        .chunks(40)
        .into_iter()
        .map(|c| c.collect::<String>())
        .for_each(|l| println!("{}", l));
}

type ItemWorryLevel = i64;
type MonkeyId = usize;

#[derive(Debug)]
struct Monkey {
    pub item_levels: Vec<ItemWorryLevel>,
    pub operand: ItemWorryLevel,
    pub operation: fn(ItemWorryLevel, ItemWorryLevel) -> ItemWorryLevel,
    pub test: ItemWorryLevel,
    pub next_true: MonkeyId,
    pub next_false: MonkeyId,
    pub inspected: i32,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let m = Monkey {
            item_levels: lines[1].split_once(": ").expect("Failed to parse starting items").1
                    .split_terminator(", ").map(|i| i.parse().expect("Failed to parse monkey item"))
                    .collect(),
            operand: lines[2].split_once(&['+', '*']).expect("Failed to parse operation").1[1..].parse::<i64>().unwrap_or(0),
            operation: if lines[2].contains(" + ") { ItemWorryLevel::add } else { ItemWorryLevel::mul },
            test: lines[3].split_once("by ").expect("Failed to parse test").1.parse().expect("Failed to parse test operand"),
            next_true: lines[4].split_once("monkey ").expect("Failed to parse true action").1.parse().expect("Failed to parse true action operand"),
            next_false: lines[5].split_once("monkey ").expect("Failed to parse false action").1.parse().expect("Failed to parse false action operand"),
            inspected: 0,
        };
        Ok(m)
    }
}

type Monkeys = Vec<Monkey>;

fn parse_monkeys(input: String) -> Monkeys {
    input
        .split_terminator("\n\n")
        .map(|m| m.parse().expect("Failed to parse monkey"))
        .collect()
}

fn report_monkeys(monkeys: &Monkeys) -> i64 {
    monkeys.iter()
        .sorted_by(|a, b| b.inspected.cmp(&a.inspected))
        .take(2)
        .map(|m| m.inspected as i64)
        .product()
}
    
fn day11p1(input: String) -> i64 {
    let mut monkeys = parse_monkeys(input);
    
    for _ in 1..=20 {
        for j in 0..monkeys.len() {
            for _ in 0..monkeys[j].item_levels.len() {
                let mut item = monkeys[j].item_levels.remove(0);
                monkeys[j].inspected += 1;
                item = (monkeys[j].operation)(item,
                    if monkeys[j].operand == 0 {item} else {monkeys[j].operand}
                );
                item /= 3;
                let next;
                if item % monkeys[j].test == 0 {
                    next = monkeys[j].next_true;
                } else {
                    next = monkeys[j].next_false;
                }
                monkeys[next].item_levels.push(item);
            }
        }
    }

    report_monkeys(&monkeys)
}

fn day11p2(input: String) -> i64 {
    let mut monkeys = parse_monkeys(input);

    let factor = monkeys.iter().map(|m| m.test).product::<i64>();
    
    // I spent a lot of time fighting rust on this problem, this is the first problem where ownership got in my way. My initial idea
    // was to .iter() and accumulate, the same pattern i've used in pretty much all the other problems but there were two borrows of
    // monkeys vector and my rust-fu ain't strong enough to do it. I still think this general idea is likely expressable as an iter()
    // with more rust experience
    for _ in 1..=10000 {
        for j in 0..monkeys.len() {
            for _ in 0..monkeys[j].item_levels.len() {
                let mut item = monkeys[j].item_levels.remove(0);
                monkeys[j].inspected += 1;
                item %= factor;  // I needed a chat for this, i didn't figure this out myself :-(
                item = (monkeys[j].operation)(item,
                    if monkeys[j].operand == 0 {item} else {monkeys[j].operand}
                );
                let next: MonkeyId;
                if item % monkeys[j].test == 0 {
                    next = monkeys[j].next_true;
                } else {
                    next = monkeys[j].next_false;
                }
                monkeys[next].item_levels.push(item);
            }
        }
    }

    report_monkeys(&monkeys)
}

type HeightMap = Vec<Vec<i32>>;
type Position = (i32, i32);

fn day12p1(input: String) -> i32 {
    let mut start: Position = (0, 0);
    let mut end: Position = (0, 0);
    let height_map = input
        .lines()
        .enumerate()
        .map(|(y, l)| l.chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'S' {
                    start = (x as i32, y as i32);
                    0 // a
                } else if c == 'E' {
                    end = (x as i32, y as i32);
                    25 // z
                } else {
                    c as i32 - 97
                }
            })
            .collect::<Vec<i32>>())
        .collect::<HeightMap>();
    
    *bfs(start, end, &height_map)
        .iter()
        .sorted()
        .next()
        .unwrap()
}

fn bfs(start: Position, end: Position, height_map: &HeightMap) -> Vec<i32> {
    let mut paths: Vec<i32> = vec![];
    let mut queue: Vec<(Position, i32)> = vec![(start, 0)];
    let mut visited: HashSet<Position> = HashSet::new();
    while !queue.is_empty() {
        let (pos, length) = queue.pop().unwrap();
        if pos == end {
            paths.push(length);
        } else if !visited.contains(&pos) {
            visited.insert(pos);
            let (x, y) = pos;
            let height = height_map[y as usize][x as usize];
            let mut next = vec![];
            if y > 0 && height_map[y as usize - 1][x as usize] <= height + 1 {
                next.push(((x, y-1), length + 1));
            }
            if y < height_map.len() as i32 - 1 && height_map[y as usize + 1][x as usize] <= height + 1 {
                next.push(((x, y+1), length + 1));
            }
            if x > 0 && height_map[y as usize][x as usize - 1] <= height + 1 {
                next.push(((x-1, y), length + 1));
            }
            if x < height_map[y as usize].len() as i32 - 1 && height_map[y as usize][x as usize + 1] <= height + 1 {
                next.push(((x+1, y), length + 1));
            }
            queue.splice(0..0, next);
        } else {
            // already visited
            continue;
        }
    }
    paths
}

fn day12p2(input: String) -> i32 {
    let mut starting_points: Vec<Position> = vec![];
    let mut end: Position = (0, 0);
    let height_map = input
        .lines()
        .enumerate()
        .map(|(y, l)| l.chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'S' || c == 'a' {
                    starting_points.push((x as i32, y as i32));
                    0 // a
                } else if c == 'E' {
                    end = (x as i32, y as i32);
                    25 // z
                } else {
                    c as i32 - 97
                }
            })
            .collect::<Vec<i32>>())
        .collect::<HeightMap>();
    

    starting_points
        .iter()
        .map(|s| bfs(*s, end, &height_map))
        .flatten()
        .sorted()
        .min()
        .unwrap()
}
