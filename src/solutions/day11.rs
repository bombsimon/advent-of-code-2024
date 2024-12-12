use std::collections::HashMap;

use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(11);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    find_with_steps(input, 25)
}

fn part_two(input: String) -> i64 {
    find_with_steps(input, 75)
}

fn find_with_steps(input: String, steps: i64) -> i64 {
    let s = input
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    s.iter().fold(0, |acc, v| acc + find(&mut cache, *v, steps))
}

fn find(cache: &mut HashMap<(i64, i64), i64>, n: i64, iteration: i64) -> i64 {
    if let Some(stones) = cache.get(&(n, iteration)) {
        return *stones;
    }

    if iteration == 0 {
        return 1;
    }

    let res = match (n.to_string(), n) {
        (_, _) if n == 0 => find(cache, 1, iteration - 1),
        (s, _) if s.len() % 2 == 0 => {
            let lhs = s[..s.len() / 2].parse::<i64>().unwrap();
            let rhs = s[s.len() / 2..].parse::<i64>().unwrap();

            find(cache, lhs, iteration - 1) + find(cache, rhs, iteration - 1)
        }
        (_, n) => find(cache, n * 2024, iteration - 1),
    };

    cache.insert((n, iteration), res);

    res
}

#[cfg(test)]
mod tests {
    static SOLUTION_ONE: i64 = 55312;
    static SOLUTION_TWO: i64 = 0;
    static TEST_INPUT: &str = r#"125 17"#;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
