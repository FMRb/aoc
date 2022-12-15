use std::collections::{HashMap, VecDeque};

use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    StartMark,
    EndMark,
    Elevation(char),
}

impl Cell {
    fn is_valid_neighbor(&self, other: Cell) -> bool {
        let elevation = if let Cell::Elevation(value) = self {
            *value
        } else if Cell::StartMark == *self {
            'a'
        } else {
            'z'
        };

        let other_elevation = if let Cell::Elevation(value) = other {
            value
        } else if Cell::StartMark == other {
            'a'
        } else {
            'z'
        };
        elevation <= other_elevation ||
        (other_elevation as u32 + 1) == elevation as u32
    }
}

fn convert_to_index(pos: (usize, usize), width: usize) -> usize {
    (pos.1 * width) + pos.0
}

fn convert_to_grid(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

fn get_adjacents(
    grid: &[Vec<Cell>],
    index: usize,
    width: usize,
    height: usize
) -> Vec<usize> {
    let node = convert_to_grid(index, width);
    let mut adjacents = Vec::new();

    for (x, y) in [(0,1), (1, 0), (-1, 0), (0, -1)] {
        let adjacent:(isize, isize) = (node.0 as isize + x, node.1 as isize + y);
        if adjacent.0 < 0 ||
            adjacent.0 >= width as isize ||
            adjacent.1 < 0 ||
            adjacent.1 >= height as isize {
                continue
            }
        let adjacent = (adjacent.0 as usize, adjacent.1 as usize);
        let cell = grid[node.1][node.0];
        let next_cell = grid[adjacent.1][adjacent.0];
        if next_cell.is_valid_neighbor(cell) {
            let next_index = convert_to_index(adjacent, width);
            adjacents.push(next_index);
        }
    }
    adjacents
}

fn bfs(grid: &Vec<Vec<Cell>>, start_pos: (usize, usize), end_pos: (usize, usize)) -> Option<usize>{
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![false; width*height];
    let mut queue = VecDeque::new();
    let mut parent_track: HashMap<usize, usize> = HashMap::new();

    let index_start = convert_to_index(start_pos, width);
    visited[index_start] = true;
    queue.push_back(index_start);
    while let Some(node) = queue.pop_front() {
        if node == convert_to_index(end_pos, width) {
            let mut child = node;
            let mut path = Vec::new();
            while let Some(parent) = parent_track.get(&child) {
                path.push(convert_to_grid(child, width));
                child = *parent;
            }
            return Some(path.len());
        }
        for adjacent in get_adjacents(grid, node, width, height) {
            if !visited[adjacent] {
                queue.push_back(adjacent);
                parent_track.insert(adjacent, node);
                visited[adjacent] = true;
            }
        }
    }
    None
}

fn find_all_low_elevation(grid: &Vec<Vec<Cell>>, low_elevation: char) -> Vec<(usize, usize)> {
    let mut start_positions = Vec::new();
    for j in 0..grid.len() {
        for i in 0..grid[0].len() {
            let cell = grid[j][i];
            if cell == Cell::StartMark {
                start_positions.push((i, j));
                continue;
            }

            if let Cell::Elevation(c) = cell {
                if c == low_elevation {
                    start_positions.push((i, j));
                }

            }

        }
    }
    start_positions
}

fn main() -> Result<()> {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let grid: Vec<Vec<Cell>> = std::fs::read_to_string("./data/12.input")?
        .lines()
        .enumerate()
        .map(|(j, line)|
            line.chars()
            .enumerate()
            .map(|(i, c)| 
                match c {
                    'S' => {
                        start_pos = (i, j);
                        Cell::StartMark
                    },
                    'E' => {
                        end_pos = (i, j);
                        Cell::EndMark
                    },
                    _ => Cell::Elevation(c),
                }
            ).collect::<Vec<Cell>>()
        )
        .collect();

    let starting_points = find_all_low_elevation(&grid, 'a');

    let part1_steps = bfs(&grid, start_pos, end_pos).unwrap();
    println!("Part 1: {:?}", part1_steps);
    let mut steps = Vec::new();
    for start_point in starting_points {
        if let Some(solution_steps) = bfs(&grid, start_point, end_pos) {
            steps.push(solution_steps);
        }
    }
    steps.sort();
    println!("Part 2: {:?}", steps[0]);
    Ok(())
}