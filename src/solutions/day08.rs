use std::collections::{HashMap, HashSet};

use crate::input;

pub fn solve() {
    let x = input::file_for_day(8);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    solve_nodes(input, false)
}

fn part_two(input: Vec<String>) -> i64 {
    solve_nodes(input, true)
}

fn solve_nodes(input: Vec<String>, any_node: bool) -> i64 {
    let mut map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let grid = input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, char)| {
                    if char == '.' {
                        return char;
                    }

                    let pos = (i, j);
                    let m = map.entry(char).or_default();
                    m.insert(pos);

                    char
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut anti_nodes = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            let pos = (i, j);
            let siblings = map
                .get(char)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|p| *p != pos)
                .collect::<Vec<_>>();

            for (i1, j1) in siblings {
                if any_node {
                    anti_nodes.insert((i1, j1));
                }

                let (idelta, ip) = if i > i1 { (i - i1, '+') } else { (i1 - i, '-') };
                let (jdelta, jp) = if j > j1 { (j - j1, '+') } else { (j1 - j, '-') };

                let (mut ix, mut jx) = (i, j);

                loop {
                    let new_i = match ip {
                        '-' if idelta <= ix => ix - idelta,
                        '+' if ix + idelta < grid.len() => ix + idelta,
                        _ => break,
                    };

                    let new_j = match jp {
                        '-' if jdelta <= jx => jx - jdelta,
                        '+' if jx + jdelta < grid[0].len() => jx + jdelta,
                        _ => break,
                    };

                    anti_nodes.insert((new_i, new_j));
                    if !any_node {
                        break;
                    }

                    (ix, jx) = (new_i, new_j);
                }
            }
        }
    }

    anti_nodes.len() as i64
}

#[allow(dead_code)]
fn print_grid(
    grid: &[Vec<char>],
    me: (usize, usize),
    sibling: (usize, usize),
    anti_node: (usize, usize),
) {
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let pos = (i, j);
            if pos == me {
                print!(" M ");
            } else if pos == sibling {
                print!(" S ");
            } else if pos == anti_node {
                print!(" # ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }

    println!("me={me:?}, sibling={sibling:?}, anti_node={anti_node:?}");
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 14;
    static SOLUTION_TWO: i64 = 34;
    static TEST_INPUT: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

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
