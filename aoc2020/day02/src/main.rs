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

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc

fn part_one(input: &str) -> io::Result<u32> {
    let mut correct_pass = 0;
    for line in input.lines() {
        let policy_pass = line
            .split_whitespace()
            .collect::<Vec<&str>>();

        let policy_boundaries = policy_pass[0]
            .split('-')
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let min = policy_boundaries[0];
        let max = policy_boundaries[1];

        let ref_letter = policy_pass[1]
            .chars()
            .next()
            .unwrap();

        let count_letters = policy_pass[2]
            .chars()
            .fold(0, |acc, ch| {
                if ch == ref_letter {
                    return acc + 1;
                }
                acc
            });

        if min <= count_letters && count_letters <= max {
            correct_pass += 1;
        }
    }
    Ok(correct_pass)
}

fn part_two(input: &str) -> io::Result<u32> {
    let mut correct_pass = 0;
    for line in input.lines() {
        let policy_pass = line
            .split_whitespace()
            .collect::<Vec<&str>>();

        let policy = policy_pass[0]
            .split('-')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        let first_position = policy[0];
        let second_position = policy[1];

        let ref_letter = policy_pass[1]
            .chars()
            .next()
            .unwrap();

        let password = policy_pass[2];

        let first_char = password.chars().nth(first_position - 1).unwrap();
        let second_char = password.chars().nth(second_position - 1).unwrap();
        if (ref_letter == first_char || ref_letter == second_char) && first_char != second_char {
            correct_pass += 1;
        }
    }
    Ok(correct_pass)
}