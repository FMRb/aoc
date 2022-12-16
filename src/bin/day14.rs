use std::{cmp, collections::HashMap};

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct RockPath {
    path: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Rock,
    RestSand,
}

#[derive(Debug, Clone)]
struct Grid {
    source_sand: (u32, u32),
    cells: HashMap<(u32, u32), CellType>,
    top_left: (u32, u32),
    bottom_right: (u32, u32)
}

impl Grid {
    fn new(source_sand: (u32, u32)) -> Self {
        let cells = HashMap::new();
        Self { source_sand, cells, top_left: (0, 0), bottom_right: (0, 0)}
    }

    fn place_rocks(&mut self, rock_path: RockPath) {
        let mut last_position = rock_path.path[0];
        for &(x, y) in rock_path.path.iter() {
            self.cells.insert((x, y), CellType::Rock);

            if x != last_position.0 {
                let max_x = cmp::max(x, last_position.0);
                let min_x = cmp::min(x, last_position.0);
                for i in min_x..max_x {
                    self.cells.insert((i, y), CellType::Rock);
                }
            } else if y != last_position.1 {
                let max_y = cmp::max(y, last_position.1);
                let min_y = cmp::min(y, last_position.1);
                for j in min_y..max_y {
                    self.cells.insert((x, j), CellType::Rock);
                }
            }
            last_position = (x, y);
        }
    }

    fn set_top_left(&mut self, top_left: (u32, u32)) {
        self.top_left = (top_left.0, 0);
    }
    
    fn set_bottom_right(&mut self, bottom_right: (u32, u32)) {
        self.bottom_right = bottom_right;
    }

    fn max_min_grid_values(&self) -> ((u32, u32), (u32, u32)) {
        let mut x_max = 0;
        let mut y_max = 0;
        let mut x_min = u32::MAX;
        let mut y_min = u32::MAX;

        for (&(x, y), _cell) in self.cells.iter() {
            if x_max < x {
                x_max = x;
            }

            if y_max < y {
                y_max = y;
            }

            if x_min > x {
                x_min = x;
            }

            if y_min > y {
                y_min = y;
            }
        }

        ((x_max, y_max), (x_min, y_min))
    }

    fn set_grid_floor(&mut self, floor: u32) {
        for i in 0..1000 {
            self.cells.insert((i, floor), CellType::Rock);
        }
    }

    fn count_rest_sand(&self) -> u32 {
        let mut counter = 0;
        for cell in self.cells.values() {
            if *cell == CellType::RestSand {
                counter += 1;
            }
        } 
        counter
    }

    fn simulate_sand_until_reaching_abyss(&mut self, floor_limit: bool, floor: u32) -> bool {

        let mut sand_position = self.source_sand;
        let mut reach_abyss = false;

        loop {
            let peek_down_position = (sand_position.0, sand_position.1 + 1);
            let peek_left_diagonal_position = (sand_position.0 - 1, sand_position.1 + 1);
            let peek_right_diagonal_position = (sand_position.0 + 1, sand_position.1 + 1);
            match (
                self.cells.get(&peek_down_position),
                self.cells.get(&peek_left_diagonal_position),
                self.cells.get(&peek_right_diagonal_position),
            ) {
                (None, None, None) => sand_position = peek_down_position,
                (None, None, Some(_)) => {
                    sand_position = peek_down_position;
                }
                (None, Some(_), None) => {
                    sand_position = peek_down_position;
                }
                (None, Some(_), Some(_)) => {
                    sand_position = peek_down_position;
                }
                (Some(_), None, None) => {
                    sand_position = peek_left_diagonal_position;
                }
                (Some(_), None, Some(_)) => {
                    sand_position = peek_left_diagonal_position;
                }
                (Some(_), Some(_), None) => {
                    sand_position = peek_right_diagonal_position;
                }
                (Some(_), Some(_), Some(_)) => {
                    self.cells.insert(sand_position, CellType::RestSand);
                    break;
                }
            }
            if !floor_limit && sand_position.1 > floor {
                reach_abyss = true;
                break;
            } 
        }
        reach_abyss
    }

    fn display(&self) {
        let (x_min, y_min) = self.top_left;
        let (x_max, y_max) = self.bottom_right;

        for y in y_min..y_max+3 {
            for x in x_min-200..x_max+200 {
                match self.cells.get(&(x as u32, y as u32)) {
                    Some(cell) => match cell {
                        CellType::Rock => print!("#"),
                        CellType::RestSand => print!("o"),
                    },
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

fn parse_path(input: &str) -> IResult<&str, RockPath> {
    let (input, path) = separated_list1(
        tag(" -> "),
        separated_pair(complete::u32, tag(","), complete::u32),
    )(input)?;
    Ok((input, RockPath { path }))
}

fn parse_scan(input: &str) -> IResult<&str, Vec<RockPath>> {
    separated_list1(newline, parse_path)(input)
}

fn exercise_one(mut grid: Grid) -> u32 {
    loop {
        if grid.simulate_sand_until_reaching_abyss(false, grid.bottom_right.1) {
            break;
        }
    }

    grid.count_rest_sand()

}

fn exercise_two(mut grid: Grid) -> u32 {
    grid.set_grid_floor(grid.bottom_right.1 +2);
    loop {
        grid.simulate_sand_until_reaching_abyss(true, grid.bottom_right.1 + 2);
        if let Some(cell) = grid.cells.get(&grid.source_sand) {
            if *cell == CellType::RestSand {
                break;
            }
        }
    }
    grid.display();
    grid.count_rest_sand()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/14.input")?;

    let (_, rock_paths) = parse_scan(&input).unwrap();
    let mut grid = Grid::new((500, 0));
    for path in rock_paths {
        grid.place_rocks(path);
    }

    let (bottom_right, top_left) = grid.max_min_grid_values();
    grid.set_top_left(top_left);
    grid.set_bottom_right(bottom_right);

    let part1 = exercise_one(grid.clone());
    let part2 = exercise_two(grid.clone());

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
