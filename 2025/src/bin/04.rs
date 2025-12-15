advent_of_code_2025::solution!(4);

const MAX_NEIGHBORS_FOR_REMOVAL: usize = 4;

pub fn part_one(input: &str) -> Option<u64> {
    let diagram = parse_diagram(input);
    let rows = diagram.len();
    let cols = diagram[0].len();

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            if !diagram[i][j] {
                continue;
            }

            if can_remove_roll_of_paper(&diagram, rows, cols, i, j) {
                count += 1;
            }
        }
    }

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut diagram = parse_diagram(input);

    let rows = diagram.len();
    let cols = diagram[0].len();

    let mut count = 0;
    // Start trying.
    // On every iteration, "give up" -
    // unless we remove a roll of paper, in
    // which case we should keep trying to
    // see if the newly removed roll unlocked
    // more removals
    let mut keep_trying = true;

    while keep_trying {
        keep_trying = false;

        for i in 0..rows {
            for j in 0..cols {
                if !diagram[i][j] {
                    continue;
                }

                if can_remove_roll_of_paper(&diagram, rows, cols, i, j) {
                    diagram[i][j] = false;
                    count += 1;
                    keep_trying = true;
                }
            }
        }
    }

    Some(count as u64)
}

fn can_remove_roll_of_paper(
    diagram: &Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
    i: usize,
    j: usize,
) -> bool {
    let mut neighbor_count = 0;

    for ii in i.saturating_sub(1)..=(i + 1).min(rows - 1) {
        for jj in j.saturating_sub(1)..=(j + 1).min(cols - 1) {
            if ii == i && jj == j {
                continue;
            }
            if diagram[ii][jj] {
                neighbor_count += 1;
            }
        }
    }

    neighbor_count < MAX_NEIGHBORS_FOR_REMOVAL
}

fn parse_diagram(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code_2025::template::read_file("examples", DAY));
        let ans = "..xx.xx@x.
            x@@.@.@.@@
            @@@@@.x.@@
            @.@@@@..@.
            x@.@@@@.@x
            .@@@@@@@.@
            .@.@.@.@@@
            x.@@@.@@@@
            .@@@@@@@@.
            x.x.@@@.x.";
        assert_eq!(
            result,
            Some(ans.chars().filter(|&c| c == 'x').count() as u64)
        );
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
