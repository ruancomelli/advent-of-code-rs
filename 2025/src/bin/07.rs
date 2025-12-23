advent_of_code_2025::solution!(7);

use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
enum Node {
    Empty,
    Beam(usize),
    Splitter,
    Source(usize),
}

impl TryFrom<char> for Node {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Node::Empty),
            '|' => Ok(Node::Beam(1)),
            '^' => Ok(Node::Splitter),
            'S' => Ok(Node::Source(1)),
            other => Err(format!("Invalid node {:?}", other)),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => write!(f, "."),
            Node::Beam(_) => write!(f, "|"),
            Node::Splitter => write!(f, "^"),
            Node::Source(_) => write!(f, "S"),
        }
    }
}

#[derive(Clone)]
struct Diagram(Vec<Vec<Node>>);

impl Diagram {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, i: usize, j: usize) -> Option<&Node> {
        self.0.get(i)?.get(j)
    }

    fn evolve_node_mut(&mut self, i: usize, j: usize, value: Node) {
        self.0[i][j] = value;
    }

    fn evolve_step_mut(&mut self, i: usize, j: usize) {
        let prev_i = (i as i32 - 1) as usize;
        if self.accepts_beams(i, j) {
            // Beams move downward
            // Also, sources produce downwad beams
            if let Some(Node::Beam(count) | Node::Source(count)) = self.get(prev_i, j) {
                self.add_beams(i, j, *count);
            }

            // Also nodes to the side of splitters become beams
            // if the beams are activated/receive beams
            if j > 0
                && let Some(count) = self.maybe_activated_splitter(i, j - 1)
            {
                self.add_beams(i, j, count);
            }
            if j < self.0[i].len() - 1
                && let Some(count) = self.maybe_activated_splitter(i, j + 1)
            {
                self.add_beams(i, j, count);
            }
        }
    }

    fn evolve_full(&self) -> Self {
        let mut new_diagram = self.clone();
        new_diagram.evolve_full_mut();
        new_diagram
    }

    fn evolve_full_mut(&mut self) {
        for i in 1..self.rows() {
            for j in 0..self.cols() {
                self.evolve_step_mut(i, j);
            }
        }
    }

    fn add_beams(&mut self, i: usize, j: usize, count: usize) {
        match self.get(i, j) {
            Some(Node::Empty) => self.evolve_node_mut(i, j, Node::Beam(count)),
            Some(Node::Beam(existing_count)) => {
                self.evolve_node_mut(i, j, Node::Beam(existing_count + count))
            }
            other => panic!("Cannot add beams to non-empty node {:?}", other),
        }
    }

    fn accepts_beams(&self, i: usize, j: usize) -> bool {
        matches!(self.get(i, j), Some(Node::Empty | Node::Beam(_)))
    }

    fn maybe_activated_splitter(&self, i: usize, j: usize) -> Option<usize> {
        if i > 0 {
            match (self.get(i, j), self.get(i - 1, j)) {
                (Some(Node::Splitter), Some(Node::Beam(count) | Node::Source(count))) => {
                    Some(*count)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Diagram {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(Diagram(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(Node::try_from)
                        .collect::<Result<Vec<Node>, _>>()
                })
                .collect::<Result<Vec<Vec<Node>>, _>>()?,
        ))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let diagram = Diagram::try_from(input)
        .expect("Failed to parse diagram")
        .evolve_full();

    // Count how many times beams were split
    // by counting how many splitters are activated
    Some(
        (0..diagram.rows())
            .into_iter()
            .map(|i| {
                (0..diagram.cols())
                    .into_iter()
                    .filter(|&j| diagram.maybe_activated_splitter(i, j).is_some())
                    .count() as u64
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let diagram = Diagram::try_from(input)
        .expect("Failed to parse diagram")
        .evolve_full();

    let last_row = diagram
        .0
        .last()
        .expect("Failed to get last row - diagram is empty?");

    // Count how many times beams were split
    // by counting how many splitters are activated
    Some(
        last_row
            .into_iter()
            .map(|node| match node {
                Node::Beam(count) => *count as u64,
                _ => 0,
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
