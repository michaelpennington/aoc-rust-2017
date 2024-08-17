advent_of_code::solution!(15);

fn lower_16_eq(a: u64, b: u64) -> bool {
    (a & 0xFFFF) == (b & 0xFFFF)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Generator {
    factor: u64,
}

impl Generator {
    fn gen(&self, old: u64) -> u64 {
        (old * self.factor) % 2147483647
    }

    fn gen_mod4(&self, old: u64) -> u64 {
        let mut new = old;
        loop {
            new = (new * self.factor) % 2147483647;
            if new & 0b0011 == 0 {
                break new;
            }
        }
    }

    fn gen_mod8(&self, old: u64) -> u64 {
        let mut new = old;
        loop {
            new = (new * self.factor) % 2147483647;
            if new & 0b0111 == 0 {
                break new;
            }
        }
    }
}

const GEN_A: Generator = Generator { factor: 16807 };
const GEN_B: Generator = Generator { factor: 48271 };

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut a = lines[0].split_whitespace().nth(4).unwrap().parse().unwrap();
    let mut b = lines[1].split_whitespace().nth(4).unwrap().parse().unwrap();
    let mut count = 0;
    for _ in 0..40_000_000 {
        a = GEN_A.gen(a);
        b = GEN_B.gen(b);
        if lower_16_eq(a, b) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut a = lines[0].split_whitespace().nth(4).unwrap().parse().unwrap();
    let mut b = lines[1].split_whitespace().nth(4).unwrap().parse().unwrap();
    let mut count = 0;
    for _ in 0..5_000_000 {
        a = GEN_A.gen_mod4(a);
        b = GEN_B.gen_mod8(b);
        if lower_16_eq(a, b) {
            count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(588));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(309));
    }
}
