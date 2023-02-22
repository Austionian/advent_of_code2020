use anyhow::Result;
use aoc2020::one_separated_by_empty_line;
use std::str::FromStr;

struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(|c| c == ' ' || c == '\n').collect::<Vec<_>>();

        let keys = values
            .iter()
            .map(|value| {
                value
                    .split_once(':')
                    .unwrap_or(("no semicolon found.", "error"))
                    .0
            })
            .collect::<Vec<_>>();

        let byr = keys.contains(&"byr");
        let iyr = keys.contains(&"iyr");
        let eyr = keys.contains(&"eyr");
        let hgt = keys.contains(&"hgt");
        let hcl = keys.contains(&"hcl");
        let ecl = keys.contains(&"ecl");
        let pid = keys.contains(&"pid");
        let cid = keys.contains(&"cid");

        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

fn part_one(path: &str) -> Result<usize> {
    Ok(one_separated_by_empty_line::<Passport>(path)?
        .iter()
        .filter(|passport| passport.is_valid())
        .count())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one("./data/4.input")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let res = part_one("./data/4.test").unwrap();
        assert_eq!(res, 2);
    }
}
