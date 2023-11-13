use std::env;
use std::fs;

// BFFFBBFRRR: row 70, column 7, seat ID 567
// FFFBBBFRRR: row 14, column 7, seat ID 119
// BBFFBBFRLL: row 102, column 4, seat ID 820

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
struct BoardingPass {
    total_rows: u32,
    total_columns: u32,
    row_number: u32,
    column_number: u32,
}

impl BoardingPass {
    fn new() -> Self{
        Self {
            total_rows: 128,
            total_columns: 8,
            row_number: 0,
            column_number: 0,
        }
    }

    fn calculate_row(&mut self, row_code: &str) {
        let mut range = [0, self.total_rows];

        let mut row_number = 0;
        row_code
            .chars()
            .for_each(|code_char| {
                let remaining = (range[1]-range[0]) / 2;
                match code_char {
                    'F' => {
                        if remaining == 1 {
                            row_number = range[0];
                        }
                        range[1] = range[1] - remaining;
                    }
                    'B' => {
                        if remaining == 1 {
                            row_number = range[1] - 1;
                        }
                        range[0] = range[0] + remaining;
                    }
                    _ => {
                        println!("Error: calculate row wrong match char")
                    }
                }
            });
        self.row_number = row_number;
    }

    fn calculate_column(&mut self, column_code: &str) {
        let mut range = [0, self.total_columns];
        let mut column_number = 0;
        column_code
            .chars()
            .for_each(|code_char| {
                let remaining = (range[1]-range[0]) / 2;
                match code_char {
                    'L' => {
                        if remaining == 1 {
                            column_number = range[0];
                        }
                        range[1] = range[1] - remaining;
                    }
                    'R' => {
                        if remaining == 1 {
                            column_number = range[1] - 1;
                        }
                        range[0] = range[0] + remaining;
                    }
                    _ => {
                        println!("Error: calculate column wrong match char")
                    }
                }
            });
        self.column_number = column_number;
    }

    fn calculate_seat_id(&self) -> u32 {
        self.row_number*8 + self.column_number
    }
}

fn parse_input(input: &str) -> Vec<BoardingPass>{
    let row_column_codes: Vec<(&str,&str)>= input
        .lines()
        .map(|seat_id| seat_id.split_at(7))
        .collect();

    let mut boarding_passes = Vec::new();
    for (row_code, column_code) in row_column_codes {
        let mut boarding_pass = BoardingPass::new();
        boarding_pass.calculate_row(row_code);
        boarding_pass.calculate_column(column_code);
        boarding_passes.push(boarding_pass);
    }

    boarding_passes

}

fn part_one(input: &str) -> u32 {
    let boarding_passes = parse_input(input);
    return boarding_passes
        .iter()
        .map(|boarding_pass| boarding_pass.calculate_seat_id())
        .max()
        .unwrap();
}

fn part_two(input: &str) -> u32 {
    let boarding_passes = parse_input(input);
    let seat_ids: Vec<u32> = boarding_passes
        .iter()
        .map(|boarding_pass| boarding_pass.calculate_seat_id())
        .collect();

    let higher_seat_id = seat_ids
        .iter()
        .max()
        .unwrap();

    let mut missing_seat = 0;
    for i in 0..*higher_seat_id-1 {
        if i > 0 && seat_ids.contains(&(i-1)) && !seat_ids.contains(&i) && seat_ids.contains(&(i+1)) {
            missing_seat = i;
            break;
        }
    }
    missing_seat
}