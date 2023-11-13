use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

fn calculate_priority(item: char) -> u32 {
    if item.is_lowercase() {
        let base = ('a' as u32) - 1;

        return (item as u32) - base;
    } else {
        let base: u32 = 'A' as u32;
        return (item as u32) - base + 27;
    }
}

fn exercise_one() -> Result<Vec<HashSet<char>>> {
    Ok(aoc::read_one_per_line::<String>("./data/3.input")?
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(pocket_a, pocket_b)| {
            let mut common: HashSet<char> = HashSet::new();
            for c in pocket_a.chars() {
                if pocket_b.contains(c) {
                    common.insert(c);
                }
            }
            common
        })
        .collect())
}

fn get_common_items(list_a: &str, list_b: &str, list_c: &str) -> Option<char> {
  let mut common = HashSet::new();
  for l in list_a.chars() {
    if list_b.contains(l) {
      common.insert(l);
    }
  }
  let mut result: Option<char> = None;
  for c in common.iter(){
    if list_c.contains(*c) {
      result = Some(*c);
    }
  }
  result
}

fn exercise_two() -> Result<Vec<Option<char>>> {
    let groups = aoc::read_one_per_line::<String>("./data/3.input")?;

    let mut common_items = Vec::new(); 
    for group in  groups.chunks_exact(3) {
      let list_a = group.get(0).unwrap();
      let list_b = group.get(1).unwrap();
      let list_c= group.get(2).unwrap();
      common_items.push(get_common_items(list_a, list_b, list_c));
    }
    Ok(common_items)
}

fn main() -> Result<()> {
    if let Ok(common_items) = exercise_one() {
        let mut score = 0;
        for items in common_items {
            for item in items {
                score += calculate_priority(item);
            }
        }
        println!("Part 1: {:?}", score);
    }

    if let Ok(common_items) = exercise_two() {
      let mut score = 0;
      for item in common_items {
        if let Some(item) = item {
          score += calculate_priority(item);
        }
      }
      println!("Part 2: {:?}", score);
    }
    Ok(())
}
