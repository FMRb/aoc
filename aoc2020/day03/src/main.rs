use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn (std::error::Error)>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: <path_to_input>");
        std::process::exit(1);
    }
    println!("Argument {}", args[1]);
    let path = String::from(&args[1]);
    let input = fs::read_to_string(&path)?;
    let p1 = part_one(&input)?;
    println!("Result part 1: {}", p1);
    let p2 = part_two(&input)?;
    println!("Result part 2: {}", p2);
    Ok(())
}

///// SAMPLE ///
// ..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#
#[derive(Debug,Clone)]
struct ForestMap {
    rows: usize,
    columns: usize,
    value: Vec<char>,
    position_x: usize,
    position_y: usize,
}

// squares (.) and trees (#)
impl ForestMap {
    fn new(rows: usize, columns: usize, value: Vec<char>) -> Self {
        Self {rows, columns, value: value, position_x: 0, position_y: 0}
    }

    fn is_tree(&self) -> bool {
        self.value[(self.position_y*self.columns)+(self.position_x % self.columns)] == '#'
    }

    fn move_in_forest(&mut self, slope_x: usize, slope_y: usize) {
        self.position_x += slope_x;
        self.position_y += slope_y;
    }

    fn reset(&mut self) {
        self.position_x = 0;
        self.position_y = 0;
    }
}

fn read_input(input: &str) -> io::Result<ForestMap> {
    let rows = input.lines().count();
    let columns = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .count();

    let forest: Vec<char> = input
        .lines()
        .flat_map(|line| line.chars())
        .collect();

    Ok(ForestMap::new(rows, columns, forest))
}

fn part_one(input: &str) -> io::Result<u32> {
    let mut tree_counter = 0;
    let mut forest = read_input(input).unwrap();

    let slope_x = 3;
    let slope_y = 1;

    while forest.position_y < forest.rows {
        if forest.is_tree() {
            tree_counter +=1
        }
        forest.move_in_forest(slope_x, slope_y);
    }


    Ok(tree_counter)
}

fn part_two(input: &str) -> io::Result<u32> {
    let slopes = [
        (1,1),
        (3,1),
        (5,1),
        (7,1),
        (1,2)
    ];
    let mut tress_on_slopes = Vec::with_capacity(slopes.len());

    let mut forest = read_input(input).unwrap();

    for (slope_x, slope_y) in slopes.iter() {
        let mut tree_counter = 0;
        while forest.position_y < forest.rows {
            if forest.is_tree() {
                tree_counter +=1
            }
            forest.move_in_forest(*slope_x, *slope_y);
        }
        forest.reset();
        tress_on_slopes.push(tree_counter);
    }

    Ok(tress_on_slopes.iter().product())
}