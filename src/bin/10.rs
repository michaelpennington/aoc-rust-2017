use std::{
    fmt::LowerHex,
    ops::{Index, IndexMut},
};

advent_of_code::solution!(10);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SkipList {
    list: [u8; 256],
    pos: usize,
    skip_size: usize,
}

impl Default for SkipList {
    fn default() -> Self {
        let mut list = [0; 256];
        for i in 0..=255u8 {
            list[i as usize] = i;
        }
        Self {
            list,
            pos: 0,
            skip_size: 0,
        }
    }
}

impl Index<usize> for SkipList {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index % 256]
    }
}

impl IndexMut<usize> for SkipList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.list[index % 256]
    }
}

impl SkipList {
    fn reverse(&mut self, start: usize, end: usize) {
        let elements = (start..end).map(|i| self[i]).collect::<Vec<_>>();
        for (i, j) in (start..end).zip((0..elements.len()).map(|j| elements.len() - j - 1)) {
            self[i] = elements[j];
        }
    }

    fn rotate(&mut self, length: usize) {
        self.reverse(self.pos, self.pos + length);
        self.pos += length + self.skip_size;
        self.skip_size += 1;
    }
}

impl LowerHex for SkipList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..16 {
            let num = (0..16)
                .map(|j| self[16 * i + j])
                .reduce(|acc, e| acc ^ e)
                .unwrap();
            write!(f, "{num:02x}")?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut skip_list = SkipList::default();
    for n in input.trim().split(',').map(|n| n.parse().unwrap()) {
        skip_list.rotate(n);
    }
    Some(skip_list[0] as u32 * skip_list[1] as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let nums = input
        .trim()
        .as_bytes()
        .iter()
        .chain([17, 31, 73, 47, 23].iter())
        .collect::<Vec<_>>();
    let mut skip_list = SkipList::default();
    for _ in 0..64 {
        for &n in &nums {
            skip_list.rotate(*n as usize);
        }
    }
    Some(format!("{:x}", skip_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("a2582a3a0e66e6e86e3812dcb672a272".into()))
    }

    #[test]
    fn test_part_two2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("33efeb34ea91902bb2f59c9920caa6cd".into()))
    }

    #[test]
    fn test_part_two3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some("3efbe78a8d82f29979031a4aa0b16a9d".into()))
    }

    #[test]
    fn test_part_two4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some("63960835bcdc130f0b66d7ff4f6a5a8e".into()))
    }
}
