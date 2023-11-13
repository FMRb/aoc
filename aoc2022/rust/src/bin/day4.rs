use anyhow::Result;
use std::str::FromStr;


struct Assignments {
  pair_one: (u32, u32),
  pair_two: (u32, u32),
}

impl FromStr for Assignments {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
      let (pair_one, pair_two) = s.split_once(',').unwrap();
      let pair_one = pair_one
        .split_once('-')
        .map(|(a, b)| 
          (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        ).unwrap();
      let pair_two = pair_two
        .split_once('-')
        .map(|(a, b)| 
          (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        ).unwrap();
        
      Ok(Assignments{ pair_one, pair_two })
    }
} 

fn exercise_one(inputs: &Vec<Assignments>) -> usize {
  inputs
    .iter()
    .filter(|assignment| {
      let Assignments { pair_one, pair_two } = assignment;
      (pair_one.0 <= pair_two.0 && pair_one.1 >= pair_two.1) ||
      (pair_two.0 <= pair_one.0 && pair_two.1 >= pair_one.1)
    })
    .count()
}

fn exercise_two(inputs: &Vec<Assignments>) -> usize {
  inputs
    .iter()
    .filter(|assignment| {
      let Assignments { pair_one, pair_two } = assignment;
      (pair_one.0 <= pair_two.0 && pair_one.1 >= pair_two.1) ||
      (pair_two.0 <= pair_one.0 && pair_two.1 >= pair_one.1) ||
      (pair_one.0 <= pair_two.0 && pair_one.1 >= pair_two.0) ||
      (pair_two.0 <= pair_one.0 && pair_two.1 >= pair_one.0)
    })
    .count()
}

fn main() -> Result<()> {
  let inputs: Vec<Assignments> = std::fs::read_to_string("./data/4.input")?
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

  let result_1 = exercise_one(&inputs);
  let result_2 = exercise_two(&inputs);

  println!("Part 1: {result_1}");
  println!("Part 2: {result_2}");
  
  Ok(())
}