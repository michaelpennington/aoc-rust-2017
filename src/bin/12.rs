use std::collections::HashSet;

advent_of_code::solution!(12);

#[derive(Clone, PartialEq, Eq, Debug)]
struct Partitions(Vec<HashSet<u16>>);

impl Default for Partitions {
    fn default() -> Self {
        Self(vec![[0].into()])
    }
}

impl Partitions {
    fn groups_containing(&self, num: u16) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(move |(i, ns)| ns.contains(&num).then_some(i))
    }

    fn add(&mut self, new_group: impl IntoIterator<Item = u16>) {
        let mut intersecting_groups = Vec::new();
        let new_nums = new_group.into_iter().collect::<HashSet<_>>();
        for &num in &new_nums {
            intersecting_groups.extend(self.groups_containing(num));
        }
        if intersecting_groups.is_empty() {
            self.0.push(new_nums);
        } else {
            intersecting_groups.sort();
            intersecting_groups.dedup();
            intersecting_groups.reverse();
            let first = intersecting_groups.pop().unwrap();
            self.0[first].extend(&new_nums);
            for i in intersecting_groups {
                let set = self.0.remove(i);
                self.0[first].extend(set);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut partitions = Partitions::default();
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .filter(|&s| s != "<->")
            .map(|s| s.trim_end_matches(',').parse().unwrap());
        partitions.add(nums);
    }
    partitions.0.first().map(|f| f.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut partitions = Partitions::default();
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .filter(|&s| s != "<->")
            .map(|s| s.trim_end_matches(',').parse().unwrap());
        partitions.add(nums);
    }
    Some(partitions.0.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
