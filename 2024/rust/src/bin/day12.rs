use aoc24::read_day_input;
use std::collections::HashSet;
use std::convert::Infallible;
use std::str::FromStr;

fn main() {
    let sw_parsing = std::time::Instant::now();
    let grid: Grid = read_day_input(12).parse().expect("Failed to parse grid");
    println!("Parsing took: {}µs", sw_parsing.elapsed().as_micros());

    let sw_part1 = std::time::Instant::now();
    let price = part1(&grid);
    println!(
        "Part 1: '{price}' took {}ms",
        sw_part1.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> usize {
    let regions = find_regions(grid);
    regions
        .iter()
        .map(|r| r.price(&grid))
        // .inspect(|p| println!("{p}"))
        .sum()
}

fn find_regions(grid: &Grid) -> Vec<Region> {
    let mut result = Vec::new();
    let mut seen: HashSet<Point> = HashSet::new();
    for plant in &grid.plants {
        if !seen.contains(&plant.loc) {
            let mut neighbours: Vec<Plant> = vec![plant.clone()];
            find_neighbours(grid, plant.clone(), &mut neighbours, &mut seen);
            result.push(Region {
                name: neighbours[0].name,
                plants: neighbours,
            });
        }
    }
    result
}

fn find_neighbours(
    grid: &Grid,
    plant: Plant,
    neighbours: &mut Vec<Plant>,
    seen: &mut HashSet<Point>,
) {
    if seen.contains(&plant.loc) {
        return;
    }
    seen.insert(plant.loc);

    for neighbour in plant.loc.neighbours(&grid) {
        if seen.contains(&neighbour) {
            continue;
        }

        let other_plant = &grid.plants[grid.to_index(&neighbour)];
        if plant.is_adjacent_to(other_plant, &grid) && plant.is_same_species(other_plant) {
            neighbours.push(other_plant.clone());
            find_neighbours(&grid, other_plant.clone(), neighbours, seen);
        }
    }
}

// -------------------- TYPES: GRID --------------------

struct Grid {
    plants: Vec<Plant>,
    width: isize,
    height: isize,
}

impl Grid {
    fn is_out_of_bounds(&self, p: &Point) -> bool {
        p.x < 0 || p.x >= self.width || p.y < 0 || p.y >= self.height
    }

    fn to_index(&self, point: &Point) -> usize {
        usize::try_from(point.y * self.width + point.x).unwrap()
    }
}

impl FromStr for Grid {
    type Err = Infallible; // Cell parsing will just panic

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plants = Vec::new();
        let mut width: isize = 0;
        let mut height: isize = 0;

        for (y, line) in s.lines().enumerate() {
            if width == 0 {
                width = isize::try_from(line.len()).unwrap();
            }

            for (x, c) in line.chars().enumerate() {
                let p = Plant {
                    name: c,
                    loc: Point::new(x, y),
                };
                plants.push(p);
            }

            height += 1;
        }

        Ok(Self {
            plants,
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
        let x = isize::try_from(x).unwrap_or_else(|_| panic!("Failed to parse x"));
        let y = isize::try_from(y).unwrap_or_else(|_| panic!("Failed to parse y"));
        Self { x, y }
    }

    fn neighbours(&self, grid: &Grid) -> Vec<Self> {
        let mut neighbours = Vec::new();
        for dx_dy in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let n = Point {
                x: self.x + dx_dy.0,
                y: self.y + dx_dy.1,
            };
            if !grid.is_out_of_bounds(&n) {
                neighbours.push(n);
            }
        }
        neighbours
    }
}

// -------------------- TYPES: PLANT --------------------

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Plant {
    name: char,
    loc: Point,
}

impl Plant {
    fn is_adjacent_to(&self, other: &Self, grid: &Grid) -> bool {
        self.loc.neighbours(&grid).iter().any(|n| n == &other.loc)
    }

    fn is_same_species(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn boundary_edges(&self, grid: &Grid) -> usize {
        let mut boundaries = 0;
        let mut neighbours = Vec::new();
        for dx_dy in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let n = Point {
                x: self.loc.x + dx_dy.0,
                y: self.loc.y + dx_dy.1,
            };
            neighbours.push(n);
        }
        for n in neighbours {
            if grid.is_out_of_bounds(&n) || grid.plants[grid.to_index(&n)].name != self.name {
                boundaries += 1;
            }
        }
        boundaries
    }
}

// -------------------- TYPES: REGION --------------------

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Region {
    name: char,
    plants: Vec<Plant>,
}

impl Region {
    fn area(&self) -> usize {
        self.plants.len()
    }

    fn perimeter(&self, grid: &Grid) -> usize {
        self.plants.iter().map(|p| p.boundary_edges(&grid)).sum()
    }

    fn price(&self, grid: &Grid) -> usize {
        self.area() * self.perimeter(&grid)
    }
}
