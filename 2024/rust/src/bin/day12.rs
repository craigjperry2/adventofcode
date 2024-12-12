use aoc24::read_day_input;
use std::collections::{HashMap, HashSet};
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

    let sw_part2 = std::time::Instant::now();
    let price = part2(&grid);
    println!(
        "Part 2: '{price}' took {}ms",
        sw_part2.elapsed().as_millis()
    );
}

// -------------------- PART 1 --------------------

fn part1(grid: &Grid) -> usize {
    find_regions(grid)
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

// -------------------- PART 2 --------------------

fn part2(grid: &Grid) -> usize {
    let regions = find_regions(grid);

    let point_to_region: HashMap<Point, Region> = regions
        .iter()
        .flat_map(|r| r.plants.iter().map(|p| p.loc).map(|p| (p, r.clone())))
        .collect();

    regions
        .iter()
        .map(|r| r.price2(grid, &point_to_region))
        .sum()
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

    fn top_left(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x - 1,
            y: self.y - 1,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn top_right(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x + 1,
            y: self.y - 1,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn bottom_left(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x - 1,
            y: self.y + 1,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn bottom_right(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x + 1,
            y: self.y + 1,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn up(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x,
            y: self.y - 1,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn down(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x,
            y: self.y + 1,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn left(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x - 1,
            y: self.y,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
    }

    fn right(&self, grid: &Grid) -> Option<Self> {
        let p = Point {
            x: self.x + 1,
            y: self.y,
        };
        if grid.is_out_of_bounds(&p) {
            return None;
        }
        Some(p)
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

    fn boundary_corners(&self, grid: &Grid, p2r: &HashMap<Point, Region>) -> usize {
        self.inside_corners(grid, p2r) + self.outside_corners(grid, p2r)
    }

    fn inside_corners(&self, grid: &Grid, p2r: &HashMap<Point, Region>) -> usize {
        let patterns = [
            (
                [self.loc.left(&grid), self.loc.up(&grid)],
                [self.loc.top_left(&grid)],
            ),
            // Top right = right+up same species, top-right diff species
            (
                [self.loc.right(&grid), self.loc.up(&grid)],
                [self.loc.top_right(&grid)],
            ),
            // Bottom left = left+down same species, bottom-left diff species
            (
                [self.loc.left(&grid), self.loc.down(&grid)],
                [self.loc.bottom_left(&grid)],
            ),
            // Bottom right = right+down same species, bottom-right diff species
            (
                [self.loc.right(&grid), self.loc.down(&grid)],
                [self.loc.bottom_right(&grid)],
            ),
        ];

        patterns
            .iter()
            .filter(|(same, diff)| {
                same.iter()
                    .all(|p| p.is_some() && p2r.get(&p.unwrap()) == p2r.get(&self.loc))
                    && diff
                        .iter()
                        .all(|p| p.is_some() && p2r.get(&p.unwrap()) != p2r.get(&self.loc))
            })
            .count()
    }

    fn outside_corners(&self, grid: &Grid, p2r: &HashMap<Point, Region>) -> usize {
        let patterns = [
            (
                vec![],
                vec![
                    self.loc.left(&grid),
                    self.loc.up(&grid),
                    self.loc.top_left(&grid),
                ],
            ),
            // Top right = right+up+top-right diff species
            (
                vec![],
                vec![
                    self.loc.right(&grid),
                    self.loc.up(&grid),
                    self.loc.top_right(&grid),
                ],
            ),
            // Bottom left = left+down+bottom-left diff species
            (
                vec![],
                vec![
                    self.loc.left(&grid),
                    self.loc.down(&grid),
                    self.loc.bottom_left(&grid),
                ],
            ),
            // Bottom right = right+down+bottom-right diff species
            (
                vec![],
                vec![
                    self.loc.right(&grid),
                    self.loc.down(&grid),
                    self.loc.bottom_right(&grid),
                ],
            ),
            // Special cases... nasty! :'(
            (
                vec![self.loc.top_left(&grid)],
                vec![self.loc.left(&grid), self.loc.up(&grid)],
            ),
            // Top right = right+up+top-right diff species
            (
                vec![self.loc.top_right(&grid)],
                vec![self.loc.right(&grid), self.loc.up(&grid)],
            ),
            // Bottom left = left+down+bottom-left diff species
            (
                vec![self.loc.bottom_left(&grid)],
                vec![self.loc.left(&grid), self.loc.down(&grid)],
            ),
            // Bottom right = right+down+bottom-right diff species
            (
                vec![self.loc.bottom_right(&grid)],
                vec![self.loc.right(&grid), self.loc.down(&grid)],
            ),
        ];

        patterns
            .iter()
            .filter(|(same, diff)| {
                same.iter()
                    .all(|p| p.is_some() && p2r.get(&p.unwrap()) == p2r.get(&self.loc))
                    && diff.iter().all(|p| {
                        p.is_none() || (p.is_some() && p2r.get(&p.unwrap()) != p2r.get(&self.loc))
                    })
            })
            .count()
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
        self.area() * self.perimeter(grid)
    }

    fn price2(&self, grid: &Grid, point_to_region: &HashMap<Point, Region>) -> usize {
        self.area() * self.sides(grid, point_to_region)
    }

    fn sides(&self, grid: &Grid, point_to_region: &HashMap<Point, Region>) -> usize {
        self.plants
            .iter()
            .map(|p| p.boundary_corners(grid, point_to_region))
            .sum()
    }
}
