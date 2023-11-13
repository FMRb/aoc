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
    // let p1 = part_one(&input);
    // println!("Result part 1: {}", p1);
    let p2 = part_two(&input);
    println!("Result part 2: {}", p2);
    Ok(())
}

fn part_one(input: &str) -> u32 {
    let mut lines = input.lines();
    let timestamp_depart = lines.next().unwrap().parse::<u32>().unwrap();
    let bus_ids: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let mut timestamp = timestamp_depart;
    loop {
        for bus_id in &bus_ids {
            if timestamp % bus_id == 0 {
                return (timestamp - timestamp_depart) * bus_id;
            }
        }
        timestamp += 1;
    }
}

/*

Example: 7,13,x,x,59,x,31,19

N |
    N % 7 == 0
    N + 1 % 13 == 0
    Make sure keep Periodic between 7 and 13
    7 * 13 = 91 therefore
    N + 91 % 7 == 0
    N + 1 + 91 % 7 == 0

N |
    N % 7 == 0
    N % 13 == -1

    N' = local solution for numbers 7 and 13
    N % 91 = -N'



    N + 3 % 59 == 0
*/
fn part_two(input: &str) -> usize {
    input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, b)| {
            if let Ok(bus) = b.parse::<usize>() {
                Some((i, bus))
            } else {
                None
            }
        })
        .fold((0, 1), |(mut t, step_by), (n, bus)| {
            while (t + n) % bus != 0 {
                t += step_by;
            }
            (t, step_by * bus)
        })
        .0
}
