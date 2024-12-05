use std::collections::{HashMap, HashSet};

use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(5);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let (rules, input) = parse_input(input);

    input
        .iter()
        .map(|nums| {
            for i in 1..nums.len() {
                let val = nums[i];
                let Some(rule) = rules.get(&val) else {
                    continue;
                };

                for cmp in nums.iter().take(i) {
                    if rule.contains(cmp) {
                        return 0;
                    }
                }
            }

            nums[nums.len() / 2]
        })
        .sum()
}

fn part_two(input: String) -> i64 {
    let (rules, mut input) = parse_input(input);

    input
        .iter_mut()
        .map(|nums| {
            let mut swapped = false;
            for i in 1..nums.len() {
                let val = nums[i];
                let Some(rule) = rules.get(&val) else {
                    continue;
                };

                for j in 0..i {
                    let val_before = nums[j];
                    if rule.contains(&val_before) {
                        nums.swap(i, j);
                        swapped = true;
                    }
                }
            }

            if swapped {
                nums[nums.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn parse_input(input: String) -> (HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>) {
    let mut p = input.split("\n\n");
    let mut rules: HashMap<i64, HashSet<i64>> = HashMap::new();

    p.next().unwrap().lines().for_each(|line| {
        let mut l = line.split("|");
        let n1 = l.next().unwrap().parse::<i64>().unwrap();
        let n2 = l.next().unwrap().parse::<i64>().unwrap();

        let m = rules.entry(n1).or_default();
        m.insert(n2);
    });

    let res = p
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (rules, res)
}

#[cfg(test)]
mod tests {
    static SOLUTION_ONE: i64 = 143;
    static SOLUTION_TWO: i64 = 123;
    static TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
