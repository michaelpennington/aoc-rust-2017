use anyhow::anyhow;

use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Condition {
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
}

impl Condition {
    fn check(&self, a: isize, b: isize) -> bool {
        match self {
            Condition::Gt => a > b,
            Condition::Lt => a < b,
            Condition::Ge => a >= b,
            Condition::Le => a <= b,
            Condition::Eq => a == b,
            Condition::Ne => a != b,
        }
    }
}

impl FromStr for Condition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Lt),
            "<=" => Ok(Self::Le),
            ">" => Ok(Self::Gt),
            ">=" => Ok(Self::Ge),
            "==" => Ok(Self::Eq),
            "!=" => Ok(Self::Ne),
            s => Err(anyhow!("{s} is not a valid condition")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Offset {
    Inc,
    Dec,
}

impl Offset {
    fn calc(&self, amount: isize, target: &mut isize) {
        match self {
            Offset::Inc => *target += amount,
            Offset::Dec => *target -= amount,
        }
    }
}

impl FromStr for Offset {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Self::Inc),
            "dec" => Ok(Self::Dec),
            s => Err(anyhow!("{s} is not a valid offset")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Instruction<'a> {
    target: &'a str,
    offset: Offset,
    amount: isize,
    check: &'a str,
    condition: Condition,
    value: isize,
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> anyhow::Result<Self> {
        let mut parts = s.split_whitespace();
        let target = parts.next().unwrap();
        let offset = parts.next().unwrap().parse()?;
        let amount = parts.next().unwrap().parse()?;
        let check = parts.nth(1).unwrap();
        let condition = parts.next().unwrap().parse()?;
        let value = parts.next().unwrap().parse()?;
        Ok(Self {
            target,
            offset,
            amount,
            check,
            condition,
            value,
        })
    }
}

struct Computer<'a> {
    registers: HashMap<&'a str, isize>,
}

impl<'a> Computer<'a> {
    fn compute(&mut self, program: impl IntoIterator<Item = Instruction<'a>>) -> Option<isize> {
        for instruction in program {
            let check = *self.registers.entry(instruction.check).or_insert(0);
            if instruction.condition.check(check, instruction.value) {
                instruction.offset.calc(
                    instruction.amount,
                    self.registers.entry(instruction.target).or_insert(0),
                );
            }
        }
        self.registers.values().max().copied()
    }

    fn compute_highest(
        &mut self,
        program: impl IntoIterator<Item = Instruction<'a>>,
    ) -> Option<isize> {
        let mut highest = 0;
        for instruction in program {
            let check = *self.registers.entry(instruction.check).or_insert(0);
            if instruction.condition.check(check, instruction.value) {
                instruction.offset.calc(
                    instruction.amount,
                    self.registers.entry(instruction.target).or_insert(0),
                );
                highest = highest.max(self.registers.values().max().copied().unwrap_or(0));
            }
        }
        Some(highest)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut comp = Computer {
        registers: HashMap::new(),
    };
    let instructions = input.lines().map(|l| Instruction::from_str(l).unwrap());
    comp.compute(instructions)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut comp = Computer {
        registers: HashMap::new(),
    };
    let instructions = input.lines().map(|l| Instruction::from_str(l).unwrap());
    comp.compute_highest(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }
}
