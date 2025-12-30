advent_of_code_2025::solution!(9);

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Point(usize, usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Red,
    Green,
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err("Invalid number of coordinates".to_string());
        }

        Ok(Point(
            parts[0]
                .parse()
                .map_err(|_| "Invalid x coordinate".to_string())?,
            parts[1]
                .parse()
                .map_err(|_| "Invalid y coordinate".to_string())?,
        ))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points = parse_red_points(input);

    // Keep track of the best pivots. A pivot is a candidate for the
    // top-left (TL), top-right (TR), bottom-left (BL), and bottom-right (BR)
    // corners.
    // This greatly reduces the number of computations we need because we can
    // find the pivots in O(n) time by doing a single-pass through the points.
    // After we do that, we know that we'll have a rectangle using either
    // a TLxBR pair or TRxBL pair, which we can easily find in a Cartesian
    // product.
    let mut tl_pivots: Vec<&Point> = Vec::new();
    let mut tr_pivots: Vec<&Point> = Vec::new();
    let mut bl_pivots: Vec<&Point> = Vec::new();
    let mut br_pivots: Vec<&Point> = Vec::new();

    // Do a single-pass through the points keeping track of the best pivots
    for point in points.iter() {
        // First, remove all points that are strictly worse than the current point by
        // keeping only the ones that are "better" in at least one of the axes.
        // Then add the current one if it is not strictly worse than any of the remaining
        // pivots.
        tl_pivots.retain(|p| p.0 < point.0 || p.1 < point.1);
        if !tl_pivots.iter().any(|p| p.0 <= point.0 && p.1 <= point.1) {
            tl_pivots.push(point);
        }

        tr_pivots.retain(|p| p.0 > point.0 || p.1 < point.1);
        if !tr_pivots.iter().any(|p| p.0 >= point.0 && p.1 <= point.1) {
            tr_pivots.push(point);
        }

        bl_pivots.retain(|p| p.0 < point.0 || p.1 > point.1);
        if !bl_pivots.iter().any(|p| p.0 <= point.0 && p.1 >= point.1) {
            bl_pivots.push(point);
        }

        br_pivots.retain(|p| p.0 > point.0 || p.1 > point.1);
        if !br_pivots.iter().any(|p| p.0 >= point.0 && p.1 >= point.1) {
            br_pivots.push(point);
        }
    }

    Some(
        tl_pivots
            .into_iter()
            .cartesian_product(br_pivots)
            .chain(tr_pivots.into_iter().cartesian_product(bl_pivots))
            .map(|(pivot1, pivot2)| calculate_area(pivot1, pivot2))
            .max()
            .unwrap_or(1),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let red_points = parse_red_points(input);

    // the grid can be thought-of as a `max_x` per `max_y` matrix; however, to optimize for
    // space, we'll use a sparse matrix representation by mapping pairs of indices to the tile
    let mut grid: HashMap<(usize, usize), Tile> = HashMap::new();

    // first paint red tiles
    for red_point in red_points.iter() {
        grid.insert((red_point.0 as usize, red_point.1 as usize), Tile::Red);
    }

    let (x_min, y_min, x_max, y_max) = min_x_min_y_max_x_max_y(&grid);

    // println!("Grid - painted red:");
    // print_grid(&grid);

    // green tiles rule 1: consecutive red tiles are connected by green tiles
    // green tiles rule 2: tiles inside the polygon are also green; to keep track
    // of that, we'll instead calculate the polygon's outter border. This can be
    // inferred from the polygon's directed edges. If the overall polygon is clockwise,
    // then:
    // - when we move right, the outter border is above the current edge;
    // - when we move left, the outter border is below the current edge;
    // - when we move up, the outter border is to the left of the current edge;
    // - when we move down, the outter border is to the right of the current edge;
    let is_cw = is_clockwise(&red_points);
    let mut outer_border: HashSet<(usize, usize)> = HashSet::new();

    for (origin, dest) in red_points.iter().circular_tuple_windows() {
        // a guarantee given by the problem is that consecutive red tiles are
        // either vertically or horizontally connected; so we'll either walk
        // up, down, left or right from `origin` to `dest`

        if origin.0 != dest.0 {
            // walk horizontally
            let y = origin.1; // == dest.1
            let x_start = origin.0.min(dest.0);
            let x_end = dest.0.max(origin.0);
            for x in (x_start + 1)..x_end {
                grid.insert((x as usize, y as usize), Tile::Green);
            }

            if is_cw && origin.0 < dest.0 {
                if y > y_min {
                    for x in x_start..=x_end {
                        outer_border.insert((x as usize, y - 1 as usize));
                    }
                }
            } else {
                if y < y_max {
                    for x in x_start..=x_end {
                        outer_border.insert((x as usize, y + 1 as usize));
                    }
                }
            }
        } else {
            // walk vertically
            let x = origin.0; // == dest.0
            let y_start = origin.1.min(dest.1);
            let y_end = dest.1.max(origin.1);
            for y in (y_start + 1)..y_end {
                grid.insert((x as usize, y as usize), Tile::Green);
            }

            if is_cw && origin.1 < dest.1 {
                if x < x_max {
                    for y in y_start..=y_end {
                        outer_border.insert((x + 1 as usize, y as usize));
                    }
                }
            } else {
                if x > x_min {
                    for y in y_start..=y_end {
                        outer_border.insert((x - 1 as usize, y as usize));
                    }
                }
            }
        }
    }

    // println!("Grid - painted green walls:");
    // print_grid(&grid);

    // some walls might have been accidentally labeled as an outer border if
    // we have two walls close enough - so remove them now
    outer_border.retain(|point| !grid.contains_key(point));

    // println!("Grid - with border:");
    // print_grid_with_border(&grid, &outer_border);

    // now let's calculate the maximum area
    // but skipping the rectangles that contain any outter point

    let mut max_area: u64 = 0;

    for i in 0..red_points.len() {
        for j in (i + 1)..red_points.len() {
            let origin = &red_points[i];
            let dest = &red_points[j];

            let x_start = origin.0.min(dest.0);
            let x_end = dest.0.max(origin.0);
            let y_start = origin.1.min(dest.1);
            let y_end = dest.1.max(origin.1);

            if outer_border
                .iter()
                .any(|&(x, y)| x_start <= x && x <= x_end && y_start <= y && y <= y_end)
            {
                continue;
            }

            max_area = max_area.max(calculate_area(origin, dest));
        }
    }

    Some(max_area)
}

fn parse_red_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(Point::try_from)
        .collect::<Result<Vec<Point>, _>>()
        .expect("Failed to parse points")
}

fn calculate_area(origin: &Point, dest: &Point) -> u64 {
    ((origin.0 as i64 - dest.0 as i64).abs() + 1) as u64
        * ((origin.1 as i64 - dest.1 as i64).abs() + 1) as u64
}

fn min_x_min_y_max_x_max_y(grid: &HashMap<(usize, usize), Tile>) -> (usize, usize, usize, usize) {
    let (min_x, max_x) = grid
        .keys()
        .map(|&(x, _)| x)
        .minmax()
        .into_option()
        .unwrap_or((0, 0));
    let (min_y, max_y) = grid
        .keys()
        .map(|&(_, y)| y)
        .minmax()
        .into_option()
        .unwrap_or((0, 0));
    (min_x, min_y, max_x, max_y)
}

// fn print_grid(grid: &HashMap<(usize, usize), Tile>) {
//     let (min_x, min_y, max_x, max_y) = min_x_min_y_max_x_max_y(&grid);

//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             match grid.get(&(x, y)) {
//                 None => print!("."),
//                 Some(Tile::Red) => print!("R"),
//                 Some(Tile::Green) => print!("G"),
//             }
//         }
//         println!();
//     }
// }

// fn print_grid_with_border(grid: &HashMap<(usize, usize), Tile>, border: &HashSet<(usize, usize)>) {
//     let (min_x, min_y, max_x, max_y) = min_x_min_y_max_x_max_y(&grid);

//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             match (grid.get(&(x, y)), border.contains(&(x, y))) {
//                 (None, true) => print!("|"),
//                 (None, false) => print!("."),
//                 (Some(Tile::Red), false) => print!("R"),
//                 (Some(Tile::Green), false) => print!("G"),
//                 _ => panic!("Unexpected tile type"),
//             }
//         }
//         println!();
//     }
// }

fn is_clockwise(poly: &[Point]) -> bool {
    poly.iter()
        .circular_tuple_windows()
        .map(|(origin, dest)| (dest.0 as i64 - origin.0 as i64) * (dest.1 as i64 + origin.1 as i64))
        .sum::<i64>()
        < 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
