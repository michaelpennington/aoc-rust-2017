use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(13);

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Firewall {
    map: HashMap<u16, u16>,
    largest: u16,
}

impl FromStr for Firewall {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::with_capacity(s.lines().count());
        let mut largest = 0;
        for line in s.lines() {
            let (i, len) = line
                .split_once(": ")
                .ok_or(anyhow!("line must contain ': '"))
                .and_then(|(i, len)| Ok((i.parse()?, len.parse()?)))?;
            largest = largest.max(i);
            map.insert(i, len);
        }
        Ok(Self { map, largest })
    }
}

impl Firewall {
    fn severity(&self) -> u32 {
        (0..=self.largest)
            .filter_map(|i| self.map.get(&i).map(|n| (i, n)))
            .map(|(i, n)| if i % (2 * (n - 1)) == 0 { i * n } else { 0 })
            .sum::<u16>() as u32
    }

    fn severity_with_delay(&self, delay: u32) -> u32 {
        (0..=self.largest)
            .filter_map(|i| self.map.get(&i).map(|n| (i, n)))
            .map(|(i, n)| {
                if (i as u32 + delay) % (2 * (n - 1)) as u32 == 0 {
                    if i == 0 {
                        1
                    } else {
                        i as u32 * *n as u32
                    }
                } else {
                    0
                }
            })
            .sum::<u32>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let firewall = Firewall::from_str(input).unwrap();
    Some(firewall.severity())
}

pub fn part_two(input: &str) -> Option<u32> {
    let firewall = Firewall::from_str(input).unwrap();
    (0u32..).find(|i| firewall.severity_with_delay(*i) == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
