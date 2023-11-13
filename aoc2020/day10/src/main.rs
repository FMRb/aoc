use std::fs;
use std::{collections::HashMap, env};

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

fn part_one(input: &str) -> u32 {
    let mut adapters_joltage: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    adapters_joltage.sort();

    let max_jolts_difference = 3;
    let device_joltage = adapters_joltage.last().copied().unwrap() + max_jolts_difference;
    adapters_joltage.push(device_joltage);

    let mut charging_outlet_counter = 0;
    let mut one_differences = 0;
    let mut three_differences = 0;
    for adapter in adapters_joltage {
        match adapter - charging_outlet_counter {
            1 => {
                one_differences += 1;
            }
            2 => {}
            3 => {
                three_differences += 1;
            }
            _ => {
                println!(
                    "Charging outlet counter: {}, Adapter: {}",
                    charging_outlet_counter, adapter
                );
                panic!("Wrong difference in jolts")
            }
        }
        charging_outlet_counter = adapter;
    }

    one_differences * three_differences
}

fn calculate_combinations(
    cache: &mut HashMap<usize, u64>,
    joltages: &Vec<u32>,
    index: usize,
) -> u64 {
    let len = joltages.len() - 1;
    if index > len {
        return 0;
    }

    if index == len {
        return 1;
    }

    if let Some(&cache_value) = cache.get(&index) {
        return cache_value;
    }

    let mut counter = 0;
    let target = joltages[index];

    if let Some(&one_next) = joltages.get(index + 1) {
        if one_next - target <= 3 {
            counter += calculate_combinations(cache, joltages, index + 1);
        }
    }
    if let Some(&two_next) = joltages.get(index + 2) {
        if two_next - target <= 3 {
            counter += calculate_combinations(cache, joltages, index + 2);
        }
    }
    if let Some(&three_next) = joltages.get(index + 3) {
        if three_next - target <= 3 {
            counter += calculate_combinations(cache, joltages, index + 3);
        }
    }
    cache.insert(index, counter);
    counter
}

fn part_two(input: &str) -> u64 {
    let mut adapters_joltage: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let device_joltage = adapters_joltage.iter().copied().max().unwrap() + 3;
    adapters_joltage.push(0);
    adapters_joltage.push(device_joltage);
    adapters_joltage.sort_unstable();

    let mut cache: HashMap<usize, u64> = HashMap::new();
    calculate_combinations(&mut cache, &adapters_joltage, 0)
}
