advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<usize> {
    let step = input.trim().parse::<usize>().unwrap();
    let mut buf = Vec::with_capacity(2018);
    buf.push(0);
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + step) % buf.len() + 1;
        buf.insert(pos, i);
    }
    buf.get(pos + 1).copied()
}

pub fn part_two(input: &str) -> Option<usize> {
    let step = input.trim().parse::<usize>().unwrap();
    let mut pos = 0;
    let mut last = 0;
    for i in 1..=50_000_000 {
        pos = (pos + step) % i + 1;
        if pos == 1 {
            last = i;
        }
    }
    Some(last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(638));
    }
}
