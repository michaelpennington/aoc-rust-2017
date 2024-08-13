use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Memory {
    inner: [u8; 24],
    len: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct MemIter {
    mem: Memory,
}

impl Iterator for MemIter {
    type Item = Memory;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.mem;
        self.mem.distribute();
        Some(out)
    }
}

impl IntoIterator for Memory {
    type Item = Memory;

    type IntoIter = MemIter;

    fn into_iter(self) -> Self::IntoIter {
        MemIter { mem: self }
    }
}

impl Memory {
    fn from_slice(slice: &[u8]) -> Self {
        if slice.len() > 24 {
            panic!("Arr must have length 24 or less, has {}", slice.len());
        }
        let mut inner = [0; 24];
        for (i, byte) in slice.iter().enumerate() {
            inner[i] = *byte;
        }
        Self {
            inner,
            len: slice.len(),
        }
    }

    fn distribute(&mut self) {
        let inner = &mut self.inner[..self.len];
        let max = *inner.iter().max().unwrap();
        let mpos = inner.iter().position(|&n| n == max).unwrap();
        inner[mpos] = 0;
        for i in 1..=(max as usize) {
            inner[(mpos + i) % self.len] += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let items = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    let mem = Memory::from_slice(&items);
    let mut seen = HashSet::new();
    for (i, m) in mem.into_iter().enumerate() {
        if !seen.insert(m) {
            return Some(i as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let items = input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    let mem = Memory::from_slice(&items);
    let mut seen = HashMap::new();
    for (i, m) in mem.into_iter().enumerate() {
        if let Some(j) = seen.insert(m, i) {
            return Some((i - j) as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
