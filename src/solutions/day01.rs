use crate::input;
use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(1);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let pairs: Vec<(i64, i64)> = input
        .into_iter()
        .map(|line| {
            let mut nums = line.split_whitespace();
            (
                nums.next().unwrap().parse::<i64>().unwrap(),
                nums.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect();

    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();

    right.sort_unstable();
    left.sort_unstable();

    right
        .into_iter()
        .zip(left)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_two(input: Vec<String>) -> i64 {
    let mut left = Vec::new();
    let mut right = HashMap::new();

    input.iter().for_each(|l| {
        let mut nums = l.split_whitespace();
        left.push(nums.next().unwrap().parse::<i64>().unwrap());

        *right
            .entry(nums.next().unwrap().parse::<i64>().unwrap())
            .or_insert(0) += 1
    });

    left.into_iter()
        .map(|n| n * right.get(&n).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 11;
    static SOLUTION_TWO: i64 = 31;
    static TEST_INPUT: &str = r#"
3   4
4   3
2   5
1   3
3   9
3   3
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
