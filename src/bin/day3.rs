use anyhow::Result;

fn tree_counter(path: &str, travses: Vec<(usize, usize)>) -> Result<usize> {
    let input = std::fs::read_to_string(path)?
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let width = input[0].len();

    let mut x = 0;
    let mut tree_count = 0;
    let mut tree_collection = vec![];

    for (right, down) in travses.iter() {
        for i in 1..input.len() {
            if i > 0 && i % down != 0 {
                continue;
            }
            x += right;
            if x > width - 1 {
                x = x - width;
            }
            if input[i][x] == '#' {
                tree_count += 1;
            }
        }
        tree_collection.push(tree_count);
        tree_count = 0;
        x = 0;
    }

    Ok(tree_collection.iter().fold(1, |acc, v| acc * v))
}

fn part_one(path: &str) -> Result<usize> {
    tree_counter(path, vec![(3, 1)])
}

fn part_two(path: &str) -> Result<usize> {
    tree_counter(path, vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)])
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one("./data/3.input")?);
    println!("Part Two: {}", part_two("./data/3.input")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let res = part_one("./data/3.test").unwrap();
        assert_eq!(res, 7);
    }

    #[test]
    fn part_two_test() {
        let res = part_two("./data/3.test").unwrap();
        assert_eq!(res, 336);
    }
}
