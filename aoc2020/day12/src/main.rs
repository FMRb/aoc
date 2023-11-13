use std::collections::LinkedList;
use std::env;
use std::fs;
use std::{result::Result, str::FromStr};

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum NavigationAction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct BadNavigationAction;

impl NavigationAction {
    fn parse_char(c: char) -> Result<NavigationAction, BadNavigationAction> {
        match c {
            'N' => Ok(NavigationAction::North),
            'S' => Ok(NavigationAction::South),
            'E' => Ok(NavigationAction::East),
            'W' => Ok(NavigationAction::West),
            'L' => Ok(NavigationAction::Left),
            'R' => Ok(NavigationAction::Right),
            'F' => Ok(NavigationAction::Forward),
            _ => Err(BadNavigationAction),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Eq)]
enum CompassPoint {
    North,
    East,
    South,
    West,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct NavigationInstruction {
    action: NavigationAction,
    value: i32,
}

struct BadNavigationInstruction;

impl FromStr for NavigationInstruction {
    type Err = BadNavigationInstruction;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);
        let navigation_action =
            NavigationAction::parse_char(action.chars().next().unwrap()).unwrap();
        let value = value.parse::<i32>().unwrap();
        Ok(Self {
            action: navigation_action,
            value,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Ferry {
    direction_index: u32,
    direction_points: [CompassPoint; 4],
    waypoint: LinkedList<i32>,
    position: (i32, i32, i32, i32), // (North,East,South,West)
}

impl Ferry {
    fn new() -> Self {
        let mut waypoint_values = LinkedList::new();
        waypoint_values.push_back(1); // Currently North
        waypoint_values.push_back(10); // Currently East
        waypoint_values.push_back(0);
        waypoint_values.push_back(0);
        Self {
            direction_index: 1,
            direction_points: [
                CompassPoint::North,
                CompassPoint::East,
                CompassPoint::South,
                CompassPoint::West,
            ],
            waypoint: waypoint_values,
            position: (0, 0, 0, 0),
        }
    }

    fn instruct_move(&mut self, instruction: NavigationInstruction) {
        let (ref mut north_pos, ref mut east_pos, ref mut south_pos, ref mut west_pos) =
            &mut self.position;
        match instruction.action {
            NavigationAction::North => {
                *north_pos += instruction.value;
            }
            NavigationAction::South => {
                *south_pos += instruction.value;
            }
            NavigationAction::East => {
                *east_pos += instruction.value;
            }
            NavigationAction::West => {
                *west_pos += instruction.value;
            }
            NavigationAction::Right => match instruction.value {
                90 => {
                    self.direction_index =
                        (self.direction_index + 1) % self.direction_points.len() as u32;
                }
                180 => {
                    self.direction_index =
                        (self.direction_index + 2) % self.direction_points.len() as u32;
                }
                270 => {
                    self.direction_index =
                        (self.direction_index + 3) % self.direction_points.len() as u32;
                }
                _ => {
                    panic!("Wrong value for Right {}", instruction.value)
                }
            },
            NavigationAction::Left => match instruction.value {
                90 => {
                    self.direction_index =
                        ((self.direction_index + self.direction_points.len() as u32) - 1)
                            % self.direction_points.len() as u32;
                }
                180 => {
                    self.direction_index =
                        ((self.direction_index + self.direction_points.len() as u32) - 2)
                            % self.direction_points.len() as u32;
                }
                270 => {
                    self.direction_index =
                        ((self.direction_index + self.direction_points.len() as u32) - 3)
                            % self.direction_points.len() as u32;
                }
                _ => {
                    panic!("Wrong value for Right {}", instruction.value)
                }
            },
            NavigationAction::Forward => {
                match self.direction_points[self.direction_index as usize] {
                    CompassPoint::North => {
                        *north_pos += instruction.value;
                    }
                    CompassPoint::South => {
                        *south_pos += instruction.value;
                    }
                    CompassPoint::East => {
                        *east_pos += instruction.value;
                    }
                    CompassPoint::West => {
                        *west_pos += instruction.value;
                    }
                }
            }
        }
    }

    fn move_relative_waypoint(&mut self, instruction: NavigationInstruction) {
        let (ref mut north_pos, ref mut east_pos, ref mut south_pos, ref mut west_pos) =
            self.position;
        match instruction.action {
            NavigationAction::North => {
                let north_waypoint = self.waypoint.iter_mut().nth(0).unwrap();
                *north_waypoint += instruction.value;
            }
            NavigationAction::South => {
                let south_waypoint = self.waypoint.iter_mut().nth(2).unwrap();
                *south_waypoint += instruction.value;
            }
            NavigationAction::East => {
                let east_waypoint = self.waypoint.iter_mut().nth(1).unwrap();
                *east_waypoint += instruction.value;
            }
            NavigationAction::West => {
                let west_waypoint = self.waypoint.iter_mut().nth(3).unwrap();
                *west_waypoint += instruction.value;
            }
            NavigationAction::Right => match instruction.value {
                90 => {
                    let temp = self.waypoint.pop_back().unwrap();
                    self.waypoint.push_front(temp);
                }
                180 => {
                    for _ in 0..2 {
                        let temp = self.waypoint.pop_back().unwrap();
                        self.waypoint.push_front(temp);
                    }
                }
                270 => {
                    for _ in 0..3 {
                        let temp = self.waypoint.pop_back().unwrap();
                        self.waypoint.push_front(temp);
                    }
                }
                _ => {
                    panic!("Wrong value for Right {}", instruction.value)
                }
            },
            NavigationAction::Left => match instruction.value {
                90 => {
                    let temp = self.waypoint.pop_front().unwrap();
                    self.waypoint.push_back(temp);
                }
                180 => {
                    for _ in 0..2 {
                        let temp = self.waypoint.pop_front().unwrap();
                        self.waypoint.push_back(temp);
                    }
                }
                270 => {
                    for _ in 0..3 {
                        let temp = self.waypoint.pop_front().unwrap();
                        self.waypoint.push_back(temp);
                    }
                }
                _ => {
                    panic!("Wrong value for Right {}", instruction.value)
                }
            },
            NavigationAction::Forward => {
                for (i, value) in self.waypoint.iter().enumerate() {
                    match i {
                        0 => *north_pos += value * instruction.value,
                        1 => *east_pos += value * instruction.value,
                        2 => *south_pos += value * instruction.value,
                        3 => *west_pos += value * instruction.value,
                        _ => panic!("Waypoint cannot have more than 4 items {}", i),
                    }
                }
            }
        }
    }

    fn distance_from_start(&self) -> i32 {
        let (north_pos, east_pos, south_pos, west_pos) = self.position;
        ((north_pos - south_pos) as i32).abs() + (east_pos - west_pos).abs()
    }
}

fn parse_input(input: &str) -> Vec<NavigationInstruction> {
    input
        .lines()
        .filter_map(|line| line.parse::<NavigationInstruction>().ok())
        .collect()
}

fn part_one(input: &str) -> i32 {
    let navigation_instruction = parse_input(input);
    let mut ferry = Ferry::new();

    for instruction in navigation_instruction {
        ferry.instruct_move(instruction);
    }
    ferry.distance_from_start()
}

fn part_two(input: &str) -> i32 {
    let navigation_instruction = parse_input(input);
    let mut ferry = Ferry::new();

    for instruction in navigation_instruction {
        ferry.move_relative_waypoint(instruction);
    }
    ferry.distance_from_start()
}
