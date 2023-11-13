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

// 0,3,6
fn part_one(input: &str) -> i32 {
    let starting_numbers: Vec<i32> = input
        .split(',')
        .filter_map(|i| i.parse::<i32>().ok())
        .collect();

    let mut turns_map = HashMap::new();
    // Skip the last item of the list
    for i in 0..starting_numbers.len() - 1 {
        let &number = starting_numbers.get(i).unwrap();
        turns_map.insert(number, (i + 1) as i32);
    }

    let turn_goal = 2020;
    let mut turn = starting_numbers.len() as i32;
    let mut spoken_number = *starting_numbers.last().unwrap();

    while turn < turn_goal {
        let speak = match turns_map.get(&spoken_number) {
            Some(last_turn) => turn - last_turn,
            None => 0,
        };
        turns_map.insert(spoken_number, turn);
        turn += 1;
        spoken_number = speak;
    }
    spoken_number
}

fn part_two(input: &str) -> i64 {
    let starting_numbers: Vec<i64> = input
        .split(',')
        .filter_map(|i| i.parse::<i64>().ok())
        .collect();

    let mut turns_map = HashMap::new();
    // Skip the last item of the list
    for i in 0..starting_numbers.len() - 1 {
        let &number = starting_numbers.get(i).unwrap();
        turns_map.insert(number as i64, (i + 1) as i64);
    }

    let turn_goal = 30000000;
    let mut turn = starting_numbers.len() as i64;
    let mut spoken_number = *starting_numbers.last().unwrap();

    while turn < turn_goal {
        let speak = match turns_map.get(&spoken_number) {
            Some(last_turn) => turn - last_turn,
            None => 0,
        };
        turns_map.insert(spoken_number, turn);
        turn += 1;
        spoken_number = speak;
    }
    spoken_number
}
