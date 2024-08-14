use std::collections::HashMap;

advent_of_code::solution!(3);

fn isqrt(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut op = n;
    let mut res = 0;
    let mut one = 1 << (n.ilog2() & !1);

    while one != 0 {
        if op >= res + one {
            op -= res + one;
            res = (res >> 1) + one;
        } else {
            res >>= 1;
        }
        one >>= 2;
    }

    res
}

fn coords(n: u32) -> (i32, i32) {
    let m = isqrt(n) as i32;
    let m2 = ((n as f64).sqrt() * 2.0).floor() as i32;
    let n = n as i32;
    let mut x = (m + 1) / 2;
    let mut y = -x;
    if m2 % 2 == 0 {
        x += n - m * (m + 1);
    } else {
        y += n - m * (m + 1);
    }
    if m % 2 == 1 {
        x = -x;
        y = -y;
    }
    (x, y)
}

pub fn part_one(input: &str) -> Option<u32> {
    let n = input.trim().parse::<u32>().unwrap();
    let (x, y) = coords(n - 1);
    Some(x.unsigned_abs() + y.unsigned_abs())
}

pub fn part_two(input: &str) -> Option<u32> {
    let n = input.trim().parse::<u32>().unwrap();
    let mut map = HashMap::new();
    map.insert((0, 0), 1);
    for (x, y) in (1..).map(coords) {
        let new: u32 = (x - 1..=x + 1)
            .flat_map(|x| (y - 1..=y + 1).map(move |y| (x, y)))
            .map(|pt| map.get(&pt).copied().unwrap_or_default())
            .sum();
        if new > n {
            return Some(new);
        }
        map.insert((x, y), new);
    }
    None
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
        assert_eq!(result, Some(1968));
    }
}
