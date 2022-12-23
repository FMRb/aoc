use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Wall,
    Open,
    Empty,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Board {
    map: Vec<Vec<Tile>>,
    current_position: (usize, usize),
    facing: Direction,
}

enum CubeFace {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Board {
    const CUBE_SIZE: usize = 50;

    fn new(map: Vec<Vec<Tile>>) -> Self {
        let (x, _) = map[0]
            .iter()
            .enumerate()
            .find(|(_, tile)| **tile == Tile::Open)
            .unwrap();
        Self {
            map,
            current_position: (x, 0),
            facing: Direction::Right,
        }
    }

    fn cube_face(&self, pos: &(usize, usize)) -> CubeFace {
        let range_a = 0..=(Board::CUBE_SIZE - 1);
        let range_b = Board::CUBE_SIZE..=(Board::CUBE_SIZE * 2 - 1);
        let range_c = (Board::CUBE_SIZE * 2)..=(Board::CUBE_SIZE * 3 - 1);
        let range_d = (Board::CUBE_SIZE * 3)..=(Board::CUBE_SIZE * 4 - 1);
        match *pos {
            (x, y) if range_b.contains(&x) && range_a.contains(&y) => CubeFace::One, // 1
            (x, y) if range_c.contains(&x) && range_a.contains(&y) => CubeFace::Two, // 2
            (x, y) if range_b.contains(&x) && range_b.contains(&y) => CubeFace::Three,
            (x, y) if range_a.contains(&x) && range_c.contains(&y) => CubeFace::Four, // 4
            (x, y) if range_b.contains(&x) && range_c.contains(&y) => CubeFace::Five, // 5
            (x, y) if range_a.contains(&x) && range_d.contains(&y) => CubeFace::Six,  // 3
            _ => panic!("Range cube face wrong {pos:?}"),
        }
    }

    fn pos_to_vec(&self, pos: &(usize, usize)) -> Tile {
        self.map[pos.1][pos.0]
    }

    fn is_out_of_bounds(&self, pos: &(isize, isize)) -> bool {
        if pos.0 < 0 || pos.1 < 0 || self.map.len() <= pos.1 as usize {
            return true;
        }
        let pos = (pos.0 as usize, pos.1 as usize);

        if pos.0 >= self.map[pos.1].len() {
            return true;
        }

        if self.pos_to_vec(&pos) == Tile::Empty {
            return true;
        }

        false
    }

    fn regular_next_pos(&self) -> ((usize, usize), Direction) {
        let delta = match self.facing {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_pos = (
            self.current_position.0 as isize + delta.0,
            self.current_position.1 as isize + delta.1,
        );

        if self.is_out_of_bounds(&new_pos) {
            match self.facing {
                Direction::Up => {
                    let mut temp_pos = (self.current_position.0, self.map.len() - 1);
                    loop {
                        if let Some(tile) = self.map[temp_pos.1].get(temp_pos.0) {
                            if *tile != Tile::Empty {
                                break;
                            }
                        }
                        temp_pos = (temp_pos.0, temp_pos.1 - 1);
                    }
                    (temp_pos, self.facing)
                }
                Direction::Down => {
                    let mut temp_pos = (self.current_position.0, 0);
                    loop {
                        if let Some(tile) = self.map[temp_pos.1].get(temp_pos.0) {
                            if *tile != Tile::Empty {
                                break;
                            }
                        }
                        temp_pos = (temp_pos.0, temp_pos.1 + 1);
                    }
                    (temp_pos, self.facing)
                }
                Direction::Left => {
                    let mut temp_pos = (
                        self.map[self.current_position.1].len() - 1,
                        self.current_position.1,
                    );
                    while self.map[temp_pos.1][temp_pos.0 as usize] == Tile::Empty {
                        temp_pos = (temp_pos.0 - 1, temp_pos.1);
                    }
                    (temp_pos, self.facing)
                }
                Direction::Right => {
                    let mut temp_pos = (0, self.current_position.1);
                    while self.map[temp_pos.1][temp_pos.0 as usize] == Tile::Empty {
                        temp_pos = (temp_pos.0 + 1, temp_pos.1);
                    }
                    (temp_pos, self.facing)
                }
            }
        } else {
            ((new_pos.0 as usize, new_pos.1 as usize), self.facing)
        }
    }

    fn cube_next_pos(&self) -> ((usize, usize), Direction) {
        let delta = match self.facing {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_pos = (
            self.current_position.0 as isize + delta.0,
            self.current_position.1 as isize + delta.1,
        );

        if self.is_out_of_bounds(&new_pos) {
            let cube_face = self.cube_face(&self.current_position);
            match self.facing {
                Direction::Up => match cube_face {
                    CubeFace::One => (
                        (0, self.current_position.0 + (Board::CUBE_SIZE * 2)),
                        Direction::Right,
                    ),
                    CubeFace::Two => (
                        (
                            self.current_position.0 - (Board::CUBE_SIZE * 2),
                            self.map.len() - 1,
                        ),
                        Direction::Up,
                    ),
                    CubeFace::Three => panic!("This is not allowed"),
                    CubeFace::Four => (
                        (Board::CUBE_SIZE, self.current_position.0 + Board::CUBE_SIZE),
                        Direction::Right,
                    ),
                    CubeFace::Five => panic!("This is not allowed"),
                    CubeFace::Six => panic!("This is not allowed"),
                },
                Direction::Down => match cube_face {
                    CubeFace::One => panic!("This is not allowed"),
                    CubeFace::Two => (
                        (
                            (Board::CUBE_SIZE * 2) - 1,
                            self.current_position.0 - Board::CUBE_SIZE,
                        ),
                        Direction::Left,
                    ),
                    CubeFace::Three => panic!("This is not allowed"),
                    CubeFace::Four => panic!("This is not allowed"),
                    CubeFace::Five => (
                        (
                            Board::CUBE_SIZE - 1,
                            self.current_position.0 + (Board::CUBE_SIZE * 2),
                        ),
                        Direction::Left,
                    ),
                    CubeFace::Six => (
                        (self.current_position.0 + (Board::CUBE_SIZE * 2), 0),
                        Direction::Down,
                    ),
                },
                Direction::Left => match cube_face {
                    CubeFace::One => (
                        (0, (Board::CUBE_SIZE * 3) - 1 + self.current_position.1),
                        Direction::Right,
                    ),
                    CubeFace::Two => panic!("This is not allowed"),
                    CubeFace::Three => (
                        (
                            self.current_position.1 - Board::CUBE_SIZE,
                            Board::CUBE_SIZE * 2,
                        ),
                        Direction::Down,
                    ),
                    CubeFace::Four => (
                        (
                            self.current_position.1 - Board::CUBE_SIZE,
                            Board::CUBE_SIZE * 2,
                        ),
                        Direction::Down,
                    ),
                    CubeFace::Five => panic!("This is not allowed"),
                    CubeFace::Six => (
                        (self.current_position.1 - Board::CUBE_SIZE * 2, 0),
                        Direction::Down,
                    ),
                },
                Direction::Right => match cube_face {
                    CubeFace::One => panic!("This is not allowed"),
                    CubeFace::Two => (
                        (
                            Board::CUBE_SIZE * 2 - 1,
                            (Board::CUBE_SIZE * 3 - 1) - self.current_position.1,
                        ),
                        Direction::Left,
                    ),
                    CubeFace::Three => (
                        (
                            self.current_position.1 + Board::CUBE_SIZE,
                            Board::CUBE_SIZE - 1,
                        ),
                        Direction::Up,
                    ),
                    CubeFace::Four => panic!("This is not allowed"),
                    CubeFace::Five => (
                        (
                            Board::CUBE_SIZE * 3 - 1,
                            (Board::CUBE_SIZE * 3 - 1) - self.current_position.1,
                        ),
                        Direction::Left,
                    ),
                    CubeFace::Six => (
                        (
                            self.current_position.1 - (Board::CUBE_SIZE * 2),
                            Board::CUBE_SIZE * 3 - 1,
                        ),
                        Direction::Up,
                    ),
                },
            }
        } else {
            ((new_pos.0 as usize, new_pos.1 as usize), self.facing)
        }
    }

    fn cube_next_pos_example(&self) -> ((usize, usize), Direction) {
        let delta = match self.facing {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let new_pos = (
            self.current_position.0 as isize + delta.0,
            self.current_position.1 as isize + delta.1,
        );

        if self.is_out_of_bounds(&new_pos) {
            let cube_face = self.cube_face(&self.current_position);
            match self.facing {
                Direction::Up => match cube_face {
                    CubeFace::One => ((self.current_position.0, self.map.len() - 1), Direction::Up),
                    CubeFace::Two => ((self.current_position.0 + 8, 0), Direction::Down),
                    CubeFace::Three => ((8, self.current_position.0 - 4), Direction::Right),
                    CubeFace::Four => panic!("This is not allowed"),
                    CubeFace::Five => panic!("This is not allowed"),
                    CubeFace::Six => ((11, 19 - self.current_position.0), Direction::Left),
                },
                Direction::Down => match cube_face {
                    CubeFace::One => panic!("This is not allowed"),
                    CubeFace::Two => (
                        (11 - self.current_position.0, self.map.len() - 1),
                        Direction::Up,
                    ),
                    CubeFace::Three => ((8, 15 - self.current_position.0), Direction::Right),
                    CubeFace::Four => panic!("This is not allowed"),
                    CubeFace::Five => ((11 - self.current_position.0, 7), Direction::Up),
                    CubeFace::Six => ((0, 19 - self.current_position.0), Direction::Right),
                },
                Direction::Left => match cube_face {
                    CubeFace::One => ((self.current_position.1 + 4, 4), Direction::Down),
                    CubeFace::Two => ((19 - self.current_position.1, 11), Direction::Up),
                    CubeFace::Three => panic!("This is not allowed"),
                    CubeFace::Four => panic!("This is not allowed"),
                    CubeFace::Five => ((15 - self.current_position.1, 7), Direction::Up),
                    CubeFace::Six => panic!("This is not allowed"),
                },
                Direction::Right => match cube_face {
                    CubeFace::One => ((15, 11 - self.current_position.1), Direction::Left),
                    CubeFace::Two => panic!("This is not allowed"),
                    CubeFace::Three => panic!("This is not allowed"),
                    CubeFace::Four => ((19 - self.current_position.1, 8), Direction::Down),
                    CubeFace::Five => panic!("This is not allowed"),
                    CubeFace::Six => ((11, 11 - self.current_position.1), Direction::Left),
                },
            }
        } else {
            ((new_pos.0 as usize, new_pos.1 as usize), self.facing)
        }
    }

    fn move_position(&mut self, instruction: Instruction, has_cube_wrapping: bool) {
        match instruction {
            Instruction::Move(steps) => {
                println!(
                    "Current position: {:?} and facing {:?}",
                    self.current_position, self.facing
                );
                println!("Moving instruction {instruction:?}");

                for _ in 0..steps {
                    let (new_pos, new_direction) = if has_cube_wrapping {
                        self.cube_next_pos()
                    } else {
                        self.regular_next_pos()
                    };
                    let new_tile = self.pos_to_vec(&new_pos);
                    if new_tile == Tile::Open {
                        self.current_position = (new_pos.0 as usize, new_pos.1 as usize);
                        self.facing = new_direction;
                        println!(
                            "Update position: {:?} - {:?}",
                            self.current_position, self.facing
                        );
                    } else if new_tile == Tile::Wall {
                        println!("hitting wall: {:?}", self.current_position);
                        break;
                    } else {
                        panic!("You shouldn't reach empty tile {:?}", new_pos);
                    }
                }
            }
            Instruction::Right => {
                self.facing = match self.facing {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            Instruction::Left => {
                self.facing = match self.facing {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
        }
    }

    fn value_facing(&self) -> u32 {
        match self.facing {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

fn parse_map(map: &str) -> Board {
    let mut board = Vec::new();
    for line in map.lines() {
        let mut rows = Vec::new();
        for c in line.chars() {
            let tile = match c {
                ' ' => Tile::Empty,
                '.' => Tile::Open,
                '#' => Tile::Wall,
                _ => panic!("Found wrong char"),
            };
            rows.push(tile);
        }
        board.push(rows);
    }
    Board::new(board)
}

#[derive(Debug)]
enum Instruction {
    Move(u32),
    Right,
    Left,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut num = 0;
    let mut instructions = Vec::new();
    for c in input.chars() {
        match c {
            '0'..='9' => {
                let n = c.to_digit(10).unwrap();
                if num == 0 {
                    num = n
                } else {
                    num = (num * 10) + n;
                }
            }
            'R' => {
                instructions.push(Instruction::Move(num));
                num = 0;
                instructions.push(Instruction::Right);
            }
            'L' => {
                instructions.push(Instruction::Move(num));
                num = 0;
                instructions.push(Instruction::Left);
            }
            _ => panic!("Wrong char instruction"),
        }
    }
    instructions.push(Instruction::Move(num));
    instructions
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/22.input")?;

    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut board = parse_map(map);
    let instructions = parse_instructions(instructions);

    for instruction in instructions {
        board.move_position(instruction, true);
    }

    // println!("Solution {:?} - {:?}", board.current_position, board.facing);
    let password = 1000 * (board.current_position.1 as u32 + 1)
        + 4 * (board.current_position.0 as u32 + 1)
        + board.value_facing();
    println!("Password {password}");

    Ok(())
}
