use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(19);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    LR,
    Junc,
    UD,
    Letter(char),
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::UD),
            '-' => Ok(Self::LR),
            '+' => Ok(Self::Junc),
            c if c.is_ascii_uppercase() => Ok(Self::Letter(c)),
            _ => Err(anyhow!("{value} is not a valid tile")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    tiles: HashMap<(usize, usize), Tile>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.char_indices() {
                if c != ' ' {
                    tiles.insert((x, y), c.try_into()?);
                }
            }
        }
        Ok(Self { tiles })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    D,
    L,
    R,
    U,
}

impl Dir {
    fn next(&self, coords: (usize, usize)) -> (usize, usize) {
        let (x, y) = coords;
        match self {
            Dir::D => (x, y + 1),
            Dir::L => (x - 1, y),
            Dir::R => (x + 1, y),
            Dir::U => (x, y - 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let map = input.parse::<Map>().unwrap();
    let mut dir = Dir::D;
    let mut s = String::new();
    let mut loc = *map.tiles.keys().find(|&(_, y)| *y == 0).unwrap();
    loc = dir.next(loc);
    while let Some(t) = map.tiles.get(&loc) {
        match (t, dir) {
            (Tile::Junc, Dir::D | Dir::U) => {
                match (
                    map.tiles.get(&Dir::L.next(loc)),
                    map.tiles.get(&Dir::R.next(loc)),
                ) {
                    (None, None) => unreachable!(),
                    (None, Some(t)) => match t {
                        Tile::LR | Tile::Junc | Tile::Letter(_) => dir = Dir::R,
                        Tile::UD => unreachable!(),
                    },
                    (Some(t), None) => match t {
                        Tile::LR | Tile::Junc | Tile::Letter(_) => dir = Dir::L,
                        Tile::UD => unreachable!(),
                    },
                    (Some(t1), Some(t2)) => match (t1, t2) {
                        (Tile::LR | Tile::Junc | Tile::Letter(_), Tile::UD) => dir = Dir::L,
                        (Tile::UD, Tile::LR | Tile::Junc | Tile::Letter(_)) => dir = Dir::R,
                        (
                            Tile::Junc | Tile::Letter(_) | Tile::LR,
                            Tile::LR | Tile::Junc | Tile::Letter(_),
                        )
                        | (Tile::UD, Tile::UD) => {
                            unreachable!()
                        }
                    },
                }
            }
            (Tile::Junc, Dir::L | Dir::R) => {
                match (
                    map.tiles.get(&Dir::U.next(loc)),
                    map.tiles.get(&Dir::D.next(loc)),
                ) {
                    (None, None) => unreachable!(),
                    (None, Some(t)) => match t {
                        Tile::UD | Tile::Junc | Tile::Letter(_) => dir = Dir::D,
                        Tile::LR => unreachable!(),
                    },
                    (Some(t), None) => match t {
                        Tile::UD | Tile::Junc | Tile::Letter(_) => dir = Dir::U,
                        Tile::LR => unreachable!(),
                    },
                    (Some(t1), Some(t2)) => match (t1, t2) {
                        (Tile::UD | Tile::Junc | Tile::Letter(_), Tile::LR) => dir = Dir::U,
                        (Tile::LR, Tile::UD | Tile::Junc | Tile::Letter(_)) => dir = Dir::D,
                        (
                            Tile::Junc | Tile::Letter(_) | Tile::UD,
                            Tile::UD | Tile::Junc | Tile::Letter(_),
                        )
                        | (Tile::LR, Tile::LR) => {
                            unreachable!()
                        }
                    },
                }
            }
            (Tile::Letter(l), _) => s.push(*l),
            _ => {}
        }
        loc = dir.next(loc);
    }
    Some(s)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input.parse::<Map>().unwrap();
    let mut dir = Dir::D;
    let mut loc = *map.tiles.keys().find(|&(_, y)| *y == 0).unwrap();
    loc = dir.next(loc);
    let mut i = 1;
    while let Some(t) = map.tiles.get(&loc) {
        match (t, dir) {
            (Tile::Junc, Dir::D | Dir::U) => {
                match (
                    map.tiles.get(&Dir::L.next(loc)),
                    map.tiles.get(&Dir::R.next(loc)),
                ) {
                    (None, None) => unreachable!(),
                    (None, Some(t)) => match t {
                        Tile::LR | Tile::Junc | Tile::Letter(_) => dir = Dir::R,
                        Tile::UD => unreachable!(),
                    },
                    (Some(t), None) => match t {
                        Tile::LR | Tile::Junc | Tile::Letter(_) => dir = Dir::L,
                        Tile::UD => unreachable!(),
                    },
                    (Some(t1), Some(t2)) => match (t1, t2) {
                        (Tile::LR | Tile::Junc | Tile::Letter(_), Tile::UD) => dir = Dir::L,
                        (Tile::UD, Tile::LR | Tile::Junc | Tile::Letter(_)) => dir = Dir::R,
                        (
                            Tile::Junc | Tile::Letter(_) | Tile::LR,
                            Tile::LR | Tile::Junc | Tile::Letter(_),
                        )
                        | (Tile::UD, Tile::UD) => {
                            unreachable!()
                        }
                    },
                }
            }
            (Tile::Junc, Dir::L | Dir::R) => {
                match (
                    map.tiles.get(&Dir::U.next(loc)),
                    map.tiles.get(&Dir::D.next(loc)),
                ) {
                    (None, None) => unreachable!(),
                    (None, Some(t)) => match t {
                        Tile::UD | Tile::Junc | Tile::Letter(_) => dir = Dir::D,
                        Tile::LR => unreachable!(),
                    },
                    (Some(t), None) => match t {
                        Tile::UD | Tile::Junc | Tile::Letter(_) => dir = Dir::U,
                        Tile::LR => unreachable!(),
                    },
                    (Some(t1), Some(t2)) => match (t1, t2) {
                        (Tile::UD | Tile::Junc | Tile::Letter(_), Tile::LR) => dir = Dir::U,
                        (Tile::LR, Tile::UD | Tile::Junc | Tile::Letter(_)) => dir = Dir::D,
                        (
                            Tile::Junc | Tile::Letter(_) | Tile::UD,
                            Tile::UD | Tile::Junc | Tile::Letter(_),
                        )
                        | (Tile::LR, Tile::LR) => {
                            unreachable!()
                        }
                    },
                }
            }
            _ => {}
        }
        i += 1;
        loc = dir.next(loc);
    }
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("ABCDEF".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(38));
    }
}
