use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

advent_of_code::solution!(7);

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProgramTree<'a> {
    tree: HashMap<Node<'a>, Vec<&'a str>>,
    leaves: Vec<Node<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node<'a> {
    label: &'a str,
    weight: u32,
}

impl<'a> Node<'a> {
    fn from_str(s: &'a str) -> anyhow::Result<Self> {
        let (label, weight) = s.split_once(' ').ok_or(anyhow!("{s} not a valid node"))?;
        let weight = weight
            .trim_end_matches(')')
            .trim_start_matches('(')
            .parse()?;
        Ok(Self { label, weight })
    }
}

impl<'a> ProgramTree<'a> {
    fn from_str(s: &'a str) -> anyhow::Result<Self> {
        let mut leaves = Vec::new();
        let mut tree = HashMap::new();
        for line in s.lines().map(|l| l.trim()) {
            if let Some((node, others)) = line.split_once(" -> ") {
                let others = others.split(", ").collect();
                let node = Node::from_str(node)?;
                tree.insert(node, others);
            } else {
                let node = Node::from_str(line.trim())?;
                leaves.push(node)
            }
        }
        Ok(Self { tree, leaves })
    }

    fn root(&self) -> &str {
        let mut node = self.leaves.first().unwrap();
        while let Some((n, _)) = self.tree.iter().find(|(_, v)| v.contains(&node.label)) {
            node = n;
        }
        node.label
    }

    fn parent_n(&self, label: &str) -> Option<&Node<'a>> {
        self.tree
            .iter()
            .find_map(|(n, v)| v.contains(&label).then_some(n))
    }

    fn brethren(&'a self, label: &'a str) -> Box<dyn Iterator<Item = &'a str> + '_> {
        if let Some(parent) = self.parent_n(label) {
            Box::new(
                self.tree
                    .get(parent)
                    .unwrap()
                    .iter()
                    .filter(move |&&b| b != label)
                    .copied(),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }

    fn node(&self, label: &str) -> Option<&Node<'a>> {
        self.tree.keys().find(|k| k.label == label)
    }

    fn balance(&mut self) -> u32 {
        let mut weights = HashMap::new();
        for leaf in &self.leaves {
            weights.insert(leaf.label, leaf.weight);
        }
        let mut all_nodes: HashSet<_> = self.tree.keys().collect();
        loop {
            let mut new = HashSet::new();
            for node in &all_nodes {
                if let Some(weight) = weights.get(node.label) {
                    if self
                        .brethren(node.label)
                        .all(|sibling| weights.get(sibling).map(|w| w != weight).unwrap_or(false))
                        && self.brethren(node.label).any(|sibling| {
                            weights.get(sibling).map(|w| w != weight).unwrap_or(false)
                        })
                    {
                        let broth = self.brethren(node.label).next().unwrap();
                        let bweight = weights[broth];
                        match weight.cmp(&bweight) {
                            std::cmp::Ordering::Less => {
                                return self.node(node.label).unwrap().weight + weight - bweight
                            }
                            std::cmp::Ordering::Equal => unreachable!(),
                            std::cmp::Ordering::Greater => {
                                return self.node(node.label).unwrap().weight + bweight - weight
                            }
                        }
                    }
                } else if let Some(c) = self.tree[node].first().and_then(|n| weights.get(n)) {
                    weights.insert(node.label, node.weight + c * self.tree[node].len() as u32);
                    new.insert(*node);
                }
            }
            all_nodes = new;
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let tree = ProgramTree::from_str(input).unwrap();
    Some(tree.root().into())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut tree = ProgramTree::from_str(input).unwrap();
    Some(tree.balance())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("tknk".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(60));
    }
}
