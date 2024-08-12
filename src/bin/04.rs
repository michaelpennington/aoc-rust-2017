use std::collections::HashSet;

advent_of_code::solution!(4);

fn contains_dups(s: &str) -> bool {
    let mut set = HashSet::new();
    s.split_whitespace().all(|w| set.insert(w))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Chars([u32; 26]);

impl Chars {
    fn insert(&mut self, c: char) {
        self.0[c as usize - 'a' as usize] += 1
    }
}

fn contains_angrams(s: &str) -> bool {
    let mut set = HashSet::new();
    for word in s.split_whitespace() {
        let mut chars = Chars([0; 26]);
        for c in word.chars() {
            chars.insert(c);
        }
        if !set.insert(chars) {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().filter(|l| contains_dups(l)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter(|l| contains_dups(l))
            .filter(|l| contains_angrams(l))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
