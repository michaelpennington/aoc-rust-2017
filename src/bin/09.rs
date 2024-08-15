advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut seen_bang = false;
    let mut in_garbage = false;
    let mut stack = Vec::new();
    for c in input.trim().chars() {
        if seen_bang && in_garbage {
            seen_bang = false;
            continue;
        }
        match (c, in_garbage) {
            ('{', false) => stack.push(()),
            ('}', false) => {
                total += stack.len();
                stack.pop();
            }
            ('>', false) => unreachable!(),
            ('>', true) => in_garbage = false,
            ('!', true) => seen_bang = true,
            ('<', false) => in_garbage = true,
            (',', false) => {}
            (_, true) => {}
            (_, false) => unreachable!(),
        }
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let mut seen_bang = false;
    let mut in_garbage = false;
    for c in input.trim().chars() {
        if seen_bang && in_garbage {
            seen_bang = false;
            continue;
        }
        match (c, in_garbage) {
            ('{', false) => {}
            ('}', false) => {}
            ('>', false) => unreachable!(),
            ('>', true) => in_garbage = false,
            ('!', true) => seen_bang = true,
            ('<', false) => in_garbage = true,
            (',', false) => {}
            (_, true) => total += 1,
            (_, false) => unreachable!(),
        }
    }
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(75));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
