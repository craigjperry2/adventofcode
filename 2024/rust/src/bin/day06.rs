use aoc24::read_day_input;
use std::collections::HashSet;
use std::convert::Infallible;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

fn main() {
    let grid: Grid = read_day_input(6).parse().expect("Failed to parse grid");
    let distinct_positions = part1(&grid).len();
    println!("Part 1:\n{distinct_positions}");
}

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Obstacle,
    PositionFacingNorth,
    PositionFacingEast,
    PositionFacingSouth,
    PositionFacingWest,
}

// TODO: not really sure how to consolidate the ., #, ^, >, v, < literals into the enum definition.
//       you can assign an int to each variant so maybe store  ascii value? Yuk...
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use Cell::*;
        match self {
            Empty => write!(f, "."),
            Obstacle => write!(f, "#"),
            PositionFacingNorth => write!(f, "^"),
            PositionFacingEast => write!(f, ">"),
            PositionFacingSouth => write!(f, "v"),
            PositionFacingWest => write!(f, "<"),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        use Cell::*;
        match c {
            '.' => Empty,
            '#' => Obstacle,
            '^' => PositionFacingNorth,
            '>' => PositionFacingEast,
            'v' => PositionFacingSouth,
            '<' => PositionFacingWest,
            _ => panic!("Invalid cell: {c}"),
        }
    }
}

struct Grid {
    cells: Vec<Cell>,
    width: isize,
    height: isize,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn is_obstacle(&self, p: &Point) -> bool {
        self.cells[p.to_offset(self.width)] == Cell::Obstacle
    }
}

// FromStr because Grid has a text representation we want to parse()
impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;

        for line in s.lines() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            for c in line.chars() {
                cells.push(Cell::from(c));
            }

            height += 1;
        }

        Ok(Self {
            cells,
            width,
            height,
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[usize::try_from(y * self.width + x).unwrap()];
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn step(&self, d: &Direction) -> Self {
        use Direction::*;
        match d {
            North => Self {
                x: self.x,
                y: self.y - 1,
            },
            East => Self {
                x: self.x + 1,
                y: self.y,
            },
            South => Self {
                x: self.x,
                y: self.y + 1,
            },
            West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn from_offset(width: isize, offset: usize) -> Self {
        let ioffset = isize::try_from(offset).unwrap();
        Self {
            x: ioffset % width,
            y: ioffset / width,
        }
    }

    fn to_offset(&self, width: isize) -> usize {
        usize::try_from(self.y * width + self.x).unwrap()
    }
}

struct Guard {
    p: Point,
    d: Direction,
}

fn part1(grid: &Grid) -> HashSet<Point> {
    let mut positions = HashSet::new();
    let start = find_start(grid);
    recursive_step_through_grid(grid, start, &mut positions);
    positions
}

fn recursive_step_through_grid(grid: &Grid, loc: Guard, visited: &mut HashSet<Point>) {
    if grid.is_out_of_bounds(&loc.p) {
        return;
    }

    visited.insert(loc.p);

    let next_p = loc.p.step(&loc.d);
    let next_loc = if !grid.is_out_of_bounds(&next_p) && grid.is_obstacle(&next_p) {
        // Obstacle ahead, turn right 90 but don't move forward
        Guard {
            p: loc.p,
            d: loc.d.turn_right(),
        }
    } else {
        // Move one forward in same direction
        Guard {
            p: next_p,
            d: loc.d,
        }
    };

    recursive_step_through_grid(grid, next_loc, visited);
}

fn find_start(grid: &Grid) -> Guard {
    // TODO: there's a simpler way to capture North in the type system here, i'm sure...
    let offset: usize = grid
        .cells
        .iter()
        .position(|c| *c == Cell::PositionFacingNorth)
        .expect("Did not find a starting position");
    Guard {
        p: Point::from_offset(grid.width, offset),
        d: Direction::North,
    }
}
