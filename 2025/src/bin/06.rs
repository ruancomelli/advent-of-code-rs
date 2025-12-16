advent_of_code_2025::solution!(6);

enum Op {
    Add,
    Multiply,
}

pub fn part_one(input: &str) -> Option<u64> {
    fn parse_problem(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
        let mut operands_matrix: Vec<Vec<u64>> = Vec::new();

        for line in input.lines() {
            if line.contains(|c| c == '+' || c == '*') {
                return (
                    operands_matrix,
                    line.split_whitespace()
                        .map(|op| match op {
                            "+" => Op::Add,
                            "*" => Op::Multiply,
                            other => panic!("Invalid operation: {}", other),
                        })
                        .collect(),
                );
            }

            operands_matrix.push(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            );
        }

        panic!("Did not find row of operations");
    }

    let (operands_matrix, operations) = parse_problem(input);

    Some(
        operations
            .into_iter()
            .enumerate()
            .map(|(index, operation)| {
                let operands = operands_matrix.iter().map(|row| row[index]);
                match operation {
                    Op::Add => operands.sum::<u64>(),
                    Op::Multiply => operands.product(),
                }
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // Source - https://stackoverflow.com/a/64499219
    // Posted by Netwave, modified by community. See post 'Timeline' for change history
    // Retrieved 2025-12-16, License - CC BY-SA 4.0

    fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap_or(' '))
                    .collect::<Vec<char>>()
            })
            .collect()
    }

    let input_lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let transposed_input_lines = transpose(input_lines.clone());
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut acc: Vec<String> = Vec::new();
    for transposed_input_line in transposed_input_lines {
        // Groups are separated by whitespace-only lines
        if transposed_input_line.iter().all(|&c| c.is_whitespace()) {
            groups.push(acc.clone());
            acc.clear();
        } else {
            acc.push(transposed_input_line.iter().collect());
        }
    }
    groups.push(acc);

    Some(
        groups
            .into_iter()
            .map(|mut group| {
                // The operation to be executed is always the last character of the first line in the group
                let operation = match group[0].pop().unwrap() {
                    '+' => Op::Add,
                    '*' => Op::Multiply,
                    op => panic!("Invalid operation {:?}", op),
                };

                let operands = group
                    .into_iter()
                    .map(|line| line.trim().parse::<u64>().unwrap());

                match operation {
                    Op::Add => operands.sum::<u64>(),
                    Op::Multiply => operands.product(),
                }
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
