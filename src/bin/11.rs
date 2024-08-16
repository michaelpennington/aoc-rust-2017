use std::{ops::Add, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(11);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    NE,
    N,
    NW,
    SE,
    S,
    SW,
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ne" => Ok(Self::NE),
            "n" => Ok(Self::N),
            "nw" => Ok(Self::NW),
            "se" => Ok(Self::SE),
            "s" => Ok(Self::S),
            "sw" => Ok(Self::SW),
            _ => Err(anyhow!("{s} is not a valid dir")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct HexCoord {
    q: isize,
    r: isize,
    s: isize,
}

impl Add<Dir> for HexCoord {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self {
        let HexCoord { q, r, s } = self;
        match rhs {
            Dir::NE => Self {
                q: q + 1,
                r: r - 1,
                s,
            },
            Dir::N => Self {
                q,
                r: r - 1,
                s: s + 1,
            },
            Dir::NW => Self {
                q: q - 1,
                r,
                s: s + 1,
            },
            Dir::SE => Self {
                q: q + 1,
                r,
                s: s - 1,
            },
            Dir::S => Self {
                q,
                r: r + 1,
                s: s - 1,
            },
            Dir::SW => Self {
                q: q - 1,
                r: r + 1,
                s,
            },
        }
    }
}

impl HexCoord {
    fn distance(&self) -> usize {
        (self.q.unsigned_abs() + self.r.unsigned_abs() + self.s.unsigned_abs()) / 2
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let end = input
        .trim()
        .split(',')
        .map(|d| d.parse().unwrap())
        .fold(HexCoord::default(), |acc, d| acc + d);
    Some(end.distance() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut coord = HexCoord::default();
    input
        .trim()
        .split(',')
        .map(|d| d.parse().unwrap())
        .map(|d| {
            coord = coord + d;
            coord.distance() as u32
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one3() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one4() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }
}
