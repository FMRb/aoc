use anyhow::Result;
use itertools::Itertools;

fn descending_calculator(size: usize) -> Result<usize> {
    Ok(aoc::read_one_per_line::<u32>("./data/1.input")?
        .windows(size)
        .filter(|w| w[0] < w[size - 1])
        .collect_vec()
        .len()
    )
}

fn main() -> Result<()> {
    println!("Part 1: {}", descending_calculator(2)?);

    Ok(())
}