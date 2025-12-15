advent_of_code_2025::solution!(1);

const INITIAL_POSITION: u8 = 50;
const MAXIMUM_POSITION: u8 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let mut pointer = INITIAL_POSITION as i32;
    let mut password: u64 = 0;

    for line in input.lines() {
        let rotation = parse_rotation(&line).expect("Failed to parse line");
        pointer = (pointer + rotation).rem_euclid(MAXIMUM_POSITION as i32);
        if pointer == 0 {
            password += 1;
        }
    }

    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pointer = INITIAL_POSITION as i32;
    let mut password: u64 = 0;

    for line in input.lines() {
        let rotation = parse_rotation(&line).expect("Failed to parse line");
        password += count_zero_clicks(pointer, MAXIMUM_POSITION as i32, rotation) as u64;
        pointer = (pointer + rotation).rem_euclid(MAXIMUM_POSITION as i32);
    }

    Some(password)
}

fn parse_rotation(line: &str) -> Result<i32, String> {
    if let Some(amount_str) = line.strip_prefix("L") {
        let amount: i32 = amount_str.parse().map_err(|_| "Invalid amount")?;
        Ok(-amount)
    } else if let Some(amount_str) = line.strip_prefix("R") {
        let amount = amount_str.parse().map_err(|_| "Invalid amount")?;
        Ok(amount)
    } else {
        Err("Invalid direction".to_string())
    }
}

fn count_zero_clicks(pointer: i32, size: i32, rotation: i32) -> i32 {
    let previous_group = calculate_group_for_pointer(pointer, size, rotation);
    let new_group = calculate_group_for_pointer(pointer + rotation, size, rotation);
    (new_group - previous_group).abs()
}

/// Calculates the group for a given pointer position, size, and rotation.
///
/// The group is a useful concept for determining whether the point `0` is clicked
/// or not: we just need to check if the group changed or not.
/// The trick with subtracting 1 if rotation is negative enables us to quickly account
/// for situations like "we are currently at zero, but will move to -1" - this doesn't
/// click `0`.
fn calculate_group_for_pointer(pointer: i32, size: i32, rotation: i32) -> i32 {
    (if rotation < 0 { pointer - 1 } else { pointer }).div_euclid(size)
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
        assert_eq!(result, Some(6));
    }
}
