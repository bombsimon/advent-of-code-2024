use std::collections::HashSet;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(7);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    solve_with_op(input, false)
}

fn part_two(input: Vec<String>) -> i64 {
    solve_with_op(input, true)
}

fn solve_with_op(input: Vec<String>, or_op: bool) -> i64 {
    parse_input(input)
        .into_iter()
        .map(|(goal, set)| {
            let mut sums = get_permutations(set[0], set[1], or_op);

            set.iter().skip(2).for_each(|x| {
                let mut s2 = HashSet::new();
                for v in &sums {
                    s2.extend(
                        get_permutations(*v, *x, or_op)
                            .iter()
                            .filter(|&r| *r <= goal)
                            .collect::<HashSet<_>>(),
                    );
                }

                sums = s2;
            });

            if sums.contains(&goal) {
                goal
            } else {
                0
            }
        })
        .sum::<i64>()
}

fn parse_input(input: Vec<String>) -> Vec<(i64, Vec<i64>)> {
    input
        .iter()
        .map(|line| {
            let mut l = line.split(": ");
            let p1 = l.next().unwrap().parse::<i64>().unwrap();
            let p2 = l
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            (p1, p2)
        })
        .collect()
}

fn get_permutations(a: i64, b: i64, or_op: bool) -> HashSet<i64> {
    let mut p = HashSet::new();
    p.insert(a + b);
    p.insert(a * b);

    if or_op {
        p.insert(format!("{}{}", a, b).parse::<i64>().unwrap());
    }

    p
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 3749;
    static SOLUTION_TWO: i64 = 11387;
    static TEST_INPUT: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

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
