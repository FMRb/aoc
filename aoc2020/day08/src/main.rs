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
//// Example instructions
// nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6

const JUMP: &str = "jmp";
const ACC: &str = "acc";
const NOP: &str = "nop";

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Jmp,
    Acc,
    Nop,
}

type Instruction = (Operation, i32, bool);

// #[derive(Debug, PartialEq, Eq, Clone)]
struct Program {
    accumulator: i32,
    index_instructions: i32,
}

impl Program {
    fn new() -> Self {
        Self {
            accumulator: 0,
            index_instructions: 0,
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.index_instructions = 0;
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions:Vec<Instruction> = Vec::new();

    for line in input.lines() {
        let str_instructions:Vec<&str> = line.split_whitespace().collect();
        let operation = match str_instructions[0] {
            JUMP => {
                Operation::Jmp
            }
            ACC => {
                Operation::Acc
            }
            NOP => {
                Operation::Nop
            }
            _ => {
                panic!("Error: wrong operation");
            }
        };
        let argument = str_instructions[1].parse::<i32>().unwrap_or(0);

        let instruction:Instruction = (operation, argument, false);
        instructions.push(instruction);
    }
    instructions
}

fn execute(program: &mut Program, instructions: &mut Vec<Instruction>) -> Option<i32> {
    loop {
        if program.index_instructions > (instructions.len() - 1) as i32 {
            return Some(program.accumulator);
        }

        // Only for reading visited, not mutable
        let (operation, argument, visited) = instructions
            .get_mut(program.index_instructions as usize)
            .unwrap();

        if *visited == true {
            return None;
        }


        match *operation {
            Operation::Jmp => {
                program.index_instructions += *argument;
            }
            Operation::Acc => {
                program.accumulator += *argument;
                program.index_instructions += 1;
            }
            Operation::Nop => {
                program.index_instructions += 1;
            }
        }

        *visited = true;
    }
}
fn part_one(input: &str) -> i32 {
    let mut instructions = parse_instructions(input);

    let mut program = Program::new();
    match execute(&mut program, &mut instructions) {
        Some(acc) => { acc }
        None => { program.accumulator }
    }
}

fn part_two(input: &str) -> i32 {
    let instructions = parse_instructions(input);
    let mut program = Program::new();

    for instruction_id in 0..instructions.len() {
        let mut new_instructions = instructions.clone();

        let mut instruction = new_instructions.get_mut(instruction_id).unwrap();
        match instruction.0 {
            Operation::Jmp => {
                instruction.0 = Operation::Nop;
            }
            Operation::Nop => {
                instruction.0 = Operation::Jmp;
            }
            _ => {}
        }

        match execute(&mut program, &mut new_instructions) {
           Some(result) => {
               return result;
           }
           None => {
               program.reset();
               continue;
           }
       }
    }
    0
}