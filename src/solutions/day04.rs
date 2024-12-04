use crate::input;

pub fn solve() {
    let x = input::file_for_day(4);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

fn part_one(input: Vec<String>) -> i64 {
    let width = input[0].len();
    let height = input.len();

    let grid = input
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut xmas_found = 0;
    for i in 0..height {
        for j in 0..width {
            xmas_found += check_directions(&grid, i as isize, j as isize, width, height);
        }
    }

    xmas_found
}

fn part_two(input: Vec<String>) -> i64 {
    let width = input[0].len();
    let height = input.len();

    let grid = input
        .iter()
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut x_mas_found = 0;
    for i in 0..height {
        for j in 0..width {
            x_mas_found += check_directions_mas(&grid, i as isize, j as isize, width, height);
        }
    }

    x_mas_found
}

fn check_directions(grid: &[char], i: isize, j: isize, width: usize, height: usize) -> i64 {
    let mut valid = 0;

    for c in [
        [(i, j), (i + 1, j), (i + 2, j), (i + 3, j)], // Horizontal forward
        [(i, j), (i - 1, j), (i - 2, j), (i - 3, j)], // Horizontal backward
        [(i, j), (i, j + 1), (i, j + 2), (i, j + 3)], // Vertical down
        [(i, j), (i, j - 1), (i, j - 2), (i, j - 3)], // Vertical up
        [(i, j), (i + 1, j + 1), (i + 2, j + 2), (i + 3, j + 3)], // Diagonal down right
        [(i, j), (i - 1, j + 1), (i - 2, j + 2), (i - 3, j + 3)], // Diagonal down left
        [(i, j), (i - 1, j - 1), (i - 2, j - 2), (i - 3, j - 3)], // Diagonal up left
        [(i, j), (i + 1, j - 1), (i + 2, j - 2), (i + 3, j - 3)], // Diagonal up right
    ] {
        let (c1, c2, c3, c4) = (
            get_pos(grid, c[0].0, c[0].1, width, height),
            get_pos(grid, c[1].0, c[1].1, width, height),
            get_pos(grid, c[2].0, c[2].1, width, height),
            get_pos(grid, c[3].0, c[3].1, width, height),
        );

        if let (Some('X'), Some('M'), Some('A'), Some('S')) = (c1, c2, c3, c4) {
            valid += 1;
        }
    }

    valid
}

fn check_directions_mas(grid: &[char], i: isize, j: isize, width: usize, height: usize) -> i64 {
    let (c1, c2, c3, c4, c5) = (
        get_pos(grid, i, j, width, height),
        get_pos(grid, i + 2, j, width, height),
        get_pos(grid, i + 1, j + 1, width, height),
        get_pos(grid, i, j + 2, width, height),
        get_pos(grid, i + 2, j + 2, width, height),
    );

    match (c1, c2, c3, c4, c5) {
        (Some('M'), Some('S'), Some('A'), Some('M'), Some('S'))
        | (Some('S'), Some('S'), Some('A'), Some('M'), Some('M'))
        | (Some('M'), Some('M'), Some('A'), Some('S'), Some('S'))
        | (Some('S'), Some('M'), Some('A'), Some('S'), Some('M')) => 1,
        _ => 0,
    }
}

fn get_pos(grid: &[char], i: isize, j: isize, width: usize, height: usize) -> Option<char> {
    if i as usize >= height || i < 0 || j as usize >= width || j < 0 {
        return None;
    }

    let idx = i as usize * height + j as usize;

    Some(grid[idx])
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 18;
    static SOLUTION_TWO: i64 = 9;
    static TEST_INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

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
