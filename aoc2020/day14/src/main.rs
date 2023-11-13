use regex::Regex;
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

// Big mask using little endian

#[derive(Debug, PartialEq, Eq)]
enum Bit {
    One,
    Zero,
    X,
}

impl Default for Bit {
    fn default() -> Self {
        Bit::X
    }
}

fn parse_bit(input: &str) -> Vec<Bit> {
    input
        .chars()
        .map(|c| match c {
            '1' => Bit::One,
            '0' => Bit::Zero,
            'X' => Bit::X,
            _ => unreachable!(),
        })
        .collect()
}
// Mask

#[derive(Debug, Default, PartialEq, Eq)]
struct BitMask {
    unchanged: i64,
    overwrites: i64,
}

impl BitMask {
    fn parse(bits: Vec<Bit>) -> BitMask {
        let mut bit_mask = BitMask::default();
        for (i, b) in bits.iter().enumerate() {
            // Example
            //             XXXXXX XXXXXX XXXXXX XXXXXX XXXXXX 011XX0
            // unchanged:  111111 111111 111111 111111 111111 000110
            // overwrites: 000000 000000 000000 000000 000000 011000
            let bit_pos = 1 << (35 - i);
            match *b {
                Bit::One => bit_mask.overwrites |= bit_pos,
                Bit::X => bit_mask.unchanged |= bit_pos,
                Bit::Zero => {}
            }
        }
        bit_mask
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Write {
    value: i64,
    address: i64,
}

impl Write {
    fn parse(input_index: &str, input_value: &str) -> Write {
        let mem_re = Regex::new(r"^mem\[(\d+)\]").unwrap();
        let mem_address = mem_re
            .captures(input_index)
            .unwrap()
            .get(1)
            .map_or("", |m| m.as_str())
            .parse::<i64>()
            .unwrap();
        let value = input_value.parse::<i64>().unwrap();

        Write {
            address: mem_address,
            value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ProgramParameter {
    Mask(BitMask),
    Write(Write),
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Program {
    mask: BitMask,
    memory: HashMap<i64, i64>,
}

impl Program {
    fn set_mask(&mut self, mask: BitMask) {
        self.mask = mask;
    }

    fn write(&mut self, param: Write) {
        let masked_value = (param.value & self.mask.unchanged) | self.mask.overwrites;
        self.memory.insert(param.address, masked_value);
    }

    fn run(&mut self, program_parameter: ProgramParameter) {
        match program_parameter {
            ProgramParameter::Mask(bit_mask) => self.set_mask(bit_mask),
            ProgramParameter::Write(value) => self.write(value),
        }
    }
}

fn part_one(input: &str) -> i64 {
    let mut program = Program::default();
    for line in input.lines() {
        // Expected always mask first
        let entry = line.split_whitespace().collect::<Vec<&str>>();
        let parameter = if entry[0] == "mask" {
            let bits = parse_bit(entry[2]);

            ProgramParameter::Mask(BitMask::parse(bits))
        } else {
            // ["mem[<mem_address>]","=","<value>"]
            ProgramParameter::Write(Write::parse(entry[0], entry[2]))
        };

        program.run(parameter);
    }

    program.memory.values().sum()
}

/*
    Example:

    address: 000000 000000 000000 000000 000000 101010  (decimal 42)
    mask:    000000 000000 000000 000000 000000 X1001X
    result:  000000 000000 000000 000000 000000 X1101X



    input mask: 000000 000000 000000 000000 000000 X1001X
    floatings: [
                000000 000000 000000 000000 000000 000000,
                000000 000000 000000 000000 000000 000001
                000000 000000 000000 000000 000000 100000
                000000 000000 000000 000000 000000 100001
               ]
    overwrites: 000000 000000 000000 000000 000000 010010
    zeros:      111111 111111 111111 111111 111111 001100

 zeros & input: 000000 000000 000000 000000 000000 001000

    Apply:      000000 000000 000000 000000 000000 101010  (decimal 42)

    Results:    000000 000000 000000 000000 000000 011010  (decimal 26)
                000000 000000 000000 000000 000000 011011  (decimal 27)
                000000 000000 000000 000000 000000 111010  (decimal 58)
                000000 000000 000000 000000 000000 111011  (decimal 59)

*/

#[derive(Debug, Default, PartialEq, Eq)]
struct FloatingMask {
    floatings: Vec<i64>,
    ones: i64,
    zeroes: i64,
}

impl FloatingMask {
    fn parse(bits: Vec<Bit>) -> FloatingMask {
        let mut bit_mask = FloatingMask::default();
        bit_mask.floatings.push(0);
        for (i, b) in bits.iter().enumerate() {
            let bit_pos = 1 << (35 - i);
            match *b {
                Bit::One => bit_mask.ones |= bit_pos,
                Bit::X => {
                    let mut new_floatings =
                        bit_mask
                            .floatings
                            .iter()
                            .fold(Vec::new(), |mut acc, floating| {
                                let one_option = floating | bit_pos;
                                acc.push(one_option);
                                acc
                            });
                    bit_mask.floatings.append(&mut new_floatings)
                }
                Bit::Zero => bit_mask.zeroes |= bit_pos,
            }
        }
        bit_mask
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Program2Parameter {
    Mask(FloatingMask),
    Write(Write),
}
#[derive(Debug, Default, PartialEq, Eq)]
struct Program2 {
    mask: FloatingMask,
    memory: HashMap<i64, i64>,
}

impl Program2 {
    fn set_mask(&mut self, mask: FloatingMask) {
        self.mask = mask;
    }

    fn write(&mut self, param: Write) {
        for floating in &self.mask.floatings {
            let address = ((param.address & self.mask.zeroes) | self.mask.ones) | floating;
            self.memory.insert(address, param.value);
        }
    }

    fn run(&mut self, program_parameter: Program2Parameter) {
        match program_parameter {
            Program2Parameter::Mask(mask) => self.set_mask(mask),
            Program2Parameter::Write(value) => self.write(value),
        }
    }
}

fn part_two(input: &str) -> i64 {
    let mut program = Program2::default();
    for line in input.lines() {
        // Expected always mask first
        let entry = line.split_whitespace().collect::<Vec<&str>>();
        let parameter = if entry[0] == "mask" {
            let bits = parse_bit(entry[2]);

            Program2Parameter::Mask(FloatingMask::parse(bits))
        } else {
            // ["mem[<mem_address>]","=","<value>"]
            Program2Parameter::Write(Write::parse(entry[0], entry[2]))
        };

        program.run(parameter);
    }

    program.memory.values().sum()
}
