use std::collections::HashSet;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(10);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let (starting_points, grid) = grid(input);
    let mut seen_nines = HashSet::new();

    for (i, j) in starting_points {
        traverse(&grid, &mut seen_nines, 10, (i, j), (i, j));
    }

    seen_nines.len() as i64
}

fn part_two(input: Vec<String>) -> i64 {
    let (starting_points, grid) = grid(input);
    let mut seen_nines = HashSet::new();

    starting_points.into_iter().fold(0, |acc, (i, j)| {
        acc + traverse(&grid, &mut seen_nines, 10, (i, j), (i, j))
    })
}

fn grid(input: Vec<String>) -> (HashSet<(usize, usize)>, Vec<Vec<u32>>) {
    let mut starting_points = HashSet::new();
    let g = input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    let n = c.to_digit(10).unwrap();
                    if n == 0 {
                        starting_points.insert((i, j));
                    }

                    n
                })
                .collect::<Vec<_>>()
        })
        .collect();

    (starting_points, g)
}

fn traverse(
    grid: &[Vec<u32>],
    seen_nines: &mut HashSet<(usize, usize, usize, usize)>,
    prev: u32,
    (i, j): (usize, usize),
    (start_i, start_j): (usize, usize),
) -> i64 {
    let pos_value = grid[i][j];
    if prev != 10 && pos_value != prev + 1 {
        return 0;
    }

    if pos_value == 9 {
        seen_nines.insert((start_i, start_j, i, j));
        return 1;
    }

    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .fold(0, |acc, (i_add, j_add)| {
            if (i == 0 && *i_add < 0) || (j == 0 && *j_add < 0) {
                return acc;
            }

            let (new_i, new_j) = ((i as i32 + i_add) as usize, (j as i32 + j_add) as usize);
            if new_i > grid.len() - 1 || new_j > grid[0].len() - 1 {
                return acc;
            }

            acc + traverse(
                grid,
                seen_nines,
                pos_value,
                (new_i, new_j),
                (start_i, start_j),
            )
        })
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<u32>], (i, j): (usize, usize), (i1, j1): (usize, usize)) {
    for (x, row) in grid.iter().enumerate() {
        for (y, val) in row.iter().enumerate() {
            if i == x && j == y {
                print!("[{val}]");
            } else if i1 == x && j1 == y {
                print!("<{val}>");
            } else {
                print!(" {val} ");
            }
        }

        println!()
    }

    println!()
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 36;
    static SOLUTION_TWO: i64 = 81;
    static TEST_INPUT: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

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
