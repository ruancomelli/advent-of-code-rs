advent_of_code_2025::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .split(',')
        .map(parse_range)
        .map(|(start, end)| count_invalid_ids_in_range(start, end))
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn count_invalid_ids_in_range()

fn parse_range(s: &str) -> (u64, u64) {
    let mut parts = s.split('-');
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
