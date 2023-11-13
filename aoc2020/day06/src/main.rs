use std::env;
use std::fs;
use std::collections::HashSet;

// abc

// a
// b
// c

// ab
// ac

// a
// a
// a
// a

// b
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
    let r: usize = input
        .split("\n\n")
        .map(|group| {
            return group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(HashSet::new(),|answers_a, answers_b|
                    answers_a.union(&answers_b).copied().collect()
                ).len();
        })
        .sum();
    r as u32
}

fn part_two(input: &str) -> u32 {
    let r: usize = input
        .split("\n\n")
        .map(|group| {
            let sets = group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>());

            let mut common_answers = sets.clone().next().unwrap();
            for answers in sets.skip(1) {
                common_answers = common_answers.intersection(&answers).copied().collect();
            }
            return common_answers.len();
        })
        .sum();
    r as u32
}