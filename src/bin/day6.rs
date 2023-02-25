use anyhow::Result;
use std::collections::HashSet;

fn part_one(path: &str) -> Result<usize> {
    Ok(std::fs::read_to_string(path)?
        .split("\n\n")
        .map(|answers| {
            let mut set = HashSet::new();
            let _ = answers
                .lines()
                .map(|line| {
                    let _ = line.chars().map(|c| set.insert(c)).collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
            set.len()
        })
        .sum())
}

fn part_two(path: &str) -> Result<usize> {
    Ok(std::fs::read_to_string(path)?
        .split("\n\n")
        .map(|answers| {})
        .sum())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one("./data/6.input")?);
    println!("Part Two: {}", part_two("./data/6.input")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let res = part_one("./data/6.test").unwrap();
        assert_eq!(res, 11);
    }

    #[test]
    fn part_two_test() {
        let res = part_two("./data/6.test").unwrap();
        assert_eq!(res, 6);
    }
}
