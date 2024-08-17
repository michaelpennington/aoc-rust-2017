use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(16);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Program {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    N,
    M,
    O,
    P,
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "a" => Ok(Self::A),
            "B" | "b" => Ok(Self::B),
            "C" | "c" => Ok(Self::C),
            "D" | "d" => Ok(Self::D),
            "E" | "e" => Ok(Self::E),
            "F" | "f" => Ok(Self::F),
            "G" | "g" => Ok(Self::G),
            "H" | "h" => Ok(Self::H),
            "I" | "i" => Ok(Self::I),
            "J" | "j" => Ok(Self::J),
            "K" | "k" => Ok(Self::K),
            "L" | "l" => Ok(Self::L),
            "M" | "m" => Ok(Self::M),
            "N" | "n" => Ok(Self::N),
            "O" | "o" => Ok(Self::O),
            "P" | "p" => Ok(Self::P),
            _ => Err(anyhow!("{s} is not a valid program")),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Program::A => "a",
            Program::B => "b",
            Program::C => "c",
            Program::D => "d",
            Program::E => "e",
            Program::F => "f",
            Program::G => "g",
            Program::H => "h",
            Program::I => "i",
            Program::J => "j",
            Program::K => "k",
            Program::L => "l",
            Program::M => "m",
            Program::N => "n",
            Program::O => "o",
            Program::P => "p",
        };
        write!(f, "{c}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LineDance([Program; 16]);

impl Default for LineDance {
    fn default() -> Self {
        use Program::*;
        Self([A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P])
    }
}

impl Display for LineDance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl LineDance {
    fn swap_p(&mut self, a: &Program, b: &Program) {
        let a = self.0.iter().position(|x| x == a).unwrap();
        let b = self.0.iter().position(|x| x == b).unwrap();
        self.0.swap(a, b);
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
    }

    fn spin(&mut self, amount: usize) {
        self.0.rotate_right(amount % 16);
    }

    fn process_one(&mut self, i: &Instruction) {
        match *i {
            Instruction::Spin(s) => self.spin(s),
            Instruction::Swap(i, j) => self.swap(i, j),
            Instruction::SwapP(a, b) => self.swap_p(&a, &b),
        }
    }

    fn process_many<'a>(&mut self, is: impl IntoIterator<Item = &'a Instruction>) {
        for i in is {
            self.process_one(i);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Spin(usize),
    Swap(usize, usize),
    SwapP(Program, Program),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (t, s) = s.split_at(1);
        match t {
            "s" => {
                let amount = s.parse()?;
                Ok(Self::Spin(amount))
            }
            "x" => {
                let (b, e) = s
                    .split_once('/')
                    .ok_or(anyhow!("x must be followed with '/'"))?;
                Ok(Self::Swap(b.parse()?, e.parse()?))
            }
            "p" => {
                let (b, e) = s
                    .split_once('/')
                    .ok_or(anyhow!("p must be followed with '/'"))?;
                Ok(Self::SwapP(b.parse()?, e.parse()?))
            }
            _ => Err(anyhow!("{t} is not a valid inst prefix")),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let instructions = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mut line_dance = LineDance::default();
    line_dance.process_many(&instructions);
    Some(format!("{line_dance}"))
}

pub fn part_two(input: &str) -> Option<String> {
    let instructions = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let mut line_dance = LineDance::default();
    let beginning = LineDance::default();
    let mut i = 0;
    let i = loop {
        line_dance.process_many(&instructions);
        i += 1;
        if line_dance == beginning {
            break i;
        }
    };
    for _ in 0..(1_000_000_000 % i) {
        line_dance.process_many(&instructions);
    }
    Some(format!("{line_dance}"))
}
