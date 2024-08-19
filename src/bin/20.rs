use std::str::FromStr;

advent_of_code::solution!(20);

#[derive(Clone, Copy, Eq)]
struct Point {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.p
            .0
            .cmp(&other.p.0)
            .then_with(|| self.p.1.cmp(&other.p.1))
            .then_with(|| self.p.2.cmp(&other.p.2))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Point {
    fn update(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }

    fn magnitude(&self) -> u64 {
        self.p.0.unsigned_abs() + self.p.1.unsigned_abs() + self.p.2.unsigned_abs()
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");
        let mut ps = parts
            .next()
            .unwrap()
            .strip_prefix("p=<")
            .unwrap()
            .strip_suffix(">")
            .unwrap()
            .split(',')
            .map(|s| s.trim());
        let mut vs = parts
            .next()
            .unwrap()
            .strip_prefix("v=<")
            .unwrap()
            .strip_suffix(">")
            .unwrap()
            .split(',')
            .map(|s| s.trim());
        let mut aa = parts
            .next()
            .unwrap()
            .strip_prefix("a=<")
            .unwrap()
            .strip_suffix(">")
            .unwrap()
            .split(',')
            .map(|s| s.trim());
        let p = (
            ps.next().unwrap().parse()?,
            ps.next().unwrap().parse()?,
            ps.next().unwrap().parse()?,
        );
        let v = (
            vs.next().unwrap().parse()?,
            vs.next().unwrap().parse()?,
            vs.next().unwrap().parse()?,
        );
        let a = (
            aa.next().unwrap().parse()?,
            aa.next().unwrap().parse()?,
            aa.next().unwrap().parse()?,
        );
        Ok(Self { p, v, a })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut particles = input
        .lines()
        .map(|l| l.parse::<Point>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..4000 {
        for p in &mut particles {
            p.update();
        }
    }
    Some(
        particles
            .iter()
            .enumerate()
            .min_by_key(|(_, p)| p.magnitude())
            .unwrap()
            .0 as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut particles = input
        .lines()
        .map(|l| l.parse::<Point>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..40 {
        particles.sort();
        let mut dupes = {
            let mut v = Vec::new();
            for (i, p) in particles.iter().enumerate() {
                if particles
                    .iter()
                    .enumerate()
                    .any(|(j, p2)| p2 == p && i != j)
                {
                    v.extend(
                        particles
                            .iter()
                            .enumerate()
                            .filter(|&(_, p2)| p2 == p)
                            .map(|(j, _)| j),
                    )
                }
            }
            v
        };
        dupes.sort_by_key(|k| std::cmp::Reverse(*k));
        dupes.dedup();
        for i in dupes {
            particles.remove(i);
        }
        for p in &mut particles {
            p.update();
        }
    }
    Some(particles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));
    }
}
