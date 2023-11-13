use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn (std::error::Error)>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <path_to_input>");
        std::process::exit(1);
    }
    println!("Argument {}", args[1]);
    let path = String::from(&args[1]);
    let input = fs::read_to_string(&path)?;
    let p1 = part_one(&input);
    println!("Result part 1: {}", p1);
    let p2 = part_two(&input);
    println!("Result part 2: {}", p2);
    Ok(())
}

type Coordinates = (i32, i32, i32);

#[derive(Debug, Default, PartialEq, Eq)]
struct Grid {
    cube_map: HashMap<Coordinates, bool>,
}

const NEIGHBORS_2D: [(i32, i32); 9] = [
    (0, 0),
    (1, 0),
    (0, 1),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

const OUTER_NEIGHBORS: [i32; 3] = [1, 0, -1];

/*
 xxx
 xxx
 xxx
*/

impl Grid {
    fn compute_new_state(&self, target: Coordinates) -> bool {
        let mut active_counter = 0;
        for &z in OUTER_NEIGHBORS.iter() {
            for i in 0..NEIGHBORS_2D.len() {
                if z == 0 && i == 0 {
                    continue;
                }
                let (x, y) = NEIGHBORS_2D[i];
                let neighbor_coordinates = (target.0 + x, target.1 + y, target.2 + z);
                if let Some(&is_active) = self.cube_map.get(&neighbor_coordinates) {
                    if is_active {
                        active_counter += 1;
                    }
                }
            }
        }

        let state = if let Some(&s) = self.cube_map.get(&target) {
            s
        } else {
            false
        };

        // If a cube is active and exactly 2 or 3 of its neighbors are also active,
        // the cube remains active. Otherwise, the cube becomes inactive.
        //
        // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
        // Otherwise, the cube remains inactive.
        match active_counter {
            2 | 3 if state => true,
            3 if !state => true,
            _ => false,
        }
    }

    fn visit_neighbors(&self, target: Coordinates) -> Vec<(Coordinates, bool)> {
        let mut visited_neighbors = Vec::with_capacity(26);
        for z_dim in OUTER_NEIGHBORS.iter() {
            for d in NEIGHBORS_2D.iter() {
                let neighbor_coordinate = (target.0 + d.0, target.1 + d.1, target.2 + z_dim);
                let new_neighbor_state = self.compute_new_state(neighbor_coordinate);
                visited_neighbors.push((neighbor_coordinate, new_neighbor_state));
            }
        }
        visited_neighbors
    }

    fn get_active_cubes(&self) -> Vec<Coordinates> {
        self.cube_map
            .iter()
            .filter(|&(_, value)| *value)
            .map(|(key, _)| *key)
            .collect()
    }
}

type Coord4 = (i32, i32, i32, i32);

#[derive(Debug, PartialEq, Eq, Default)]
struct Grid4D {
    cube_map: HashMap<Coord4, bool>,
}

impl Grid4D {
    fn neighbors(&self, origin: Coord4) -> Vec<(Coord4, bool)> {
        let mut neighbors = Vec::new();
        let (o_x, o_y, o_z, o_w) = origin;
        for &w in OUTER_NEIGHBORS.iter() {
            for &z in OUTER_NEIGHBORS.iter() {
                for i in 0..NEIGHBORS_2D.len() {
                    if w == 0 && z == 0 && i == 0 {
                        continue;
                    }

                    let (x, y) = NEIGHBORS_2D[i];
                    let coordinates = (o_x + x, o_y + y, o_z + z, o_w + w);
                    if let Some(&is_active) = self.cube_map.get(&coordinates) {
                        neighbors.push((coordinates, is_active));
                    } else {
                        neighbors.push((coordinates, false));
                    }
                }
            }
        }
        neighbors
    }

    fn compute_new_state(&self, origin: Coord4) -> bool {
        let neighbors = self.neighbors(origin);
        let active_counter = neighbors.iter().filter(|&(_, a)| *a).count();

        let state = if let Some(&s) = self.cube_map.get(&origin) {
            s
        } else {
            false
        };

        // If a cube is active and exactly 2 or 3 of its neighbors are also active,
        // the cube remains active. Otherwise, the cube becomes inactive.
        //
        // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
        // Otherwise, the cube remains inactive.
        match active_counter {
            2 | 3 if state => true,
            3 if !state => true,
            _ => false,
        }
    }

    fn get_active_cubes(&self) -> Vec<Coord4> {
        self.cube_map
            .iter()
            .filter(|&(_, value)| *value)
            .map(|(key, _)| *key)
            .collect()
    }
}

// .#.
// ..#
// ###
fn parse_cubes_3d(input: &str) -> Grid {
    let mut grid = Grid::default();
    let mut y = 0;
    for line in input.lines() {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {
                grid.cube_map.insert((x as i32, y, 0), false);
            }
            '#' => {
                grid.cube_map.insert((x as i32, y, 0), true);
            }
            _ => panic!("Unknown char in the input"),
        });
        y += 1;
    }
    grid
}

fn parse_cubes_4d(input: &str) -> Grid4D {
    let mut grid = Grid4D::default();
    let mut y = 0;
    for line in input.lines() {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {
                grid.cube_map.insert((x as i32, y, 0, 0), false);
            }
            '#' => {
                grid.cube_map.insert((x as i32, y, 0, 0), true);
            }
            _ => panic!("Unknown char in the input"),
        });
        y += 1;
    }
    grid
}

fn run(grid: &Grid) -> Grid {
    let mut new_grid = Grid::default();
    let active_cubes = grid.get_active_cubes();
    for cube in active_cubes {
        let neighbors = grid.visit_neighbors(cube);
        let new_state = grid.compute_new_state(cube);

        neighbors.iter().for_each(|&(coordinate, state)| {
            new_grid.cube_map.insert(coordinate, state);
        });
        new_grid.cube_map.insert(cube, new_state);
    }
    new_grid
}

fn run_4d(grid: &Grid4D) -> Grid4D {
    let mut new_grid = Grid4D::default();
    let active_cubes = grid.get_active_cubes();
    for cube in active_cubes {
        let neighbors = grid.neighbors(cube);
        let new_state = grid.compute_new_state(cube);

        neighbors.iter().for_each(|&(coordinate, _)| {
            let new_state = grid.compute_new_state(coordinate);
            new_grid.cube_map.insert(coordinate, new_state);
        });
        new_grid.cube_map.insert(cube, new_state);
    }
    new_grid
}

fn part_one(input: &str) -> usize {
    let mut grid = parse_cubes_3d(input);
    for _ in 0..6 {
        grid = run(&grid);
    }
    grid.get_active_cubes().len()
}

fn part_two(input: &str) -> usize {
    let mut grid = parse_cubes_4d(input);
    for _ in 0..6 {
        grid = run_4d(&grid);
    }
    grid.get_active_cubes().len()
}
