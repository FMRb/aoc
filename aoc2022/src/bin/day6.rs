use std::slice::Windows;

use anyhow::Result;

fn is_unique(window: &(usize, &[char])) -> bool {
  let (_, list) = window;

  for i in 0..list.len() {
    let item = list[i];
    for j in 0..list.len() {
      if j == i {
        continue;
      }
      if item == list[j] {
        return false;
      }
    }
  }
  return true;
}

fn calculate_marker(code: &Vec<Vec<char>>, window: usize) {

  for line in code {
    let mut m = line.windows(window).enumerate();
    let result = m.find(is_unique);

    if let Some((i, _)) = result {
      println!("Index: {}", i + window);
    }
  }
}
fn main() -> Result<()> {
  let lines: Vec<Vec<char>> = std::fs::read_to_string("./data/6.input")?
    .lines()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect();
  
  calculate_marker(&lines, 4);
  calculate_marker(&lines, 14);
  

  Ok(())
} 