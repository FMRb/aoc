use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, self, newline, digit1, multispace1},
    multi::{separated_list1, many1},
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Instruction {
  from: u32, 
  to: u32,
  quantity: u32,
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, cell) = alt((
      tag("   "),
      delimited(
        complete::char('['),
        alpha1,
        complete::char(']')
      )
    ))(input)?;
    
    let result = match cell {
      "   " => None,
      value => Some(value)
    };

    Ok((input, result))
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Option<&str>>> {
  let (input, boxes) = separated_list1(tag(" "), parse_crate)(input)?;
  Ok((input, boxes))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
  // move 1 from 2 to 1
  let (input, _) = tag("move ")(input)?;
  let (input, quantity) = digit1(input)?;
  let (input, _) = tag(" from ")(input)?;
  let (input, from) = digit1(input)?;
  let (input, _) = tag(" to ")(input)?;
  let (input, to) = digit1(input)?;

  let quantity = u32::from_str_radix(quantity, 10).unwrap();
  let from = u32::from_str_radix(from, 10).unwrap();
  let to = u32::from_str_radix(to, 10).unwrap();

  Ok((input, Instruction{ from, to, quantity }))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
  let (input, instructions) = separated_list1(newline, parse_instruction)(input)?;
  Ok((input, instructions))
}

fn stacks(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Instruction>)> {
  let (input, crate_rows) = separated_list1(newline, parse_crates)(input)?;
  let (input, _) = newline(input)?;
  let (input, number) = many1(preceded(multispace1, digit1))(input)?;
  let (input, _) = multispace1(input)?;

  let (input, instructions) = parse_instructions(input)?;
  
  let mut stacks: Vec<Vec<&str>> = Vec::with_capacity(number.len());
  for _ in 0..number.len() {
    stacks.push(Vec::new());
  }

  for row in crate_rows.iter().rev() {
    assert_eq!(number.len(), row.len());
    for j in 0..row.len() {
      if let Some(crate_id) = row[j] {
        stacks[j].push(crate_id);
      }
    }
  }


  Ok((input, (stacks, instructions)))
}

fn exercise_one(instructions: &Vec<Instruction>, stacks: &mut Vec<Vec<& str>>) { 
  for instruction in instructions {
    match instruction {
      Instruction{ from, to, quantity } => {
        let from_len = stacks[(from - 1) as usize].len();
        let move_crate: Vec<&str> = stacks[(from - 1) as usize]
          .drain(from_len - (*quantity as usize)..)
          .collect();
        
        for crate_id in move_crate.iter().rev() {
          stacks[(to - 1) as usize].push(crate_id);
        }
      }
    }
  }
}

fn exercise_two(instructions: &Vec<Instruction>, stacks: &mut Vec<Vec<&str>>) {
  for instruction in instructions {
    match instruction {
      Instruction{ from, to, quantity } => {
        let from_len = stacks[(from - 1) as usize].len();
        let move_crate: Vec<&str> = stacks[(from - 1) as usize]
          .drain(from_len - (*quantity as usize)..)
          .collect();
        
        for crate_id in move_crate.iter() {
          stacks[(to - 1) as usize].push(crate_id);
        }
      }
    }
  }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/5.input")?;

    let (_, (stacks, instructions)) = stacks(&input).unwrap();

    let mut stacks_one = stacks.clone();
    let mut stacks_two = stacks.clone();
    exercise_one(&instructions, &mut stacks_one);
    exercise_two(&instructions, &mut stacks_two);
    
    for stack in stacks_one.iter() {
      print!("{}", stack.last().unwrap());
    }
    println!();
    for stack in stacks_two.iter() {
      print!("{}", stack.last().unwrap());
    }
    println!();
    Ok(())
}
