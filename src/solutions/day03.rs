use crate::input;

pub fn solve() {
    let x = input::raw_file_for_day(3);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: String) -> i64 {
    regex::Regex::new(r"(?m)mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(&input)
        .map(|c| c.extract())
        .fold(0, |acc, (_, [n1, n2])| {
            acc + n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap()
        })
}

fn part_two(input: String) -> i64 {
    regex::Regex::new(
        r#"(?mx)
            (
                mul\((\d{1,3}),(\d{1,3})\)|
                don't\(\)()()|
                do\(\)()()
            )"#,
    )
    .unwrap()
    .captures_iter(&input)
    .map(|c| c.extract())
    .fold((0, true), |(val, should_add), (_, [m, n1, n2])| match m {
        "do()" => (val, true),
        "don't()" => (val, false),
        _ if !should_add => (val, should_add),
        _ => (
            val + n1.parse::<i64>().unwrap() * n2.parse::<i64>().unwrap(),
            should_add,
        ),
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
