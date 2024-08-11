advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .max()
                    .unwrap()
                    - l.split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .min()
                        .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        for (n1, n2) in nums
            .clone()
            .iter()
            .flat_map(|n1| nums.iter().map(move |n2| (n1, n2)))
            .filter(|(n1, n2)| n1 != n2)
        {
            if n1 % n2 == 0 {
                sum += n1 / n2;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9));
    }
}
