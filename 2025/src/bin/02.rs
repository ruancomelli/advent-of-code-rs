advent_of_code_2025::solution!(2);

use std::collections::HashSet;

/// Represents a class of invalid IDs.
///
/// For example, `InvalidIdClass(1)` represents the invalid IDs formed by
/// repeating a single-digit number, such as `55` (which is `5` appearing twice).
/// `InvalidIdClass(3)` would be numbers like 123123, which are repetitions of
/// three-digit numbers.
struct InvalidIdClass {
    digits: u8,
}

impl InvalidIdClass {
    /// Interestingly, all numbers that are formed by repeating
    /// a single-digit number are of the form `11 * n` where `n`
    /// has only one digit (from `1` to `9`).
    /// Numbers repeating two-digits numbers are of the form `101 * n`.
    /// Therefore, we can think of an `InvalidIdClass` as being an arithmetic
    /// sequence with a ration of `11`, `101`, `1001` etc.
    fn ratio(&self) -> u64 {
        1 + 10u64.pow(self.digits as u32)
    }

    /// Return the smallest number in this class.
    fn smallest(&self) -> u64 {
        10u64.pow(self.digits as u32 - 1) * self.ratio()
    }

    /// Return the largest number in this class.
    fn largest(&self) -> u64 {
        (10u64.pow(self.digits as u32) - 1) * self.ratio()
    }

    /// Return the smallest number in this class that is greater than or equal to `n`.
    fn ceil(&self, n: u64) -> u64 {
        let ratio = self.ratio();
        let ceil = n.div_ceil(ratio);
        (ceil * ratio).max(self.smallest())
    }

    /// Return the largest number in this class that is less than or equal to `n`.
    fn floor(&self, n: u64) -> u64 {
        let ratio = self.ratio();
        let floor = n / ratio;
        (floor * ratio).min(self.largest())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    fn sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
        let (lower, upper) = range;

        // Find the invalid ID class that might contain the lower bound
        let lower_digits_count = lower.to_string().len();
        let lower_id_class = if lower_digits_count % 2 == 0 {
            lower_digits_count / 2
        } else {
            lower_digits_count / 2 + 1
        } as u8;

        // Find the invalid ID class that might contain the upper bound
        let upper_digits_count = upper.to_string().len();
        let upper_id_class = (upper_digits_count / 2) as u8;

        let mut sum: u64 = 0;
        for id_class in lower_id_class..=upper_id_class {
            let id_class = InvalidIdClass { digits: id_class };
            let min = id_class.ceil(lower);
            let max = id_class.floor(upper);
            let range = (min..=max).step_by(id_class.ratio() as usize);
            sum += range.sum::<u64>();
        }

        sum
    }

    Some(
        input
            .split(',')
            .map(parse_range)
            .map(sum_invalid_ids_in_range)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    fn sum_invalid_ids_in_range(range: (u64, u64)) -> u64 {
        let (lower, upper) = range;
        let mut sum: u64 = 0;

        for id in lower..=upper {
            let digits = id.to_string().chars().collect::<Vec<char>>();
            let digits_count = digits.len();

            for repeated_digits_count in 1..=(digits_count / 2) {
                let unique_chunks = digits
                    .chunks(repeated_digits_count)
                    .into_iter()
                    .collect::<HashSet<_>>();
                if unique_chunks.len() == 1 {
                    sum += id;
                    break;
                }
            }
        }

        sum
    }

    Some(
        input
            .split(',')
            .map(parse_range)
            .map(sum_invalid_ids_in_range)
            .sum(),
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
