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

fn part_one(input: &str) -> io::Result<u32> {
    let entries: Vec<u32> = input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .filter_map(|entry| if entry < 2020 { Some(entry) } else { None })
        .collect();

    for i in 0..entries.len()-1 {
        for j in (i+1)..entries.len() {
            if entries[i] + entries[j] == 2020 {
                println!("Entry A: {} Entry B:{}", entries[i], entries[j]);
                return Ok(entries[i] * entries[j]);
            }
        }
    }
    Ok(0)
}

fn part_two(input: &str) -> io::Result<u32> {
    let entries: Vec<u32> = input
        .lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .filter_map(|entry| if entry < 2020 { Some(entry) } else { None })
        .collect();

    for i in 0..entries.len()-2 {
        for j in (i+1)..entries.len()-1 {
            for z in (j+1)..entries.len() {
                if entries[i] + entries[j] + entries[z] == 2020 {
                    println!("Entry A: {} Entry B:{} Entry C: {}", entries[i], entries[j], entries[z]);
                    return Ok(entries[i] * entries[j] * entries[z]);
                }
            }
        }
    }
    Ok(0)
}
