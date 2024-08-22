use std::{fmt::Debug, str::FromStr, sync::OnceLock};

advent_of_code::solution!(24);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Component(u32, u32);

impl FromStr for Component {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once('/').unwrap();
        Ok(Self(x.parse()?, y.parse()?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ComponentList {
    components: [Component; 64],
    len: usize,
}

static COMPONENTS: OnceLock<ComponentList> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Bridge {
    components: [(usize, bool); 55],
    len: usize,
}

impl Debug for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let components = COMPONENTS.get().unwrap();
        for c in self.components.iter().take(self.len).map(|(i, b)| {
            let mut cs = components.components[*i];
            if *b {
                std::mem::swap(&mut cs.0, &mut cs.1);
            }
            cs
        }) {
            write!(f, "{}/{}--", c.0, c.1)?;
        }
        Ok(())
    }
}

impl Bridge {
    fn last(&self) -> &Component {
        &COMPONENTS.get().unwrap().components[self.components[self.len - 1].0]
    }

    fn next_available(&self) -> impl Iterator<Item = (usize, bool)> + '_ {
        let components = COMPONENTS.get().unwrap();
        components
            .components
            .iter()
            .take(components.len)
            .enumerate()
            .filter_map(|(i, c)| {
                let num = if self.len == 0 {
                    0
                } else {
                    let last = self.last();
                    if self.components[self.len - 1].1 {
                        last.0
                    } else {
                        last.1
                    }
                };
                (!self.components.contains(&(i, true))
                    && !self.components.contains(&(i, false))
                    && (c.0 == num || c.1 == num))
                    .then_some((i, c.1 == num))
            })
    }

    fn score(&self) -> u32 {
        let components = COMPONENTS.get().unwrap();
        self.components
            .iter()
            .take(self.len)
            .map(|(i, _)| {
                let comp = components.components[*i];
                comp.0 + comp.1
            })
            .sum()
    }

    fn add(&mut self, new: (usize, bool)) {
        self.components[self.len] = new;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    COMPONENTS.get_or_init(|| {
        let len = input.lines().count();
        let mut components = [Component::default(); 64];
        for (line, comp) in input.lines().zip(&mut components) {
            *comp = line.parse().unwrap();
        }
        ComponentList { components, len }
    });
    let start = Bridge {
        components: [(usize::MAX, false); 55],
        len: 0,
    };
    let mut bridges = vec![start];

    let mut final_bridges = Vec::new();
    let mut i = 0;
    loop {
        println!("i={i};\tnum bridges: {}", bridges.len());
        i += 1;
        let mut to_remove = Vec::new();
        let mut new_bs = Vec::new();
        for (i, bridge) in bridges.iter().enumerate() {
            let mut new_bridges = Vec::new();
            for new in bridge.next_available() {
                let mut bridge = *bridge;
                bridge.add(new);
                new_bridges.push(bridge);
            }
            if new_bridges.is_empty() {
                final_bridges.push(*bridge);
            } else {
                new_bs.extend(new_bridges);
            }
            to_remove.push(i);
        }
        to_remove.sort_by_key(|&i| std::cmp::Reverse(i));
        new_bs.sort();
        new_bs.dedup();
        if new_bs == bridges {
            break final_bridges.iter().map(|b| b.score()).max();
        }
        for i in to_remove {
            bridges.remove(i);
        }
        bridges = new_bs;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    COMPONENTS.get_or_init(|| {
        let len = input.lines().count();
        let mut components = [Component::default(); 64];
        for (line, comp) in input.lines().zip(&mut components) {
            *comp = line.parse().unwrap();
        }
        ComponentList { components, len }
    });
    let start = Bridge {
        components: [(usize::MAX, false); 55],
        len: 0,
    };
    let mut bridges = vec![start];

    let mut final_bridges = Vec::new();
    let mut i = 0;
    let mut longest = 0;
    loop {
        println!("i={i};\tnum bridges: {}", bridges.len());
        i += 1;
        let mut to_remove = Vec::new();
        let mut new_bs = Vec::new();
        for (i, bridge) in bridges.iter().enumerate() {
            let mut new_bridges = Vec::new();
            for new in bridge.next_available() {
                let mut bridge = *bridge;
                bridge.add(new);
                longest = bridge.len.max(longest);
                new_bridges.push(bridge);
            }
            if new_bridges.is_empty() {
                final_bridges.push(*bridge);
            } else {
                new_bs.extend(new_bridges);
            }
            to_remove.push(i);
        }
        final_bridges.retain(|b| b.len() == longest);
        to_remove.sort_by_key(|&i| std::cmp::Reverse(i));
        new_bs.sort();
        new_bs.dedup();
        if new_bs == bridges {
            break final_bridges.iter().map(|b| b.score()).max();
        }
        for i in to_remove {
            bridges.remove(i);
        }
        bridges = new_bs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
