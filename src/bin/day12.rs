use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    StartMark,
    EndMark,
    Elevation(u32),
}

impl Cell {
    fn is_bigger_by_one_elevation(&self, other: Cell) -> bool {
        let elevation = if let Cell::Elevation(value) = self {
            *value
        } else if Cell::StartMark == *self {
            'a'.to_digit(10).unwrap()
        } else {
            'z'.to_digit(10).unwrap()
        };

        let other_elevation = if let Cell::Elevation(value) = other {
            value
        } else if Cell::StartMark == other {
            'a'.to_digit(10).unwrap()
        } else {
            'z'.to_digit(10).unwrap()
        };
        elevation > other_elevation && other_elevation + 1 == elevation
    }
}

fn covert_to_index(pos: (usize, usize), width: usize, height: usize) -> usize {
    (pos.1 * height) + pos.1
}

fn convert_to_grid(index: usize, width: usize, height: usize) -> (usize, usize) {
    (index % width, index / height)
}

fn get_adjacents(
    grid: &Vec<Vec<Cell>>,
    index: usize,
    width: usize,
    height: usize
) -> Vec<usize> {
    let node = convert_to_grid(index, width, height);
    let mut adjacents = Vec::new();

    for (x, y) in [(0,1), (1, 0), (-1, 0), (0, -1)] {
        let adjacent:(isize, isize) = (node.0 as isize + x, node.1 as isize + y);
        if adjacent.0 < 0 ||
            adjacent.0 > width as isize ||
            adjacent.1 < 0 ||
            adjacent.1 > height as isize {
                continue
            }
        let adjacent = (adjacent.0 as usize, adjacent.1 as usize);
        let cell = grid[node.1][node.0];
        let next_cell = grid[adjacent.1][adjacent.0];
        if next_cell.is_bigger_by_one_elevation(cell) {
            let next_index = covert_to_index(adjacent, width, height);
            adjacents.push(next_index);
        }
    }
    adjacents
}

fn bfs(grid: &Vec<Vec<Cell>>) {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = vec![false; width*height];
    let mut queue = Vec::new();

    while let Some(node) = queue.pop() {
        println!("{:?}", convert_to_grid(node, width, height));
        for adjacent in get_adjacents(grid, node, width, height) {
            if visited[adjacent] == false {
                queue.push(adjacent);
                visited[adjacent] = true;
            }
        }
    }
}

fn main() -> Result<()> {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let grid: Vec<Vec<Cell>> = std::fs::read_to_string("./data/12.example")?
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
                    _ => {
                        dbg!(c);
                        Cell::Elevation(c.to_digit(10).unwrap())
                    },
                }
            ).collect::<Vec<Cell>>()
        )
        .collect();
    bfs(&grid);
    Ok(())
}