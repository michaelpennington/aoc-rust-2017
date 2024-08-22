use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(25);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
enum State {
    #[default]
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
}

impl FromStr for State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            _ => Err(anyhow!("{s} is not a valid state")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Dir {
    #[default]
    Left,
    Right,
}

impl Dir {
    fn mv(&self, i: &mut isize) {
        match self {
            Dir::Left => *i -= 1,
            Dir::Right => *i += 1,
        }
    }
}

impl FromStr for Dir {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(anyhow!("{s} is not a valid dir")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Value {
    #[default]
    Zero,
    One,
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::Zero),
            "1" => Ok(Self::One),
            _ => Err(anyhow!("{s} is not a valid value")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct NextSteps {
    write: Value,
    dir: Dir,
    next: State,
}

impl FromStr for NextSteps {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.lines().map(|l| l.trim().trim_end_matches("."));
        let write = l
            .next()
            .unwrap()
            .trim_start_matches("- Write the value ")
            .parse()?;
        let dir = l
            .next()
            .unwrap()
            .trim_start_matches("- Move one slot to the ")
            .parse()?;
        let next = l
            .next()
            .unwrap()
            .trim_start_matches("- Continue with state ")
            .parse()?;
        Ok(Self { write, dir, next })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Next {
    if_zero: NextSteps,
    if_one: NextSteps,
}

impl FromStr for Next {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new_lines = s.match_indices('\n');
        let start1 = new_lines.next().unwrap().0 + 1;
        let end1 = new_lines.nth(2).unwrap().0;
        let start2 = new_lines.next().unwrap().0 + 1;
        let if_zero = s[start1..end1].parse()?;
        let if_one = s[start2..].parse()?;
        Ok(Self { if_zero, if_one })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Rules {
    states: [Next; 6],
    start: State,
    diagnostic_after: usize,
}

impl FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pt1, rest) = s.split_once("\n\n").unwrap();
        let mut pt1 = pt1.lines();
        let mut states = [Next::default(); 6];
        let start = pt1
            .next()
            .unwrap()
            .trim_start_matches("Begin in state ")
            .trim_end_matches('.')
            .parse()?;
        let diagnostic_after = pt1
            .next()
            .unwrap()
            .trim_start_matches("Perform a diagnostic checksum after ")
            .trim_end_matches(" steps.")
            .parse()?;

        for chunk in rest.split("\n\n") {
            let (state, rest) = chunk.split_once('\n').unwrap();
            let state: State = state
                .trim_start_matches("In state ")
                .trim_end_matches(":")
                .parse()?;
            let next = rest.parse()?;
            states[state as usize] = next;
        }
        Ok(Self {
            states,
            start,
            diagnostic_after,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Computer {
    tape: HashMap<isize, Value>,
    pc: isize,
}

impl Computer {
    fn compute(&mut self, rules: &Rules) -> u32 {
        let mut state = rules.start;
        for _ in 0..rules.diagnostic_after {
            let rule = rules.states[state as usize];
            let rule = match self.tape.get(&self.pc) {
                Some(Value::One) => rule.if_one,
                _ => rule.if_zero,
            };
            self.tape.insert(self.pc, rule.write);
            rule.dir.mv(&mut self.pc);
            state = rule.next;
        }
        self.cksm()
    }

    fn cksm(&self) -> u32 {
        self.tape
            .values()
            .map(|v| if *v == Value::One { 1 } else { 0 })
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules = input.parse().unwrap();
    let mut computer = Computer::default();
    Some(computer.compute(&rules))
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
