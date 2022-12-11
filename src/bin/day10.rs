use anyhow::Result;
use nom::{
    IResult,
    multi::separated_list1,
    character::{complete::{newline, space1, self}},
    bytes::complete::tag,
    branch::alt
};


enum Instruction {
    Add(i32),
    Noop,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, instruction) = alt((tag("noop"), tag("addx")))(input)?;

   match instruction {
        "noop" => Ok((input, Instruction::Noop)),
        "addx" => {
            let (input, _) = space1(input)?;
            let (input, value) = complete::i32(input)?;
            Ok((input, Instruction::Add(value)))
        },
        _ => panic!("Error parsing instructions"),
    }
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    Ok(separated_list1(newline, parse_instruction)(input).unwrap())
}

fn calculate_signal_strength(cycle: u32, reg_x: i32) -> i32 {
    cycle as i32 * reg_x
}
fn benchmark_cycle(cycle: u32, reg_x: i32) -> Option<i32> {
    match cycle {
        20 => Some(calculate_signal_strength(cycle, reg_x)),
        60 => Some(calculate_signal_strength(cycle, reg_x)),
        100 => Some(calculate_signal_strength(cycle, reg_x)),
        140 => Some(calculate_signal_strength(cycle, reg_x)),
        180 => Some(calculate_signal_strength(cycle, reg_x)),
        220 => Some(calculate_signal_strength(cycle, reg_x)),
        _ => None
    }
}

fn exercise_one(instructions: &Vec<Instruction>) -> i32 {
    let mut reg_x = 1;
    let mut cycle = 0; 
    let mut signals = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                cycle += 1;
                if let Some(result) = benchmark_cycle(cycle, reg_x) {
                    println!("Cycle {cycle} - signal strength {result}");
                    signals += result;
                }
            },
            Instruction::Add(value) => {
                cycle += 1;
                // CHECK CYCLE
                if let Some(result) = benchmark_cycle(cycle, reg_x) {
                    println!("Cycle {cycle} - signal strength {result}");
                    signals += result;
                }
                cycle += 1;
                if let Some(result) = benchmark_cycle(cycle, reg_x) {
                    println!("Cycle {cycle} - signal strength {result}");
                    signals += result;
                }
                reg_x += value;
            },
        }
    }
    signals
}

fn draw_pixel(cycle: u32, sprite_pos: i32) {
    match cycle {
        40 => println!(),
        80 => println!(),
        120 => println!(),
        160 => println!(),
        200 => println!(),
        240 => println!(),
        _ => ()
    };
    let cycle = cycle % 40;

    if cycle as i32 >= sprite_pos && cycle as i32 <= (sprite_pos + 2) {
        print!("#");
    } else {
        print!(".");
    }

}

fn exercise_two(instructions: &Vec<Instruction>) {

    let mut sprite_pos = 0;
    let mut reg_x = 1;
    let mut cycle = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Add(value) => {
                draw_pixel(cycle, sprite_pos);
                cycle += 1;
                draw_pixel(cycle, sprite_pos);
                cycle +=1;
                reg_x += value; 
                sprite_pos = reg_x - 1;
            },
            Instruction::Noop => {
                draw_pixel(cycle, sprite_pos);
                cycle +=1;
            },
        }
    }
    println!();

}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/10.input")?;
    let (_, instructions) = parse_instructions(&input).unwrap();

    let signals = exercise_one(&instructions);
    println!("Part 1: {signals}");
    exercise_two(&instructions);
    // FECZELHE
    Ok(())
}