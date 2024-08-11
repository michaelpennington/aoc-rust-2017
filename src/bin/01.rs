use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let chars = input
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    Some(
        chars
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| if a == b { a.parse::<u32>().unwrap() } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars = input
        .trim()
        .split("")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let len = chars.len();
    Some(
        chars
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if *c == chars[(i + len / 2) % len] {
                    c.parse::<u32>().unwrap()
                } else {
                    0
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_six() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_seven() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_eight() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 8,
        ));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_nine() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 9,
        ));
        assert_eq!(result, Some(4));
    }
}
