use std::cmp::Ordering;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(2);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .fold((0, None, true), |(prev, direction, safe), v| {
                    if !safe {
                        return (0, None, false);
                    }

                    if prev == 0 {
                        return (v, None, true);
                    }

                    let diff = prev - v;
                    let dir = diff.cmp(&0);

                    if diff.abs() > 3 || (direction.is_some() && direction.unwrap() != dir) {
                        return (0, None, false);
                    }

                    (v, Some(dir), true)
                })
        })
        .filter(|(_, _, t)| *t)
        .count() as i64
}

fn part_two(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|line| {
            let n = line
                .split_whitespace()
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let mut direction: Option<Ordering> = None;
            let mut skips = 0;
            let (mut i, mut j) = (0, 1);

            while i < n.len() && j < n.len() {
                let diff = n[i] - n[j];
                let dir = diff.cmp(&0);

                // Diff to big, start skipping or increase if we're already trying.
                if diff.abs() > 3 || (direction.is_some() && direction.unwrap() != dir) {
                    // If we already skipped but `i` is 0, don't fail but start over and skip 0th
                    // index completely.
                    if skips >= 1 && i != 0 {
                        return false;
                    }

                    // [ _, i, j, _ ] If (i, j) like this fails
                    // [ i, _, j, _ ] Retry from start and skip 2nd
                    // [ _, i, j, _ ] If that fails, skip, reset and try this
                    //
                    // When we retry on a failure related to the first digit we need to reset the
                    // direction and try again since it might be a direction failure.
                    let is_adjacent = j - i == 1;
                    (i, j, direction) = match (i, j) {
                        (i, j) if is_adjacent && i == 1 => (0, j, None),
                        (i, j) if is_adjacent => (i, j + 1, direction),
                        (i, j) if i == 0 && j == 2 => (1, 2, None),
                        (i, j) => (i + 1, j, direction),
                    };

                    skips += 1;
                    continue;
                }

                (i, j, direction) = (j, j + 1, Some(dir));
            }

            true
        })
        .filter(|safe| *safe)
        .count() as i64
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 2;
    static SOLUTION_TWO: i64 = 4;
    static TEST_INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn part_one() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::test_vec(TEST_INPUT);
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
