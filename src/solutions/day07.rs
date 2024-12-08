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

fn solve_with_op(input: Vec<String>, with_concat: bool) -> i64 {
    parse_input(input)
        .into_iter()
        .filter_map(|(goal, set)| {
            if is_valid(goal, set, with_concat) {
                Some(goal)
            } else {
                None
            }
        })
        .sum::<i64>()
}

fn is_valid(goal: i64, nums: Vec<i64>, with_concat: bool) -> bool {
    if nums.len() == 1 {
        return nums[0] == goal;
    }

    let (n1, n2) = (nums[0], nums[1]);
    let mut plus = vec![n1 + n2];
    plus.extend(nums[2..].iter());

    if is_valid(goal, plus, with_concat) {
        return true;
    }

    let mut addition = vec![n1 * n2];
    addition.extend(nums[2..].iter());

    if is_valid(goal, addition, with_concat) {
        return true;
    }

    if !with_concat {
        return false;
    }

    let mut concat = vec![format!("{n1}{n2}").parse::<i64>().unwrap()];
    concat.extend(nums[2..].iter());

    is_valid(goal, concat, with_concat)
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
