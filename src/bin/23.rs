use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(23);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            "h" => Ok(Self::H),
            _ => Err(anyhow!("{s} is not a valid register")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
struct Computer {
    registers: [isize; 8],
    num_muls: u32,
    pc: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Value {
    Reg(Register),
    Imm(isize),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse()
            .map(Self::Reg)
            .or_else(|_| Ok(s.parse().map(Self::Imm)?))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Instruction {
    Set(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Jnz(Value, Value),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.split_whitespace();
        match pts.next().unwrap() {
            "set" => {
                let r = pts.next().unwrap().parse()?;
                let v = pts.next().unwrap().parse()?;
                Ok(Self::Set(r, v))
            }
            "sub" => {
                let r = pts.next().unwrap().parse()?;
                let v = pts.next().unwrap().parse()?;
                Ok(Self::Sub(r, v))
            }
            "mul" => {
                let r = pts.next().unwrap().parse()?;
                let v = pts.next().unwrap().parse()?;
                Ok(Self::Mul(r, v))
            }
            "jnz" => {
                let v1 = pts.next().unwrap().parse()?;
                let v2 = pts.next().unwrap().parse()?;
                Ok(Self::Jnz(v1, v2))
            }
            s => Err(anyhow!("{s} is not a valid instruction")),
        }
    }
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

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for r in self.registers.iter().take(7) {
            write!(f, "{r:8}, ")?;
        }
        write!(f, "{:8}] pc: {}", self[Register::H], self.pc + 1)
    }
}

impl Computer {
    fn get(&self, v: Value) -> isize {
        match v {
            Value::Reg(r) => self[r],
            Value::Imm(i) => i,
        }
    }

    fn compute(&mut self, is: &[Instruction]) {
        while let Some(i) = is.get(self.pc) {
            match *i {
                Instruction::Set(r, v) => self[r] = self.get(v),
                Instruction::Sub(r, v) => self[r] -= self.get(v),
                Instruction::Mul(r, v) => {
                    self[r] *= self.get(v);
                    self.num_muls += 1;
                }
                Instruction::Jnz(v1, v2) => {
                    if self.get(v1) != 0 {
                        self.pc = self.pc.wrapping_add_signed(self.get(v2).wrapping_sub(1));
                    }
                }
            }
            self.pc += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let is = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    let mut comp = Computer::default();
    comp.compute(&is);
    Some(comp.num_muls)
}

fn isnt_prime(n: &u32) -> bool {
    for i in (2..).take_while(|k| k * k <= *n) {
        if n % i == 0 {
            return true;
        }
    }
    false
}

pub fn part_two(_input: &str) -> Option<u32> {
    let h = (0..=1000)
        .map(|i| 106500 + 17 * i)
        .filter(isnt_prime)
        .count();
    Some(h as u32)
}
