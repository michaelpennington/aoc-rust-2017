use std::{
    collections::HashSet,
    fmt::{Display, LowerHex},
    ops::{Index, IndexMut},
};

advent_of_code::solution!(14);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct SkipList {
    list: [u8; 256],
    pos: usize,
    skip_size: usize,
}

impl Default for SkipList {
    fn default() -> Self {
        let mut list = [0; 256];
        for i in 0..=255u8 {
            list[i as usize] = i;
        }
        Self {
            list,
            pos: 0,
            skip_size: 0,
        }
    }
}

impl Index<usize> for SkipList {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index % 256]
    }
}

impl IndexMut<usize> for SkipList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.list[index % 256]
    }
}

impl SkipList {
    fn reverse(&mut self, start: usize, end: usize) {
        let elements = (start..end).map(|i| self[i]).collect::<Vec<_>>();
        for (i, j) in (start..end).zip((0..elements.len()).map(|j| elements.len() - j - 1)) {
            self[i] = elements[j];
        }
    }

    fn rotate(&mut self, length: usize) {
        self.reverse(self.pos, self.pos + length);
        self.pos += length + self.skip_size;
        self.skip_size += 1;
    }

    fn from_bytes(bytes: &mut Vec<u8>) -> Self {
        let mut out = Self::default();
        bytes.extend_from_slice(&[17, 31, 73, 47, 23]);
        for _ in 0..64 {
            for &b in &*bytes {
                out.rotate(b as usize)
            }
        }
        out
    }

    fn finish(self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        for i in 0..16 {
            bytes[i] = (0..16)
                .map(|j| self[16 * i + j])
                .reduce(|acc, e| acc ^ e)
                .unwrap();
        }
        bytes
    }

    fn count_ones(self) -> u32 {
        let done = self.finish();
        done.iter().map(|n| n.count_ones()).sum()
    }
}

impl LowerHex for SkipList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..16 {
            let num = (0..16)
                .map(|j| self[16 * i + j])
                .reduce(|acc, e| acc ^ e)
                .unwrap();
            write!(f, "{num:02x}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Square {
    Used,
    Empty,
}

impl Square {
    fn byte_to(b: [u8; 16]) -> [Self; 128] {
        let mut out = [Self::Empty; 128];
        for i in 0..16 {
            let mut b = b[i];
            for j in 0..8 {
                out[i * 8 + j] = if b & 0x80 != 0 {
                    Square::Used
                } else {
                    Square::Empty
                };
                b <<= 1;
            }
        }
        out
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Map {
    map: [[Square; 128]; 128],
}

impl Index<(usize, usize)> for Map {
    type Output = Square;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.map[index.1][index.0]
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Square::Used => 'X',
            Square::Empty => '.',
        };
        write!(f, "{c}")
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.map {
            for i in line {
                write!(f, "{i}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn from_bytes(bytes: [[u8; 16]; 128]) -> Self {
        let mut map = [[Square::Empty; 128]; 128];
        for (bs, m) in bytes.into_iter().zip(map.iter_mut()) {
            *m = Square::byte_to(bs);
        }
        Self { map }
    }

    fn neighbors(&self, pt: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        (pt.0.saturating_sub(1)..=(pt.0 + 1).min(127))
            .flat_map(move |i| (pt.1.saturating_sub(1)..=(pt.1 + 1).min(127)).map(move |j| (i, j)))
            .filter(move |&(i, j)| (i == pt.0 && j != pt.1) || (i != pt.0 && j == pt.1))
    }

    fn find_region(
        self,
        start: (usize, usize),
        region: &mut HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut pts = Vec::new();
        for n in self.neighbors(start).filter(|&n| self[n] == Square::Used) {
            if region.contains(&n) {
                continue;
            }
            region.insert(n);
            pts.push(n);
        }
        pts
    }

    fn num_regions(&self) -> u32 {
        let mut already_processed: HashSet<(usize, usize)> = HashSet::new();
        let mut n = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map.len() {
                if already_processed.contains(&(i, j)) {
                    continue;
                }
                if self[(i, j)] == Square::Used {
                    let mut new_pts = HashSet::new();
                    let mut pts_to_try = vec![(i, j)];
                    while !pts_to_try.is_empty() {
                        let mut new_pts_to_try = Vec::new();
                        for pt in &pts_to_try {
                            let pts = self.find_region(*pt, &mut new_pts);
                            new_pts_to_try.extend_from_slice(&pts);
                        }
                        pts_to_try = new_pts_to_try;
                    }
                    already_processed.extend(new_pts);
                    n += 1;
                }
            }
        }
        n
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.trim();
    let mut total = 0;
    for i in 0..=127 {
        let bs = format!("{input}-{i}");
        let mut bytes = Vec::from(bs.as_bytes());
        let hash = SkipList::from_bytes(&mut bytes);
        total += hash.count_ones();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.trim();
    let bytes = (0..=127)
        .map(|i| {
            let bs = format!("{input}-{i}");
            let mut bytes = Vec::from(bs.as_bytes());
            let hash = SkipList::from_bytes(&mut bytes);
            hash.finish()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let map = Map::from_bytes(bytes);
    Some(map.num_regions())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8108));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1242));
    }
}
