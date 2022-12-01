use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

pub fn read_one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn read_group_lines<T>(path: &str) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split("\r\n\r\n")
        .filter_map(|group| {
            Some(group
                .lines()
                .filter_map(|line| line.parse::<T>().ok())
                .collect_vec())
        })
        .collect_vec()
    )
}