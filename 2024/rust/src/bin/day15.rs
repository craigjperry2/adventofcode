use aoc24::read_day_input;
use itertools::Itertools;
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let input = read_day_input(15);
    let (map, instructions) = input.split_once("\n\n").expect("Failed to parse input");
    let grid: Grid = map.parse().expect("Failed to parse grid");
    let instructions: Vec<Direction> = instructions
        .chars()
        .filter_map(|c| {
            if c == '\n' {
                None
            } else {
                Some(Direction::from(c))
            }
        })
        .collect();
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let gps_sum = part1(&grid, &instructions);
    println!(
        "Part 1: '{gps_sum}' took {}µs",
        sw_part1.elapsed().as_micros()
    );

    let sw_part2 = std::time::Instant::now();
    let gps_sum = part2(&grid, &instructions);
    println!(
        "Part 2: '{gps_sum}' took {}µs",
        sw_part2.elapsed().as_micros()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid, instructions: &[Direction]) -> isize {
    let mut grid = grid.clone();
    instructions.iter().for_each(|d| {
        let next = grid.loc.step(d);
        if grid.is_out_of_bounds(&next) || grid.is_obstacle(&next) {
            return;
        }

        // send out a probe looking for a space to push into
        let mut probe = next;
        let mut count = 1;
        let mut has_space = false;
        while !grid.is_out_of_bounds(&probe) && !grid.is_obstacle(&probe) {
            if grid.is_space(&probe) {
                has_space = true;
                break;
            }
            count += 1;
            probe = probe.step(d);
        }

        // if there's a space, offset back_probe from probe by 1 and walk them back to grid.loc, swapping each index in turn
        if has_space {
            let back_dir = d.opposite();
            let mut back_probe = probe.step(&back_dir);
            for _ in 0..count {
                grid.grid.swap(
                    probe.to_offset(grid.width),
                    back_probe.to_offset(grid.width),
                );
                back_probe = back_probe.step(&back_dir);
                probe = probe.step(&back_dir);
            }
            grid.loc = probe.step(&d); // rewind, overshot by 1 above
        }
    });

    // -------------------- FINAL STATE ACHIEVED --------------------

    grid.grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == Cell::Box {
                Some(Point::from_offset(i, grid.width as usize))
            } else {
                None
            }
        })
        .map(|p| p.y * 100 + p.x)
        .sum()
}

// -------------------- PART 2 --------------------

fn part2(grid: &Grid, instructions: &[Direction]) -> isize {
    let mut grid = grid.clone();

    // expand grid
    grid.width *= 2;
    grid.loc.x *= 2;
    grid.grid = grid
        .grid
        .iter()
        .flat_map(|c| {
            if *c == Cell::Box {
                vec![Cell::BoxLeft, Cell::BoxRight]
            } else if *c == Cell::Obstacle {
                vec![Cell::Obstacle, Cell::Obstacle]
            } else if *c == Cell::Location {
                vec![Cell::Location, Cell::Empty]
            } else {
                vec![Cell::Empty, Cell::Empty]
            }
        })
        .collect();

    instructions.iter().for_each(|d| {
        if d == &Direction::Left || d == &Direction::Right {
            let next = grid.loc.step(&d);
            if grid.is_out_of_bounds(&next) || grid.is_obstacle(&next) {
                return;
            }

            // send out a probe looking for a space to push into
            let mut probe = next;
            let mut count = 1;
            let mut has_space = false;

            // TODO: need to update for both left and right halfs, vertically
            while !grid.is_out_of_bounds(&probe) && !grid.is_obstacle(&probe) {
                if grid.is_space(&probe) {
                    has_space = true;
                    break;
                }
                count += 1;
                probe = probe.step(d);
            }

            // if there's a space, offset back_probe from probe by 1 and walk them back to grid.loc, swapping each index in turn
            if has_space {
                let back_dir = d.opposite();
                let mut back_probe = probe.step(&back_dir);
                for _ in 0..count {
                    grid.grid.swap(
                        probe.to_offset(grid.width),
                        back_probe.to_offset(grid.width),
                    );
                    back_probe = back_probe.step(&back_dir);
                    probe = probe.step(&back_dir);
                }
                grid.loc = probe.step(&d); // rewind, overshot by 1 above
            }
        } else {
            let next = grid.loc.step(&d);
            if grid.is_out_of_bounds(&next) || grid.is_obstacle(&next) {
                return;
            }

            // send out a probe looking for a space to push into
            let mut vprobe = vec![vec![next]];
            let mut has_space = vec![vec![false]];

            // breadth-first search for space while not hitting obstacles
            'outer: loop {
                // BUG: empty vprobe means nothing to back track swap
                // BUG: duplicates pushed onto next_probe for perfectly stacked boxes
                let probe = vprobe.last().unwrap();
                let mut next_probe: Vec<Point> = vec![];
                let mut next_space = vec![false; probe.len()];
                for i in 0..probe.len() {
                    if grid.is_out_of_bounds(&probe[i]) || grid.is_obstacle(&probe[i]) {
                        break 'outer;
                    }
                    if grid.is_space(&probe[i]) {
                        next_space[i] = true;
                        continue;
                    }
                    // BUG: has_space doesnt grow
                    if grid.is_box_left(&probe[i]) {
                        next_probe.push(probe[i].step(&d));
                        let adjacent = probe[i].step(&Direction::Right);
                        next_probe.push(adjacent.step(&d));
                    }
                    if grid.is_box_right(&probe[i]) {
                        next_probe.push(probe[i].step(&d));
                        let adjacent = probe[i].step(&Direction::Left);
                        next_probe.push(adjacent.step(&d));
                    }
                }

                has_space.push(next_space.clone());
                if next_space.iter().all(|b| *b) {
                    break;
                }

                if next_probe.len() > 0 {
                    vprobe.push(next_probe.into_iter().unique().collect());
                }
            }

            if has_space.iter().any(|b| b.iter().all(|b| *b)) {
                // BUG: what if there's an obstacle blocking one box but not adjacent?

                let back_dir = d.opposite();
                for ps in vprobe.iter().rev() {
                    for p in ps {
                        grid.grid.swap(
                            p.to_offset(grid.width),
                            p.step(&back_dir).to_offset(grid.width),
                        );
                    }
                }
                grid.loc = grid.loc.step(&d);
            }
        }
    });

    // -------------------- FINAL STATE ACHIEVED --------------------

    grid.grid
        .iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == Cell::BoxLeft {
                let p = Point::from_offset(i, grid.width as usize);
                Some(p.y * 100 + p.x)
            } else {
                None
            }
        })
        .sum()
}

// -------------------- TYPES: GRID --------------------

#[derive(Clone, Debug)]
struct Grid {
    grid: Vec<Cell>,
    width: isize,
    height: isize,
    loc: Point,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn is_obstacle(&self, p: &Point) -> bool {
        let c = &self.grid[p.to_offset(self.width)];
        *c == Cell::Obstacle
    }

    fn is_space(&self, p: &Point) -> bool {
        let c = &self.grid[p.to_offset(self.width)];
        *c == Cell::Empty
    }

    fn is_box_left(&self, p: &Point) -> bool {
        let c = &self.grid[p.to_offset(self.width)];
        *c == Cell::BoxLeft
    }

    fn is_box_right(&self, p: &Point) -> bool {
        let c = &self.grid[p.to_offset(self.width)];
        *c == Cell::BoxRight
    }
}

impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Cell> = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;
        let mut start: Point = Point::new(0, 0);

        for line in s.lines() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            for (i, c) in line.chars().enumerate() {
                let cell = Cell::from(c);
                if cell == Cell::Location {
                    start = Point::new(i, height as usize);
                }
                grid.push(cell);
            }

            height += 1;
        }

        Ok(Self {
            grid,
            width,
            height,
            loc: start,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x as usize, y as usize);
                let cell = &self.grid[p.to_offset(self.width)];
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// -------------------- TYPES: CELL --------------------

#[derive(Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Cell {
    Empty = b'.',
    Obstacle = b'#',
    Box = b'O',
    BoxLeft = b'[',
    BoxRight = b']',
    Location = b'@',
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Cell::*;

        match self {
            Empty => write!(f, "{}", Empty as u8 as char),
            Obstacle => write!(f, "{}", Obstacle as u8 as char),
            Box => write!(f, "{}", Box as u8 as char),
            BoxLeft => write!(f, "{}", BoxLeft as u8 as char),
            BoxRight => write!(f, "{}", BoxRight as u8 as char),
            Location => write!(f, "{}", Location as u8 as char),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        use Cell::*;

        const EMPTY: u8 = Empty as u8;
        const OBSTACLE: u8 = Obstacle as u8;
        const BOX: u8 = Box as u8;
        const BOX_LEFT: u8 = BoxLeft as u8;
        const BOX_RIGHT: u8 = BoxRight as u8;
        const LOCATION: u8 = Location as u8;

        match c as u8 {
            EMPTY => Empty,
            OBSTACLE => Obstacle,
            BOX => Box,
            BOX_LEFT => BoxLeft,
            BOX_RIGHT => BoxRight,
            LOCATION => Location,
            _ => panic!("Invalid cell: {c}"),
        }
    }
}

// -------------------- TYPES: DIRECTION --------------------

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => panic!("Failed to parse {c}"),
        }
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
        Self {
            x: isize::try_from(x).unwrap(),
            y: isize::try_from(y).unwrap(),
        }
    }

    fn from_offset(offset: usize, width: usize) -> Self {
        Self {
            x: isize::try_from(offset % width).unwrap(),
            y: isize::try_from(offset / width).unwrap(),
        }
    }

    fn to_offset(&self, width: isize) -> usize {
        (self.y * width + self.x) as usize
    }

    fn step(&self, d: &Direction) -> Self {
        match d {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}
