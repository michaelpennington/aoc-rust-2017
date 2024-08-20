use std::{collections::HashMap, fmt::Display, ops::Index, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(21);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pixel {
    On,
    Off,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pixel::On => '#',
            Pixel::Off => '.',
        };
        write!(f, "{c}")
    }
}

impl TryFrom<char> for Pixel {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Off),
            '#' => Ok(Self::On),
            _ => Err(anyhow!("{value} is not a valid pixel")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TwoSquare {
    ps: [[Pixel; 2]; 2],
}

impl FromStr for TwoSquare {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.split('/');
        let mut row1 = pts.next().unwrap().chars();
        let mut row2 = pts.next().unwrap().chars();
        let row1 = [
            row1.next().unwrap().try_into()?,
            row1.next().unwrap().try_into()?,
        ];
        let row2 = [
            row2.next().unwrap().try_into()?,
            row2.next().unwrap().try_into()?,
        ];
        Ok(Self { ps: [row1, row2] })
    }
}

impl Index<(usize, usize)> for TwoSquare {
    type Output = Pixel;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.ps[index.1][index.0]
    }
}

impl TwoSquare {
    fn count(&self) -> usize {
        self.ps
            .iter()
            .flat_map(|p| p.iter().filter(|&s| *s == Pixel::On))
            .count()
    }

    fn flip_x(&self) -> Self {
        Self {
            ps: [[self[(1, 0)], self[(0, 0)]], [self[(1, 1)], self[(0, 1)]]],
        }
    }

    fn rotate(&self) -> Self {
        Self {
            ps: [[self[(0, 1)], self[(0, 0)]], [self[(1, 1)], self[(1, 0)]]],
        }
    }

    fn all_perms(&self) -> [Self; 8] {
        [
            *self,
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            self.flip_x(),
            self.rotate().flip_x(),
            self.rotate().rotate().flip_x(),
            self.rotate().rotate().rotate().flip_x(),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ThreeSquare {
    ps: [[Pixel; 3]; 3],
}

impl FromStr for ThreeSquare {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.split('/');
        let (mut row1, mut row2, mut row3) = (
            pts.next().unwrap().chars(),
            pts.next().unwrap().chars(),
            pts.next().unwrap().chars(),
        );
        let (row1, row2, row3) = (
            [
                row1.next().unwrap().try_into()?,
                row1.next().unwrap().try_into()?,
                row1.next().unwrap().try_into()?,
            ],
            [
                row2.next().unwrap().try_into()?,
                row2.next().unwrap().try_into()?,
                row2.next().unwrap().try_into()?,
            ],
            [
                row3.next().unwrap().try_into()?,
                row3.next().unwrap().try_into()?,
                row3.next().unwrap().try_into()?,
            ],
        );
        Ok(Self {
            ps: [row1, row2, row3],
        })
    }
}

impl ThreeSquare {
    fn count(&self) -> usize {
        self.ps
            .iter()
            .flat_map(|p| p.iter().filter(|&s| *s == Pixel::On))
            .count()
    }

    fn flip_x(&self) -> Self {
        Self {
            ps: [
                [self[(2, 0)], self[(1, 0)], self[(0, 0)]],
                [self[(2, 1)], self[(1, 1)], self[(0, 1)]],
                [self[(2, 2)], self[(1, 2)], self[(0, 2)]],
            ],
        }
    }

    fn rotate(&self) -> Self {
        Self {
            ps: [
                [self[(0, 2)], self[(0, 1)], self[(0, 0)]],
                [self[(1, 2)], self[(1, 1)], self[(1, 0)]],
                [self[(2, 2)], self[(2, 1)], self[(2, 0)]],
            ],
        }
    }

    fn all_perms(&self) -> [Self; 8] {
        [
            *self,
            self.rotate(),
            self.rotate().rotate(),
            self.rotate().rotate().rotate(),
            self.flip_x(),
            self.rotate().flip_x(),
            self.rotate().rotate().flip_x(),
            self.rotate().rotate().rotate().flip_x(),
        ]
    }
}

impl Index<(usize, usize)> for ThreeSquare {
    type Output = Pixel;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.ps[index.1][index.0]
    }
}

const START_SQUARE: ThreeSquare = ThreeSquare {
    ps: [
        [Pixel::Off, Pixel::On, Pixel::Off],
        [Pixel::Off, Pixel::Off, Pixel::On],
        [Pixel::On, Pixel::On, Pixel::On],
    ],
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ThreeArt {
    square: Vec<Vec<ThreeSquare>>,
}

impl Display for ThreeArt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.square {
            for j in 0..3 {
                for s in row {
                    for i in 0..3 {
                        write!(f, "{}", s[(i, j)])?;
                    }
                    write!(f, "|")?;
                }
                writeln!(f)?;
            }
            writeln!(f, "---------------------")?;
        }
        Ok(())
    }
}

impl From<ThreeSquare> for ThreeArt {
    fn from(value: ThreeSquare) -> Self {
        Self {
            square: vec![vec![value]],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TwoArt {
    square: Vec<Vec<TwoSquare>>,
}

impl Display for TwoArt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.square {
            for j in 0..2 {
                for s in row {
                    for i in 0..2 {
                        write!(f, "{}", s[(i, j)])?;
                    }
                    write!(f, "|")?;
                }
                writeln!(f)?;
            }
            writeln!(f, "---------------------")?;
        }
        Ok(())
    }
}

impl TwoArt {
    fn transform(&self, map: &Map2) -> ThreeArt {
        let square = self
            .square
            .iter()
            .map(|row| row.iter().map(|s| map.transform(*s)).collect())
            .collect();
        ThreeArt { square }
    }

    fn count(&self) -> usize {
        self.square
            .iter()
            .flat_map(|r| r.iter().map(|s| s.count()))
            .sum()
    }
}

impl ThreeArt {
    fn transform(&self, map: &Map3) -> TwoArt {
        let mut square = Vec::new();
        for row in &self.square {
            let mut row1 = Vec::new();
            let mut row2 = Vec::new();
            for s in row {
                let fourtwo: [[TwoSquare; 2]; 2] = map.transform(*s).into();
                row1.push(fourtwo[0][0]);
                row1.push(fourtwo[0][1]);
                row2.push(fourtwo[1][0]);
                row2.push(fourtwo[1][1]);
            }
            square.push(row1);
            square.push(row2);
        }
        TwoArt { square }
    }

    fn divisible_by_two(&self) -> bool {
        self.square.len() % 2 == 0
    }

    fn to_two_art(&self) -> TwoArt {
        if !self.divisible_by_two() {
            panic!("Must be divisible by two to convert to two_art");
        }
        let mut holder = Vec::new();
        for row in &self.square {
            let mut new_row1 = Vec::new();
            let mut new_row2 = Vec::new();
            let mut new_row3 = Vec::new();
            for s in row {
                new_row1.push(s[(0, 0)]);
                new_row1.push(s[(1, 0)]);
                new_row1.push(s[(2, 0)]);
                new_row2.push(s[(0, 1)]);
                new_row2.push(s[(1, 1)]);
                new_row2.push(s[(2, 1)]);
                new_row3.push(s[(0, 2)]);
                new_row3.push(s[(1, 2)]);
                new_row3.push(s[(2, 2)]);
            }
            holder.push(new_row1);
            holder.push(new_row2);
            holder.push(new_row3);
        }
        let mut out = TwoArt { square: Vec::new() };
        for c in holder.chunks(2) {
            let mut row = Vec::new();
            for (r1, r2) in c[0].chunks(2).zip(c[1].chunks(2)) {
                row.push(TwoSquare {
                    ps: [[r1[0], r1[1]], [r2[0], r2[1]]],
                })
            }
            out.square.push(row);
        }
        out
    }

    fn count(&self) -> usize {
        self.square
            .iter()
            .flat_map(|r| r.iter().map(|s| s.count()))
            .sum()
    }
}

enum Art {
    Two(TwoArt),
    Three(ThreeArt),
}

impl Art {
    fn reduce(&mut self) {
        match self {
            Art::Two(_) => {}
            Art::Three(t) => {
                if t.divisible_by_two() {
                    *self = Self::Two(t.to_two_art());
                }
            }
        }
    }

    fn transform(&mut self, map2: &Map2, map3: &Map3) {
        self.reduce();
        match self {
            Art::Two(tw) => *self = Art::Three(tw.transform(map2)),
            Art::Three(th) => *self = Art::Two(th.transform(map3)),
        }
    }

    fn count(&self) -> usize {
        match self {
            Art::Two(r) => r.count(),
            Art::Three(r) => r.count(),
        }
    }
}

impl From<ThreeArt> for Art {
    fn from(value: ThreeArt) -> Self {
        Self::Three(value)
    }
}

impl From<TwoArt> for Art {
    fn from(value: TwoArt) -> Self {
        Self::Two(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Map2 {
    mappings: HashMap<TwoSquare, ThreeSquare>,
}

impl Map2 {
    fn add_new(&mut self, s: &str) -> anyhow::Result<()> {
        let err = || anyhow!("Malformed string for Map2: {s}");
        let (p1, p2) = s.split_once(" => ").ok_or(err())?;
        let s1: TwoSquare = p1.parse()?;
        let s2 = p2.parse()?;
        for perm in s1.all_perms() {
            self.mappings.insert(perm, s2);
        }

        Ok(())
    }

    fn transform(&self, art: TwoSquare) -> ThreeSquare {
        self.mappings[&art]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FourTwoSquare([[TwoSquare; 2]; 2]);

impl From<FourTwoSquare> for [[TwoSquare; 2]; 2] {
    fn from(value: FourTwoSquare) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Map3 {
    mappings: HashMap<ThreeSquare, FourTwoSquare>,
}

impl FromStr for FourTwoSquare {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.split('/');
        let (r1, r2, r3, r4) = (
            pts.next().unwrap().chars().collect::<Vec<_>>(),
            pts.next().unwrap().chars().collect::<Vec<_>>(),
            pts.next().unwrap().chars().collect::<Vec<_>>(),
            pts.next().unwrap().chars().collect::<Vec<_>>(),
        );
        let s1 = TwoSquare {
            ps: [
                [r1[0].try_into()?, r1[1].try_into()?],
                [r2[0].try_into()?, r2[1].try_into()?],
            ],
        };
        let s2 = TwoSquare {
            ps: [
                [r1[2].try_into()?, r1[3].try_into()?],
                [r2[2].try_into()?, r2[3].try_into()?],
            ],
        };
        let s3 = TwoSquare {
            ps: [
                [r3[0].try_into()?, r3[1].try_into()?],
                [r4[0].try_into()?, r4[1].try_into()?],
            ],
        };
        let s4 = TwoSquare {
            ps: [
                [r3[2].try_into()?, r3[3].try_into()?],
                [r4[2].try_into()?, r4[3].try_into()?],
            ],
        };
        Ok(Self([[s1, s2], [s3, s4]]))
    }
}

impl Map3 {
    fn add_new(&mut self, s: &str) -> anyhow::Result<()> {
        let err = || anyhow!("Malformed string for Map3: {s}");
        let (p1, p2) = s.split_once(" => ").ok_or(err())?;
        let s1: ThreeSquare = p1.parse()?;
        let s2 = p2.parse()?;
        for perm in s1.all_perms() {
            self.mappings.insert(perm, s2);
        }
        Ok(())
    }

    fn transform(&self, art: ThreeSquare) -> FourTwoSquare {
        self.mappings[&art]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let num_times: usize = lines.next().unwrap().parse().unwrap();
    let mut m2 = Map2::default();
    let mut m3 = Map3::default();
    for line in lines {
        if line.len() < 25 {
            m2.add_new(line).unwrap();
        } else {
            m3.add_new(line).unwrap();
        }
    }
    let mut art = Art::from(ThreeArt::from(START_SQUARE));
    for _ in 0..num_times {
        art.transform(&m2, &m3);
    }
    Some(art.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().skip(1);
    let mut m2 = Map2::default();
    let mut m3 = Map3::default();
    for line in lines {
        if line.len() < 25 {
            m2.add_new(line).unwrap();
        } else {
            m3.add_new(line).unwrap();
        }
    }
    let mut art = Art::from(ThreeArt::from(START_SQUARE));
    for _ in 0..18 {
        art.transform(&m2, &m3);
    }
    Some(art.count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
