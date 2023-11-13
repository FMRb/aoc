use anyhow::Result;
use itertools::Itertools;

fn calories_counter(num_elves: usize) -> Result<u32> {
    Ok(aoc::read_group_lines::<u32>("./data/1.input")?
        .iter()
        .map(|group| group.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(num_elves)
        .sum())
}

fn main() -> Result<()> {
    println!("Part 1: {}", calories_counter(1)?);
    println!("Part 2: {}", calories_counter(3)?);

    Ok(())
}