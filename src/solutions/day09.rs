use std::collections::HashSet;

use crate::{input, time};

pub fn solve() {
    let x = input::raw_file_for_day(9);

    time::this("part 1", || {
        println!("Solution part 1: {}", part_one(x.clone()));
    });

    time::this("part 2", || {
        println!("Solution part 2: {}", part_two(x.clone()));
    });
}

#[derive(Debug, Clone)]
enum Block {
    File { id: usize, size: u32 },
    Space(u32),
}

fn part_one(input: String) -> i64 {
    let (_, _, mut ns) = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .fold((0, false, Vec::new()), |(id, is_space, mut ns), v| {
            for _ in 0..v {
                if is_space {
                    ns.push(None);
                } else {
                    ns.push(Some(id));
                }
            }

            (if is_space { id } else { id + 1 }, !is_space, ns)
        });

    for i in (0..ns.len()).rev() {
        let next_free_idx = ns.iter().position(|n| n.is_none()).unwrap();
        if next_free_idx >= i {
            break;
        }

        if ns[i].is_some() {
            ns.swap(next_free_idx, i);
        }
    }

    ns.iter().enumerate().fold(0, |acc, (i, v)| match v {
        None => acc,
        Some(v) => acc + v * i as i64,
    })
}

fn part_two(input: String) -> i64 {
    let (_, _, mut ns) = input
        .chars()
        .filter_map(|c| {
            if c.is_ascii_digit() {
                Some(c.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .fold((0, false, Vec::new()), |(id, is_space, mut ns), size| {
            if is_space {
                ns.push(Block::Space(size));
            } else {
                ns.push(Block::File { id, size });
            }

            (if is_space { id } else { id + 1 }, !is_space, ns)
        });

    let mut seen = HashSet::new();

    for i in (0..ns.len()).rev() {
        let (id, file_size) = match ns[i] {
            Block::File { id, size } => (id, size),
            _ => continue,
        };

        if seen.contains(&id) {
            continue;
        }

        seen.insert(id);

        let Some(next_free_idx) = ns
            .iter()
            .position(|s| matches!(s, Block::Space(n) if *n >= file_size))
        else {
            continue;
        };

        if next_free_idx >= i {
            continue;
        }

        ns.swap(i, next_free_idx);
        let old_space = &ns[i];

        let Block::Space(space_size) = old_space else {
            unreachable!();
        };

        let space_left = space_size - file_size;

        // This is dumb. If we swap so we move empty space before another empty space, we merge
        // them (make one of them zero and add to the other). If we move after an empty space, we
        // must do the same. If we move in between two empty spaces, we move the space from one to
        // the other and make one zero.
        // This allows us to always have the same size vector so the indexes still matches.
        let space_replaced = space_size - space_left;
        let before = if i >= 1 { Some(&ns[i - 1]) } else { None };
        let after = if i < ns.len() - 1 {
            Some(&ns[i + 1])
        } else {
            None
        };

        match (before, after) {
            (Some(Block::Space(n)), Some(Block::Space(m))) => {
                ns[i - 1] = Block::Space(n + m + space_replaced);
                ns[i + 1] = Block::Space(0);
                ns[i] = Block::Space(0);
            }
            (Some(Block::Space(n)), _) => {
                ns[i - 1] = Block::Space(n + space_replaced);
                ns[i] = Block::Space(0);
            }
            (_, Some(Block::Space(m))) => {
                ns[i + 1] = Block::Space(m + space_replaced);
                ns[i] = Block::Space(0);
            }
            _ => {
                ns[i] = Block::Space(space_replaced);
            }
        }

        // If there is free space left after moving a block, we must re-insert it.
        ns.insert(next_free_idx + 1, Block::Space(space_left));
    }

    ns.iter()
        .fold((0, 0), |(mut idx, mut sum), v| match v {
            Block::Space(size) => (idx + size, sum),
            Block::File { id, size } => {
                for _ in 0..*size {
                    sum += idx as i64 * *id as i64;
                    idx += 1
                }

                (idx, sum)
            }
        })
        .1
}

#[allow(dead_code)]
fn print_layout(ns: &[Block]) {
    for file in ns {
        match file {
            Block::Space(n) => {
                for _ in 0..*n {
                    print!(".");
                }
            }
            Block::File { id, size } => {
                for _ in 0..*size {
                    print!("{id}");
                }
            }
        }
    }

    println!();
}

#[cfg(test)]
mod tests {
    static SOLUTION_ONE: i64 = 1928;
    static SOLUTION_TWO: i64 = 2858;
    static TEST_INPUT: &str = r#"2333133121414131402"#;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
