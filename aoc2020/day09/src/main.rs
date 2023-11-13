use std::env;
use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn (std::error::Error)>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: <path_to_input>");
        std::process::exit(1);
    }
    println!("Argument {} preamble {}", args[1], args[2]);
    let path = String::from(&args[1]);
    let input = fs::read_to_string(&path)?;
    let preamble = args[2].parse::<u32>().unwrap_or(5);
    let p1 = part_one(&input, preamble);
    println!("Result part 1: {}", p1);
    let p2 = part_two(&input, preamble);
    println!("Result part 2: {}", p2);
    Ok(())
}


fn calculate_number(available_numbers: &Vec<u64>, num: u64) -> Option<u64> {
    let mut register: HashMap<u64, u64> = HashMap::with_capacity(available_numbers.len());

    for &available_number in available_numbers {
        if num > available_number {
            if register.contains_key(&available_number) {
                return Some(num);
            }
            let result = num - available_number;
            register.insert(result, result);
        }
    }
    None
}

fn decipher_xmax(input: &str, preamble: u32) -> u64 {
    let mut available_numbers: Vec<u64> = Vec::with_capacity(preamble as usize);

    for line in input.lines() {
        let num = line.parse::<u64>().unwrap_or(0);
        if available_numbers.len() < preamble as usize {
            available_numbers.push(num);
        } else {
            match calculate_number(&available_numbers, num) {
                Some(n) => {
                    available_numbers.remove(0);
                    available_numbers.push(n);
                }
                None => {
                    return num;
                }
            }
        }
    }
    0
}

fn part_one(input: &str, preamble: u32) -> u64 {
    decipher_xmax(input, preamble)
}

fn part_two(input: &str, preamble: u32) -> u64 {
    let buffer = input.lines().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let invalid_number = decipher_xmax(input, preamble);

    for i in 0..buffer.len() {
        let mut counter: Vec<u64> = Vec::new();
        for j in i..buffer.len() {
            let num = buffer[j];
            counter.push(num);
            let sum: u64 = counter.iter().sum();
            if sum > invalid_number {
                break;
            }
            if sum == invalid_number && counter.len() > 2 {
                let min = counter.iter().min().unwrap();
                let max = counter.iter().max().unwrap();
                return min + max;
            }
        }
    }
    0
}