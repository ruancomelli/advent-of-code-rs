use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

advent_of_code_2025::solution!(10);

type State = Vec<bool>;
type Button = Vec<usize>;

#[derive(Debug, PartialEq, Eq)]
struct MachineConfig {
    desired_state: State,
    buttons: Vec<Button>,
    joltages: Vec<u64>,
}

impl MachineConfig {
    fn new(state: &[bool], buttons: &[Button], joltages: &[u64]) -> Result<Self, &'static str> {
        // TODO: later add joltage size requirements

        for button in buttons {
            for &light in button {
                // ensure that all lights indices are suitable indices for `state`
                if light >= state.len() {
                    return Err("Button light index out of range");
                }
            }
        }
        Ok(Self {
            desired_state: state.to_vec(),
            buttons: buttons.to_vec(),
            joltages: joltages.to_vec(),
        })
    }
}

impl TryFrom<&str> for MachineConfig {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(' ');

        let state_str = parts.next().ok_or("Missing state substring".to_string())?;
        let joltages_str = parts.next_back().ok_or("Missing joltages substring")?;

        let buttons_str: Vec<&str> = parts.collect();

        let state = state_str
            .strip_prefix('[')
            .ok_or("State substring does not start with square brackets")?
            .strip_suffix(']')
            .ok_or("State substring does not end with square brackets")?
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '#' => Ok(true),
                other => Err(format!("Invalid character in state string: {}", other)),
            })
            .collect::<Result<State, String>>()?;

        let buttons = buttons_str
            .into_iter()
            .map(|button_str| {
                button_str
                    .strip_prefix('(')
                    .ok_or("State string does not start with parenthesis")?
                    .strip_suffix(')')
                    .ok_or("State string does not end with parenthesis")?
                    .split(',')
                    .map(|digit_str| {
                        digit_str
                            .parse::<u64>()
                            .map_err(|_| format!("Invalid digit in button string: {}", digit_str))
                    })
                    .collect::<Result<Vec<u64>, _>>()
            })
            .collect::<Result<Vec<Vec<u64>>, _>>()?;

        let joltages = joltages_str
            .strip_prefix('{')
            .ok_or("Joltages string does not start with curly brackets")?
            .strip_suffix('}')
            .ok_or("Joltages string does not end with curly brackets")?
            .split(',')
            .map(|digit_str| {
                digit_str
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid digit in button string: {}", digit_str))
            })
            .collect::<Result<Vec<u64>, _>>()?;

        Self::new(&state, &buttons, &joltages)
            .map_err(|err| format!("Failed to create machine: {}", err))
    }
}

impl Display for MachineConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.desired_state
                .iter()
                .map(|&light| if light { '#' } else { '.' })
                .collect::<String>()
        )?;
        write!(f, " ")?;
        write!(
            f,
            "{}",
            self.buttons
                .iter()
                .map(|button| format!(
                    "({})",
                    button.iter().map(|digit| digit.to_string()).join(",")
                ))
                .join(" ")
        )?;
        write!(f, " ")?;
        write!(
            f,
            "{{{}}}",
            self.joltages.iter().map(|j| j.to_string()).join(",")
        )?;

        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = input
        .split('\n')
        .map(|line| MachineConfig::try_from(line).expect("Failed to parse machine config"))
        .collect::<Vec<_>>();

    fn _apply_button_mut(state: &mut State, button: &Button) {
        for digit in button {
            state.push(*digit);
        }
    }

    fn _calculate_fewest_button_presses(machine: &MachineConfig) -> u64 {
        let mut seen: HashSet<State> = HashSet::new();
        let mut count = 0;
        let mut next_attempts: Vec<State> = Vec::new();

        loop {
            for ... {
                let new_state = ...;

                if new_state =
            }
        }
        0
    }

    Some(
        machines
            .into_iter()
            .map(|machine| _calculate_fewest_button_presses(&machine))
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
