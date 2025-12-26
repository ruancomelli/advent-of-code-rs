use std::collections::{HashMap, HashSet};

advent_of_code_2025::solution!(8);

const LARGEST_CIRCUITS_TO_MULTIPLY: usize = 3;

#[derive(Debug)]
struct Point(u64, u64, u64);

impl Point {
    fn squared_distance_to(&self, other: &Point) -> u64 {
        ((self.0 as i64 - other.0 as i64).pow(2)
            + (self.1 as i64 - other.1 as i64).pow(2)
            + (self.2 as i64 - other.2 as i64).pow(2)) as u64
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 3 {
            return Err("Invalid number of coordinates".to_string());
        }

        Ok(Point(
            parts[0]
                .parse()
                .map_err(|_| "Invalid x coordinate".to_string())?,
            parts[1]
                .parse()
                .map_err(|_| "Invalid y coordinate".to_string())?,
            parts[2]
                .parse()
                .map_err(|_| "Invalid z coordinate".to_string())?,
        ))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    _part_one(input, 1000)
}

pub fn _part_one(input: &str, max_connections: usize) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .map(Point::try_from)
        .collect::<Result<Vec<Point>, String>>()
        .unwrap();

    let mut sorted_pairs: Vec<(usize, usize, u64)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            sorted_pairs.push((i, j, points[i].squared_distance_to(&points[j])));
        }
    }

    sorted_pairs.sort_unstable_by_key(|(_, _, dist)| *dist);

    // each joint box has its own circuit to begin with
    let mut circuits: HashMap<usize, usize> = (0..points.len()).map(|i| (i, i)).collect();

    for (lhs, rhs, _) in sorted_pairs.into_iter().take(max_connections) {
        // find the next closest pair of points
        // note that `lhs` is always `< rhs` thanks to how we constructed `sorted_pairs`
        let lhs_circuit = circuits.get(&lhs).expect("lhs not found in circuits");
        let rhs_circuit = circuits.get(&rhs).expect("rhs not found in circuits");

        // only proceed if we haven't already connected the sets
        if lhs_circuit == rhs_circuit {
            continue;
        }

        // by convention, use the smallest point ID as the merged circuit ID
        let merged_circuit = *lhs_circuit.min(rhs_circuit);
        // and all points in the other circuit need to be assigned to the new one
        let circuit_to_merge = *lhs_circuit.max(rhs_circuit);

        for circuit in circuits.values_mut() {
            if *circuit == circuit_to_merge {
                *circuit = merged_circuit;
            }
        }
    }

    let mut circuit_size_counter = HashMap::new();
    for circuit in circuits.values() {
        *circuit_size_counter.entry(*circuit).or_insert(0) += 1;
    }

    let mut sizes = circuit_size_counter.values().collect::<Vec<_>>();
    sizes.sort_unstable();
    let size_product: u64 = sizes
        .into_iter()
        .rev()
        .take(LARGEST_CIRCUITS_TO_MULTIPLY)
        .product();

    Some(size_product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let points: Vec<Point> = input
        .lines()
        .map(Point::try_from)
        .collect::<Result<Vec<Point>, String>>()
        .unwrap();

    let mut sorted_pairs: Vec<(usize, usize, u64)> = vec![];

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            sorted_pairs.push((i, j, points[i].squared_distance_to(&points[j])));
        }
    }

    sorted_pairs.sort_unstable_by_key(|(_, _, dist)| *dist);

    // each joint box has its own circuit to begin with
    let mut circuits: HashMap<usize, usize> = (0..points.len()).map(|i| (i, i)).collect();
    let mut last_two: Option<(usize, usize)> = None;

    for (lhs, rhs, _) in sorted_pairs {
        if circuits.values().collect::<HashSet<_>>().len() == 1 {
            // we've connected all joint boxes in a single circuit!
            break;
        }

        // find the next closest pair of points
        // note that `lhs` is always `< rhs` thanks to how we constructed `sorted_pairs`
        let lhs_circuit = circuits.get(&lhs).expect("lhs not found in circuits");
        let rhs_circuit = circuits.get(&rhs).expect("rhs not found in circuits");

        // only proceed if we haven't already connected the sets
        if lhs_circuit == rhs_circuit {
            continue;
        }

        // by convention, use the smallest point ID as the merged circuit ID
        let merged_circuit = *lhs_circuit.min(rhs_circuit);
        // and all points in the other circuit need to be assigned to the new one
        let circuit_to_merge = *lhs_circuit.max(rhs_circuit);

        for circuit in circuits.values_mut() {
            if *circuit == circuit_to_merge {
                *circuit = merged_circuit;
            }
        }

        last_two = Some((lhs, rhs));
    }

    let (lhs, rhs) = last_two.expect("'last two' is empty");
    let (lhs, rhs) = (&points[lhs], &points[rhs]);

    Some(lhs.0 * rhs.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code_2025::template::read_file("examples", DAY),
            10,
        );
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
