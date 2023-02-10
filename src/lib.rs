use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

pub fn one_at_a_time<T>(path: &str) -> Result<HashSet<T>>
where
    T: FromStr + std::hash::Hash + std::cmp::Eq,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
