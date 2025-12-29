advent_of_code_2025::solution!(9);

use itertools::Itertools;

#[derive(Debug)]
struct Point(u64, u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Red,
    Green,
    None,
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

    let mut max_area: Option<u64> = None;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = (points[i].0 as i64 - points[j].0 as i64 + 1).abs() as u64
                * (points[i].1 as i64 - points[j].1 as i64 + 1).abs() as u64;

            max_area = match max_area {
                None => Some(area),
                Some(max) if area > max => Some(area),
                _ => max_area,
            };
        }
    }

    max_area
}

pub fn part_two(input: &str) -> Option<u64> {
    let red_points = parse_red_points(input);
    let max_x = red_points.iter().map(|p| p.0).max().unwrap();
    let max_y = red_points.iter().map(|p| p.1).max().unwrap();

    let mut grid = vec![vec![Tile::None; max_x as usize + 1]; max_y as usize + 1];

    // println!("Grid - all empty:");
    // print_grid(&grid);

    // first paint red tiles
    for point in red_points.iter() {
        grid[point.1 as usize][point.0 as usize] = Tile::Red;
    }

    // println!("Grid - painted red:");
    // print_grid(&grid);

    // green tiles rule 1: consecutive red tiles are connected by green tiles
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
                grid[y as usize][x as usize] = Tile::Green;
            }
        } else {
            // walk vertically
            let x = origin.0; // == dest.0
            let y_start = origin.1.min(dest.1);
            let y_end = dest.1.max(origin.1);
            for y in (y_start + 1)..y_end {
                grid[y as usize][x as usize] = Tile::Green;
            }
        }
    }

    // println!("Grid - painted green 1:");
    // print_grid(&grid);

    // green tiles rule 2: all tiles inside the region are also green
    // let's use an exterior flood fill strategy:
    // - all points in the grid start as "unknown" - represented by `None`
    // - scan through all boundary points in the grid; they are all "potentially"
    //   outside
    // - since the region is a closed loop, we know there are no holes in it: the
    //   region's interior points are all connected
    // - all non-wall boundary points start as "outside" (`Some(true)`)
    // - add those point's neighbors to the list of potentially outside
    // - if one of those points is a vertex or a wall, remove it from the list
    //   of potentially outside

    let mut outside_points: Vec<Vec<Option<bool>>> = vec![vec![None; grid[0].len()]; grid.len()];
    let mut potentially_outside: Vec<(u64, u64)> = (0..=max_x)
        .map(|x| (x, 0))
        .chain((0..=max_x).map(|x| (x, max_y)))
        .chain((0..=max_y).map(|y| (0, y)))
        .chain((0..=max_y).map(|y| (max_x, y)))
        .collect();

    while let Some((x, y)) = potentially_outside.pop() {
        // already solved this one
        if outside_points[y as usize][x as usize].is_some() {
            continue;
        }
        // check if it's known to be an interior point
        if grid[y as usize][x as usize] != Tile::None {
            outside_points[y as usize][x as usize] = Some(false);
            continue;
        }
        // so it must be an exterior point
        outside_points[y as usize][x as usize] = Some(true);
        // now add the neighbors
        if x > 0 {
            potentially_outside.push((x - 1, y));
        }
        if x < max_x {
            potentially_outside.push((x + 1, y));
        }
        if y > 0 {
            potentially_outside.push((x, y - 1));
        }
        if y < max_y {
            potentially_outside.push((x, y + 1));
        }
    }
    // points not visited in `outside_points` must be interior points
    for y in 0..=max_y {
        for x in 0..=max_x {
            if outside_points[y as usize][x as usize].is_none() {
                outside_points[y as usize][x as usize] = Some(false);
            }
        }
    }

    // now update the points in the grid: `None` becomes green if
    // `outside_points` is `Some(false)` ("surely interior")
    for y in 0..=max_y {
        for x in 0..=max_x {
            if grid[y as usize][x as usize] == Tile::None
                && outside_points[y as usize][x as usize] == Some(false)
            {
                grid[y as usize][x as usize] = Tile::Green;
            }
        }
    }

    // println!("Grid - painted green 2:");
    // print_grid(&grid);

    // now let's calculate the maximum area
    // but skipping the rectangles that contain any outter point

    let mut max_area: Option<u64> = None;

    for i in 0..red_points.len() {
        for j in (i + 1)..red_points.len() {
            let origin = &red_points[i];
            let dest = &red_points[j];

            let x_start = origin.0.min(dest.0);
            let x_end = dest.0.max(origin.0);
            let y_start = origin.1.min(dest.1);
            let y_end = dest.1.max(origin.1);

            if (x_start..=x_end).any(|x| {
                (y_start..=y_end).any(|y| outside_points[y as usize][x as usize] == Some(true))
            }) {
                // an exterior point was found inside this rectangle - skip it!
                continue;
            }

            let area = (origin.0 as i64 - dest.0 as i64 + 1).abs() as u64
                * (origin.1 as i64 - dest.1 as i64 + 1).abs() as u64;

            max_area = match max_area {
                None => Some(area),
                Some(max) if area > max => Some(area),
                _ => max_area,
            };
        }
    }

    max_area
}

fn parse_red_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(Point::try_from)
        .collect::<Result<Vec<Point>, _>>()
        .expect("Failed to parse points")
}

// fn print_grid(grid: &Vec<Vec<Tile>>) {
//     for line in grid.iter() {
//         for tile in line {
//             match tile {
//                 Tile::None => print!("."),
//                 Tile::Red => print!("R"),
//                 Tile::Green => print!("G"),
//             }
//         }
//         println!();
//     }
// }

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
