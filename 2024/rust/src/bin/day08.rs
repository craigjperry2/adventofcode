use aoc24::read_day_input;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::ops::Sub;
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let grid: Grid = read_day_input(8).parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let unique_locations = part1(&grid);
    println!(
        "Part 1: '{unique_locations}' took {}µs",
        sw_part1.elapsed().as_micros()
    );

    let sw_part2 = std::time::Instant::now();
    let unique_locations = part2(&grid);
    println!(
        "Part 2: '{unique_locations}' took {}µs",
        sw_part2.elapsed().as_micros()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> usize {
    let antinodes: Vec<AntiNode> = grid
        .nodes
        .iter()
        .flat_map(|a| find_antinodes(&grid, a.clone()))
        .collect();

    let unique_locations: HashSet<Point> = antinodes.iter().map(|a| a.loc).unique().collect();

    unique_locations.len()
}

fn find_antinodes(grid: &Grid, node: Node) -> Vec<AntiNode> {
    grid.nodes
        .iter()
        .filter(|&n| n.freq == node.freq && n.loc != node.loc)
        .filter_map(|n| {
            let candidate = node.antinode(&n);
            if grid.is_out_of_bounds(&candidate.loc) {
                None
            } else {
                Some(candidate)
            }
        })
        .collect()
}

// -------------------- PART 2 --------------------

fn part2(grid: &Grid) -> usize {
    let antinodes: Vec<AntiNode> = grid
        .nodes
        .iter()
        .flat_map(|a| find_harmonic_antinodes(&grid, a.clone()))
        .collect();

    display(grid, &antinodes);

    // unique locations
    let unique_locations: HashSet<Point> = antinodes.iter().map(|a| a.loc).unique().collect();

    unique_locations.len()
}

fn find_harmonic_antinodes(grid: &Grid, node: Node) -> Vec<AntiNode> {
    grid.nodes
        .iter()
        .filter(|&n| n.freq == node.freq && n.loc != node.loc)
        .flat_map(|n| node.harmonic_antinodes(&n, &grid))
        .collect()
}

// -------------------- TYPES: GRID --------------------

struct Grid {
    nodes: Vec<Node>,
    width: isize,
    height: isize,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }
}

impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;

        for (y, line) in s.lines().enumerate() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_alphanumeric() {
                    let n = Node {
                        freq: c,
                        loc: Point::new(x, y),
                    };
                    nodes.push(n);
                }
            }

            height += 1;
        }

        Ok(Self {
            nodes,
            width,
            height,
        })
    }
}

// -------------------- TYPES: POINT --------------------

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        let x = isize::try_from(x).unwrap_or_else(|_| panic!("Failed to parse width"));
        let y = isize::try_from(y).unwrap_or_else(|_| panic!("Failed to parse height"));
        Self { x, y }
    }

    fn difference(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// -------------------- TYPES: NODE --------------------

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Node {
    freq: char,
    loc: Point,
}

impl Node {
    fn antinode(&self, other: &Node) -> AntiNode {
        let diff = self.loc.difference(&other.loc);
        AntiNode {
            node: self.clone(),
            loc: Point {
                x: self.loc.x + diff.x,
                y: self.loc.y + diff.y,
            },
        }
    }

    fn harmonic_antinodes(&self, other: &Node, grid: &Grid) -> Vec<AntiNode> {
        let diff = self.loc.difference(&other.loc);
        let mut antinodes = Vec::new();
        let mut candidate = other.loc;
        while !grid.is_out_of_bounds(&candidate) {
            antinodes.push(AntiNode {
                node: self.clone(),
                loc: candidate,
            });
            candidate = candidate - diff;
        }
        antinodes
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct AntiNode {
    node: Node,
    loc: Point,
}

// -------------------- UTILS --------------------

fn display(grid: &Grid, antinodes: &Vec<AntiNode>) {
    let xy_antinodes: HashMap<Point, &AntiNode> = antinodes.iter().map(|a| (a.loc, a)).collect();
    let xy_nodes: HashMap<Point, &Node> = grid.nodes.iter().map(|n| (n.loc, n)).collect();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point { x, y };
            let c = match xy_antinodes.get(&p) {
                Some(_) => '#',
                None => match xy_nodes.get(&p) {
                    Some(n) => n.freq,
                    None => '.',
                },
            };
            print!("{c}");
        }
        println!();
    }
}
