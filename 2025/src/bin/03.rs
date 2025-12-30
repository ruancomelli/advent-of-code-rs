advent_of_code_2025::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(find_maximum_total_joltage_for_digit_count(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(find_maximum_total_joltage_for_digit_count(input, 12))
}

fn find_maximum_total_joltage_for_digit_count(banks: &str, digit_count: usize) -> u64 {
    banks
        .lines()
        .map(|line| {
            let bank = line.bytes().map(|byte| byte - b'0').collect::<Vec<_>>();
            find_maximum_joltage_for_digit_count(&bank, digit_count)
        })
        .sum()
}

fn find_maximum_joltage_for_digit_count(bank: &[u8], digit_count: usize) -> u64 {
    match digit_count {
        1 => *bank.iter().max().unwrap() as u64,
        digit_count if digit_count > 1 => {
            let remaining_digits = digit_count - 1;
            let (max_leading_index, &max_leading) = bank[..bank.len() - remaining_digits]
                .iter()
                .enumerate()
                .min_by_key(|(index, value)| (-(**value as i32), *index))
                .unwrap();

            10u64.pow(digit_count as u32 - 1) * max_leading as u64
                + find_maximum_joltage_for_digit_count(
                    &bank[max_leading_index + 1..],
                    digit_count - 1,
                )
        }
        digit_count => panic!("Invalid digit count: {} (should be >=1)", digit_count),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
