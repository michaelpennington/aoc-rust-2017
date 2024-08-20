use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(22);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn left(self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }

    fn right(self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }

    fn l(&mut self) {
        *self = self.left();
    }

    fn r(&mut self) {
        *self = self.right();
    }

    fn rev(&mut self) {
        *self = self.reverse();
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Virus {
    loc: (i32, i32),
    dir: Dir,
}

impl Virus {
    fn step(&mut self) {
        let (x, y) = &mut self.loc;
        match self.dir {
            Dir::N => *y -= 1,
            Dir::S => *y += 1,
            Dir::E => *x += 1,
            Dir::W => *x -= 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl Node {
    fn progress(self) -> Self {
        match self {
            Node::Clean => Node::Weakened,
            Node::Weakened => Node::Infected,
            Node::Infected => Node::Flagged,
            Node::Flagged => Node::Clean,
        }
    }

    fn p(&mut self) {
        *self = self.progress();
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map2 {
    virus: Virus,
    nodes: HashMap<(i32, i32), Node>,
}

impl FromStr for Map2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut nodes = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    nodes.insert((x as i32, y as i32), Node::Infected);
                }
            }
        }
        let virus = Virus {
            loc: (width as i32 / 2, height as i32 / 2),
            dir: Dir::N,
        };
        Ok(Self { virus, nodes })
    }
}

impl Map2 {
    fn step(&mut self) -> bool {
        match self.nodes.get_mut(&self.virus.loc) {
            Some(node) => match node {
                Node::Clean => {
                    node.p();
                    self.virus.dir.l();
                    self.virus.step();
                    false
                }
                Node::Weakened => {
                    node.p();
                    self.virus.step();
                    true
                }
                Node::Infected => {
                    node.p();
                    self.virus.dir.r();
                    self.virus.step();
                    false
                }
                Node::Flagged => {
                    node.p();
                    self.virus.dir.rev();
                    self.virus.step();
                    false
                }
            },
            None => {
                self.nodes.insert(self.virus.loc, Node::Weakened);
                self.virus.dir.l();
                self.virus.step();
                false
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map {
    virus: Virus,
    nodes: HashMap<(i32, i32), bool>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let mut nodes = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                nodes.insert((x as i32, y as i32), c == '#');
            }
        }
        let virus = Virus {
            loc: (width as i32 / 2, height as i32 / 2),
            dir: Dir::N,
        };
        Ok(Self { virus, nodes })
    }
}

impl Map {
    fn step(&mut self) -> bool {
        match self.nodes.get_mut(&self.virus.loc) {
            Some(val) => {
                if *val {
                    *val = false;
                    self.virus.dir.r();
                    self.virus.step();
                    false
                } else {
                    *val = true;
                    self.virus.dir.l();
                    self.virus.step();
                    true
                }
            }
            None => {
                self.nodes.insert(self.virus.loc, true);
                self.virus.dir.l();
                self.virus.step();
                true
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::from_str(input).unwrap();
    Some((0..10000).filter(|_| map.step()).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map2 = Map2::from_str(input).unwrap();
    Some((0..10_000_000).filter(|_| map2.step()).count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5587));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2_511_944));
    }
}
