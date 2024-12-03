use crate::input;

#[derive(Debug)]
enum Op {
    Mul(i64, i64),
    Do,
    Dont,
}

pub fn solve() {
    let x = input::raw_file_for_day(3);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    let re = regex::Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = vec![];
    for (_, [n1, n2]) in re.captures_iter(&input).map(|c| c.extract()) {
        res.push(n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap());
    }

    res.iter().sum()
}

fn part_two(input: String) -> i64 {
    let re = regex::Regex::new(
        r#"(?mx)
        (
            mul\((\d{1,3}),(\d{1,3})\)|
            don't\(\)()()|
            do\(\)()()
        )"#,
    )
    .unwrap();
    let mut res = vec![];
    for (_, [m, n1, n2]) in re.captures_iter(&input).map(|c| c.extract()) {
        let op = match m {
            "do()" => Op::Do,
            "don't()" => Op::Dont,
            _ => Op::Mul(n1.parse::<i64>().unwrap(), n2.parse::<i64>().unwrap()),
        };

        res.push(op);
    }

    res.iter()
        .fold((0, true), |(total, should_add), v| match v {
            Op::Mul(a, b) if should_add => (total + a * b, should_add),
            Op::Do => (total, true),
            Op::Mul(_, _) | Op::Dont => (total, false),
        })
        .0
}

#[cfg(test)]
mod tests {
    static SOLUTION_ONE: i64 = 161;
    static SOLUTION_TWO: i64 = 48;
    static TEST_INPUT: &str =
        r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    static TEST_INPUT_TWO: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
    }
}
