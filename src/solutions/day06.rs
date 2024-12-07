use std::collections::HashSet;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(6);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x));
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Direction::Up => (x - 1, y),
            Direction::Right => (x, y + 1),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
        }
    }

    fn as_number(&self) -> isize {
        match self {
            Direction::Up => 1,
            Direction::Right => 2,
            Direction::Down => 3,
            Direction::Left => 4,
        }
    }
}

fn part_one(input: Vec<String>) -> i64 {
    let height = input.len();
    let width = input[0].len();

    let (mut x, mut y) = (0, 0);
    let mut dir = Direction::Up;

    let grid = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let chars = line.chars().collect::<Vec<_>>();
            if let Some(j) = chars.iter().position(|&c| c == '^') {
                (x, y) = (i as isize, j as isize);
            }

            chars
        })
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();

    loop {
        visited.insert((x, y));
        let (next_x, next_y) = dir.next(x, y);

        match get_pos(&grid, next_x, next_y, width, height) {
            Some('#') => dir = dir.turn(),
            Some(_) => {
                (x, y) = (next_x, next_y);
                continue;
            }
            None => break,
        }
    }

    visited.len() as i64
}

// Nasty brute force, gave up on other impl.
fn part_two(input: Vec<String>) -> i64 {
    let height = input.len();
    let width = input[0].len();

    let (mut x, mut y) = (0, 0);
    let mut dir = Direction::Up;

    let grid = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let chars = line.chars().collect::<Vec<_>>();
            if let Some(j) = chars.iter().position(|&c| c == '^') {
                (x, y) = (i as isize, j as isize);
            }

            chars
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    let (orig_x, orig_y, orig_dir) = (x, y, dir);

    for i in 0..width {
        for j in 0..height {
            // The new obstruction can't be placed at the guard's starting position - the guard is
            // there right now and would notice.
            if i as isize == orig_x && j as isize == orig_y {
                continue;
            }

            // Just clone our grid and add a new `#` at every position til we loop.
            let mut g = grid.clone();
            let idx = get_idx(i as isize, j as isize, width, height).unwrap();
            g[idx] = '#';

            let mut visited = HashSet::new();

            loop {
                let pos = ((x, y), dir.as_number());
                if visited.contains(&pos) {
                    count += 1;
                    break;
                }

                visited.insert(pos);
                let (next_x, next_y) = dir.next(x, y);

                match get_pos(&g, next_x, next_y, width, height) {
                    Some('#') => dir = dir.turn(),
                    Some(_) => {
                        (x, y) = (next_x, next_y);
                        continue;
                    }
                    None => break,
                }
            }

            (x, y, dir) = (orig_x, orig_y, orig_dir);
        }
    }

    count
}

// WIP - Works for test input but never got it working fully.
#[allow(dead_code)]
fn part_two_non_bruteforce(input: Vec<String>) -> i64 {
    let height = input.len();
    let width = input[0].len();

    let (mut x, mut y) = (0, 0);
    let mut dir = Direction::Up;

    let grid = input
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            let chars = line.chars().collect::<Vec<_>>();
            if let Some(j) = chars.iter().position(|&c| c == '^') {
                (x, y) = (i as isize, j as isize);
            }

            chars
        })
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut blocker_added = HashSet::new();
    let mut count = 0;

    loop {
        visited.insert(((x, y), dir.as_number()));
        let (next_x, next_y) = dir.next(x, y);

        let turned = dir.turn();
        let (mut x1, mut y1) = (x, y);
        let (mut x2, mut y2) = turned.next(x1, y1);
        while let Some(c) = get_pos(&grid, x2, y2, width, height) {
            if c == '#' {
                if visited.contains(&((x1, y1), turned.turn().as_number())) {
                    blocker_added.insert((next_x, next_y));
                    count += 1;
                }

                break;
            }

            (x1, y1) = turned.next(x1, y1);
            (x2, y2) = turned.next(x2, y2);

            // TODO: Should probably check we're not crossing a previous path, no need to check
            // other side since if this path is blocked we couldn't have gotten here.
        }

        match get_pos(&grid, next_x, next_y, width, height) {
            Some('#') => dir = dir.turn(),
            Some(_) => {
                (x, y) = (next_x, next_y);
                continue;
            }
            None => break,
        }
    }

    count
}

#[allow(dead_code)]
fn print_grid(
    grid: &[char],
    x: isize,
    y: isize,
    i1: isize,
    j1: isize,
    width: usize,
    height: usize,
) {
    for i in 0..width {
        for j in 0..height {
            if i as isize == i1 && j as isize == j1 {
                print!(
                    "[{}]",
                    get_pos(grid, i as isize, j as isize, width, height).unwrap()
                );
            } else if i as isize == x && j as isize == y {
                print!(" @ ",);
            } else {
                print!(
                    " {} ",
                    get_pos(grid, i as isize, j as isize, width, height).unwrap()
                );
            }
        }
        println!();
    }

    println!()
}

fn get_idx(i: isize, j: isize, width: usize, height: usize) -> Option<usize> {
    if i as usize >= height || i < 0 || j as usize >= width || j < 0 {
        return None;
    }

    Some(i as usize * height + j as usize)
}

fn get_pos(grid: &[char], i: isize, j: isize, width: usize, height: usize) -> Option<char> {
    get_idx(i, j, width, height).map(|idx| grid[idx])
}

#[cfg(test)]
mod tests {
    use crate::input;

    static SOLUTION_ONE: i64 = 41;
    static SOLUTION_TWO: i64 = 6;
    static TEST_INPUT: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

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
