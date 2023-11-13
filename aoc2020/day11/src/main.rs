use std::{env, result::Result};
use std::{fmt, fs};

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

#[derive(Debug)]
struct BadSeatInput;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

impl SeatStatus {
    fn parse_char(c: char) -> Result<SeatStatus, BadSeatInput> {
        match c {
            '.' => Ok(SeatStatus::Floor),
            'L' => Ok(SeatStatus::Empty),
            '#' => Ok(SeatStatus::Occupied),
            _ => Err(BadSeatInput),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
struct SeatLayout {
    seats: Vec<SeatStatus>,
    rows: u32,
    columns: u32,
}

impl SeatLayout {
    fn append_row(&mut self, row: &mut Vec<SeatStatus>) {
        self.seats.append(row);
    }

    fn add_number_rows(&mut self, rows: u32) {
        self.rows = rows;
    }

    fn add_number_columns(&mut self, columns: u32) {
        self.columns = columns;
    }

    fn set_seats(&mut self, seats: Vec<SeatStatus>) {
        self.seats = seats;
    }

    fn seat_occupied(&self) -> u32 {
        let mut count = 0;
        for &seat in self.seats.iter() {
            if seat == SeatStatus::Occupied {
                count += 1;
            }
        }
        count
    }
}

impl fmt::Display for SeatLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = "".to_string();
        for i in 0..self.seats.len() {
            if i != 0 && (i as u32 % self.columns) == 0 {
                output.push('\n');
            }
            match self.seats[i] {
                SeatStatus::Floor => {
                    output.push('.');
                }
                SeatStatus::Empty => {
                    output.push('L');
                }
                SeatStatus::Occupied => {
                    output.push('#');
                }
            }
        }
        write!(f, "{}", output)
    }
}

fn parse_seat_layout(input: &str) -> SeatLayout {
    let mut seat_layout = SeatLayout::default();
    let mut count_rows = 0;
    for line in input.lines() {
        let mut row: Vec<SeatStatus> = line
            .chars()
            .map(|c| SeatStatus::parse_char(c).unwrap())
            .collect();
        seat_layout.append_row(&mut row);
        count_rows += 1;
    }

    seat_layout.add_number_rows(count_rows);
    seat_layout.add_number_columns(input.lines().next().unwrap().chars().count() as u32);
    seat_layout
}

fn adjacent_occupied_seats(seat_layout: &SeatLayout, index: usize) -> u32 {
    let adjacents: [(i32, i32); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut adjacent_seats_occupied = 0;
    for &(column, row) in adjacents.iter() {
        // Side limits guards
        if index as u32 % seat_layout.columns == 0 && row < 0 {
            continue;
        }
        if index as u32 % seat_layout.columns == seat_layout.columns - 1 && row > 0 {
            continue;
        }
        let adjacent_index = index as i32 + row + (column * seat_layout.columns as i32);
        if let Some(&adjacent_seat) = seat_layout.seats.get(adjacent_index as usize) {
            if adjacent_seat == SeatStatus::Occupied {
                adjacent_seats_occupied += 1;
            }
        }
    }
    adjacent_seats_occupied
}

////// RULES:
// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.
// Floor (.) never changes; seats don't move, and nobody sits on the floor.
fn run_one(seat_layout: &mut SeatLayout) -> u32 {
    let mut occupied_seats = seat_layout.seat_occupied();
    // println!("Initial:");
    // println!("{}", seat_layout);
    // println!("-------------------------");
    loop {
        let mut updated_seats = seat_layout.seats.clone();
        for i in 0..seat_layout.seats.len() {
            let seat_status = seat_layout.seats[i];
            let adjacent_occupied = adjacent_occupied_seats(seat_layout, i);
            match seat_status {
                SeatStatus::Empty if adjacent_occupied == 0 => {
                    let seat = updated_seats.get_mut(i).unwrap();
                    *seat = SeatStatus::Occupied;
                }
                SeatStatus::Occupied if adjacent_occupied >= 4 => {
                    let seat = updated_seats.get_mut(i).unwrap();
                    *seat = SeatStatus::Empty;
                }
                _ => {}
            }
        }

        seat_layout.set_seats(updated_seats);
        let new_occupied_seats = seat_layout.seat_occupied();
        // println!("-------------");
        // println!("{}", seat_layout);
        // println!("Seats occupied: {}", new_occupied_seats);
        if new_occupied_seats == occupied_seats {
            break;
        }

        occupied_seats = new_occupied_seats;
    }
    occupied_seats
}

fn part_one(input: &str) -> u32 {
    let mut seat_layout = parse_seat_layout(input);
    run_one(&mut seat_layout)
}

fn see_seat_direction(seat_layout: &SeatLayout, index: usize, direction: (i32, i32)) -> u32 {
    let (row, column) = direction;
    let adjacent_index = index as i32 + row + (column * seat_layout.columns as i32);

    if index as u32 % seat_layout.columns == 0 && row < 0 {
        return 0;
    }
    if index as u32 % seat_layout.columns == seat_layout.columns - 1 && row > 0 {
        return 0;
    }

    if let Some(&seat) = seat_layout.seats.get(adjacent_index as usize) {
        if seat == SeatStatus::Occupied {
            return 1;
        }
        if seat == SeatStatus::Empty {
            return 0;
        }
        return see_seat_direction(seat_layout, adjacent_index as usize, direction);
    }
    return 0;
}

fn first_seats_occupied(seat_layout: &SeatLayout, index: usize) -> u32 {
    let directions: [(i32, i32); 8] = [
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut seats_occupied = 0;
    for &direction in directions.iter() {
        seats_occupied += see_seat_direction(seat_layout, index, direction);
    }
    seats_occupied
}

fn run_two(seat_layout: &mut SeatLayout) -> u32 {
    let mut occupied_seats = seat_layout.seat_occupied();
    // println!("Initial:");
    // println!("{}", seat_layout);
    // println!("-------------------------");
    loop {
        let mut updated_seats = seat_layout.seats.clone();
        for i in 0..seat_layout.seats.len() {
            let seat_status = seat_layout.seats[i];
            let adjacent_occupied = first_seats_occupied(seat_layout, i);
            match seat_status {
                SeatStatus::Empty if adjacent_occupied == 0 => {
                    let seat = updated_seats.get_mut(i).unwrap();
                    *seat = SeatStatus::Occupied;
                }
                SeatStatus::Occupied if adjacent_occupied >= 5 => {
                    let seat = updated_seats.get_mut(i).unwrap();
                    *seat = SeatStatus::Empty;
                }
                _ => {}
            }
        }

        seat_layout.set_seats(updated_seats);
        let new_occupied_seats = seat_layout.seat_occupied();
        // println!("-------------");
        // println!("{}", seat_layout);
        // println!("Seats occupied: {}", new_occupied_seats);
        if new_occupied_seats == occupied_seats {
            break;
        }

        occupied_seats = new_occupied_seats;
    }
    occupied_seats
}

fn part_two(input: &str) -> u32 {
    let mut seat_layout = parse_seat_layout(input);
    run_two(&mut seat_layout)
}
