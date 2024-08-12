advent_of_code::solution!(5);

#[derive(Clone, PartialEq, Eq, Debug)]
struct Computer {
    program: Vec<isize>,
    pc: usize,
}

impl Computer {
    fn new_with_program(program: Vec<isize>) -> Self {
        Self { program, pc: 0 }
    }

    fn calc(&mut self) -> u32 {
        let mut counter = 0;
        while let Some(offset) = self.program.get_mut(self.pc) {
            self.pc = self.pc.wrapping_add_signed(*offset);
            *offset += 1;
            counter += 1;
        }
        counter
    }

    fn calc_v2(&mut self) -> u32 {
        let mut counter = 0;
        while let Some(offset) = self.program.get_mut(self.pc) {
            self.pc = self.pc.wrapping_add_signed(*offset);
            if *offset >= 3 {
                *offset -= 1;
            } else {
                *offset += 1;
            }
            counter += 1;
        }
        counter
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let program = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    Some(Computer::new_with_program(program).calc())
}

pub fn part_two(input: &str) -> Option<u32> {
    let program = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    Some(Computer::new_with_program(program).calc_v2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
