use anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tree {
    height: u32,
    visible_top: bool,
    visible_bottom: bool,
    visible_left: bool,
    visible_right: bool,
    scenic_score: u32,
}

impl Tree {
    fn new(height: u32) -> Self {
        Self {
            height,
            visible_top: false,
            visible_bottom: false,
            visible_left: false,
            visible_right: false,
            scenic_score: 0,
        }
    }

    fn is_visible(&self) -> bool {
        self.visible_top || self.visible_bottom || self.visible_left || self.visible_right
    }
}

struct Grid {
    value: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(value: Vec<Vec<Tree>>) -> Self {
        let height = value.len();
        let width = value[0].len();
        Self {
            value,
            width,
            height,
        }
    }

    fn is_edge(&self, x: usize, y: usize) -> bool {
        if y == self.height - 1 || y == 0 {
            return true;
        }

        if x == self.width - 1 || x == 0 {
            return true;
        }

        return false;
    }

    fn is_tree_visible_up(&self, tree_height: u32, x: usize, y: usize) -> bool {
        if self.is_edge(x, y) {
            return tree_height > self.value[y][x].height;
        }
        tree_height > self.value[y][x].height && self.is_tree_visible_up(tree_height, x, y - 1)
    }

    fn is_tree_visible_down(&self, tree_height: u32, x: usize, y: usize) -> bool {
        if self.is_edge(x, y) {
            return tree_height > self.value[y][x].height;
        }
        tree_height > self.value[y][x].height && self.is_tree_visible_down(tree_height, x, y + 1)
    }

    fn is_tree_visible_left(&self, tree_height: u32, x: usize, y: usize) -> bool {
        if self.is_edge(x, y) {
            return tree_height > self.value[y][x].height;
        }
        tree_height > self.value[y][x].height && self.is_tree_visible_left(tree_height, x - 1, y)
    }

    fn is_tree_visible_right(&self, tree_height: u32, x: usize, y: usize) -> bool {
        if self.is_edge(x, y) {
            return tree_height > self.value[y][x].height;
        }
        tree_height > self.value[y][x].height && self.is_tree_visible_right(tree_height, x + 1, y)
    }

    fn scenic_looking_up(&self, tree_height: u32, x: usize, y: usize) -> u32 {
        if self.is_edge(x, y) {
            return 1
        }

        if tree_height <= self.value[y][x].height {
            return 1;
        }

        return 1 + self.scenic_looking_up(tree_height, x, y - 1);
    }

    fn scenic_looking_down(&self, tree_height: u32, x: usize, y: usize) -> u32 {
        if self.is_edge(x, y) {
            return 1
        }

        if tree_height <= self.value[y][x].height {
            return 1;
        }
        return 1 + self.scenic_looking_down(tree_height, x, y + 1);
    }

    fn scenic_looking_left(&self, tree_height: u32, x: usize, y: usize) -> u32 {
        if self.is_edge(x, y) {
            return 1
        }

        if tree_height <= self.value[y][x].height {
            return 1;
        }

        return 1 + self.scenic_looking_left(tree_height, x - 1, y);
    }

    fn scenic_looking_right(&self, tree_height: u32, x: usize, y: usize) -> u32 {
        if self.is_edge(x, y) {
            return 1
        }

        if tree_height <= self.value[y][x].height {
            return 1;
        }

        return 1 + self.scenic_looking_right(tree_height, x + 1, y);
    }

    fn compute_visible(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let tree = self.value[j][i];
                if self.is_edge(i, j) {
                    self.value[j][i].visible_top = true;
                    continue;
                }
                // Move to the next position already
                self.value[j][i].visible_top = self.is_tree_visible_up(tree.height, i, j - 1);
                self.value[j][i].visible_bottom = self.is_tree_visible_down(tree.height, i, j + 1);
                self.value[j][i].visible_left = self.is_tree_visible_left(tree.height, i - 1, j);
                self.value[j][i].visible_right = self.is_tree_visible_right(tree.height, i + 1, j);
            }
        }
    }

    fn compute_scenic_distance(&mut self) {
        for j in 0..self.height {
            for i in 0..self.width {
                let tree = self.value[j][i];
                if self.is_edge(i, j) {
                    self.value[j][i].scenic_score = 0;
                    continue;
                }
                // Move to the next position already
                let scenic_top = self.scenic_looking_up(tree.height, i, j - 1);
                let scenic_bottom = self.scenic_looking_down(tree.height, i, j + 1);
                let scenic_left = self.scenic_looking_left(tree.height, i - 1, j);
                let scenic_right = self.scenic_looking_right(tree.height, i + 1, j);
                self.value[j][i].scenic_score =
                    scenic_top * scenic_bottom * scenic_left * scenic_right;
            }
        }
    }

    fn highest_scenic(&self) -> u32 {
      self.value
        .iter()
        .map(|row| row
          .iter()
          .map(|tree| tree.scenic_score)
          .max()
          .unwrap()
        )
        .max().unwrap()
    }

    fn count_visible(&self) -> usize {
        self.value
            .iter()
            .map(|row| row.iter().filter(|t| t.is_visible()).count())
            .sum()
    }
}

fn exercise_one(grid: &mut Grid) -> usize {
    grid.compute_visible();

    grid.count_visible()
}

fn exercise_two(grid: &mut Grid) -> u32 {
    grid.compute_scenic_distance();
    grid.highest_scenic()
}

fn main() -> Result<()> {
    let grid: Vec<Vec<Tree>> = std::fs::read_to_string("./data/8.input")?
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|d| Tree::new(d))
                .collect::<Vec<Tree>>()
        })
        .collect();

    let mut grid = Grid::new(grid);
    let result = exercise_one(&mut grid);
    let result_2 = exercise_two(&mut grid);

    println!("Part 1: {result}");
    println!("Part 2: {result_2}");
    Ok(())
}
