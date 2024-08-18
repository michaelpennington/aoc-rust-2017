use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::{anyhow, Context};

advent_of_code::solution!(18);

#[repr(usize)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Register {
    A = 0,
    B = 1,
    F = 2,
    I = 3,
    P = 4,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "f" => Ok(Self::F),
            "i" => Ok(Self::I),
            "p" => Ok(Self::P),
            _ => Err(anyhow!("{s} is not a register")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Value {
    Reg(Register),
    Imm(isize),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Self::Imm)
            .or_else(|e| s.parse().map(Self::Reg).context(e))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Jgz(Value, Value),
    Snd(Value),
    Rcv(Register),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let bd = || anyhow!("Unexpected end of string {s}");
        match parts.next().ok_or(bd())? {
            "set" => {
                let reg = parts.next().ok_or(bd())?.parse()?;
                let val = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Set(reg, val))
            }
            "add" => {
                let reg = parts.next().ok_or(bd())?.parse()?;
                let val = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Add(reg, val))
            }
            "mul" => {
                let reg = parts.next().ok_or(bd())?.parse()?;
                let val = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Mul(reg, val))
            }
            "mod" => {
                let reg = parts.next().ok_or(bd())?.parse()?;
                let val = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Mod(reg, val))
            }
            "jgz" => {
                let v1 = parts.next().ok_or(bd())?.parse()?;
                let v2 = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Jgz(v1, v2))
            }
            "snd" => {
                let reg = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Snd(reg))
            }
            "rcv" => {
                let reg = parts.next().ok_or(bd())?.parse()?;
                Ok(Self::Rcv(reg))
            }
            s => Err(anyhow!("unknown instruction {s}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Computer {
    registers: [isize; 5],
    now_playing: Option<isize>,
    pc: usize,
}

impl Index<Register> for Computer {
    type Output = isize;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for Computer {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Computer {
    fn val(&self, val: Value) -> isize {
        match val {
            Value::Reg(r) => self[r],
            Value::Imm(i) => i,
        }
    }

    fn compute_one(&mut self, i: Instruction) -> Option<isize> {
        let mut sound = None;
        match i {
            Instruction::Set(r, v) => self[r] = self.val(v),
            Instruction::Add(r, v) => self[r] += self.val(v),
            Instruction::Mul(r, v) => self[r] *= self.val(v),
            Instruction::Mod(r, v) => self[r] %= self.val(v),
            Instruction::Jgz(r, v) => {
                if self.val(r) > 0 {
                    self.pc = self.pc.wrapping_add_signed(self.val(v).wrapping_sub(1));
                }
            }
            Instruction::Snd(r) => self.now_playing = Some(self.val(r)),
            Instruction::Rcv(r) => {
                if self[r] != 0 {
                    if let Some(s) = self.now_playing {
                        sound = Some(s);
                    }
                }
            }
        }
        self.pc += 1;
        sound
    }

    fn compute_to_first_sound(&mut self, is: &[Instruction]) -> Option<isize> {
        while let Some(i) = is.get(self.pc) {
            if let Some(snd) = self.compute_one(*i) {
                return Some(snd);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct DoubleComputer {
    comp_a: Computer,
    comp_b: Computer,
    queue_a: VecDeque<isize>,
    queue_b: VecDeque<isize>,
}

impl Default for DoubleComputer {
    fn default() -> Self {
        let comp_a = Computer::default();
        let comp_b = Computer {
            registers: [0, 0, 0, 0, 1],
            ..Default::default()
        };
        Self {
            comp_a,
            comp_b,
            queue_a: VecDeque::new(),
            queue_b: VecDeque::new(),
        }
    }
}

impl DoubleComputer {
    fn compute(&mut self, is: &[Instruction]) -> u32 {
        let mut count = 0;
        loop {
            let mut a_done = false;
            let mut b_done = false;
            if let Some(i) = is.get(self.comp_a.pc) {
                match *i {
                    Instruction::Set(r, v) => self.comp_a[r] = self.comp_a.val(v),
                    Instruction::Add(r, v) => self.comp_a[r] += self.comp_a.val(v),
                    Instruction::Mul(r, v) => self.comp_a[r] *= self.comp_a.val(v),
                    Instruction::Mod(r, v) => self.comp_a[r] %= self.comp_a.val(v),
                    Instruction::Jgz(r, v) => {
                        if self.comp_a.val(r) > 0 {
                            self.comp_a.pc = self
                                .comp_a
                                .pc
                                .wrapping_add_signed(self.comp_a.val(v).wrapping_sub(1))
                        }
                    }
                    Instruction::Snd(r) => self.queue_b.push_front(self.comp_a.val(r)),
                    Instruction::Rcv(r) => {
                        if let Some(n) = self.queue_a.pop_back() {
                            self.comp_a[r] = n;
                        } else {
                            self.comp_a.pc = self.comp_a.pc.wrapping_sub(1);
                            a_done = true;
                        }
                    }
                }
                self.comp_a.pc += 1;
            }
            if let Some(i) = is.get(self.comp_b.pc) {
                match *i {
                    Instruction::Set(r, v) => self.comp_b[r] = self.comp_b.val(v),
                    Instruction::Add(r, v) => self.comp_b[r] += self.comp_b.val(v),
                    Instruction::Mul(r, v) => self.comp_b[r] *= self.comp_b.val(v),
                    Instruction::Mod(r, v) => self.comp_b[r] %= self.comp_b.val(v),
                    Instruction::Jgz(r, v) => {
                        if self.comp_b.val(r) > 0 {
                            self.comp_b.pc = self
                                .comp_b
                                .pc
                                .wrapping_add_signed(self.comp_b.val(v).wrapping_sub(1))
                        }
                    }
                    Instruction::Snd(r) => {
                        self.queue_a.push_front(self.comp_b.val(r));
                        count += 1;
                    }
                    Instruction::Rcv(r) => {
                        if let Some(n) = self.queue_b.pop_back() {
                            self.comp_b[r] = n;
                        } else {
                            self.comp_b.pc = self.comp_b.pc.wrapping_sub(1);
                            b_done = true;
                        }
                    }
                }
                self.comp_b.pc += 1;
            }

            if a_done && b_done {
                break count;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let instructions: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut comp = Computer::default();
    comp.compute_to_first_sound(&instructions)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut comp = DoubleComputer::default();
    Some(comp.compute(&instructions))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
    }
}
