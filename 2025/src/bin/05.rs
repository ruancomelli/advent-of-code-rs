advent_of_code_2025::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut fresh_ingredient_ranges: Vec<(u64, u64)> = Vec::new();
    let mut available_ingredients: Vec<u64> = Vec::new();

    let mut parsing_ranges = true;

    for line in input.lines() {
        if line.trim().is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            fresh_ingredient_ranges.push(parse_range(line));
        } else {
            available_ingredients.push(line.parse().unwrap());
        }
    }

    Some(
        available_ingredients
            .into_iter()
            .filter(|&ingredient| {
                fresh_ingredient_ranges
                    .iter()
                    .any(|&(start, end)| start <= ingredient && ingredient <= end)
            })
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut buffer: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }

        buffer.push(parse_range(line));
    }

    let mut fresh_ingredient_disjoint_ranges: Vec<(u64, u64)> = Vec::new();

    while let Some((start, end)) = buffer.pop() {
        let mut merged = false;

        for i in 0..fresh_ingredient_disjoint_ranges.len() {
            let (s, e) = fresh_ingredient_disjoint_ranges[i];
            if start <= e && end >= s {
                fresh_ingredient_disjoint_ranges.swap_remove(i);
                buffer.push((start.min(s), end.max(e)));
                merged = true;
                break;
            }
        }

        if !merged {
            fresh_ingredient_disjoint_ranges.push((start, end));
        }
    }

    Some(
        fresh_ingredient_disjoint_ranges
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<u64>(),
    )
}

fn parse_range(s: &str) -> (u64, u64) {
    let mut parts = s.trim().split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    (start, end)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
