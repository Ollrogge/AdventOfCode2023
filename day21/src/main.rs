use std::collections::{HashSet, VecDeque};

struct State {
    pos: (isize, isize),
    steps: usize,
}

impl State {
    pub fn new(pos: (isize, isize), steps: usize) -> State {
        State { pos, steps }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                start = (row as isize, col as isize);
            }
        }
    }

    (grid, start)
}

fn part1(input: &str) {
    let (grid, start) = parse_input(input);
    let mut work = VecDeque::new();

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    work.push_back(State::new(start, 0));

    let max_steps = 64;

    let (x, y) = start;
    let mut seen = HashSet::from([(x, y, 0)]);

    while let Some(s) = work.pop_front() {
        if s.steps == max_steps {
            continue;
        }
        for dir in dirs.iter() {
            let row = s.pos.0 + dir.0;
            let col = s.pos.1 + dir.1;
            if row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize {
                let c = grid[row as usize][col as usize];
                if c == '.' || c == 'S' {
                    if seen.insert((row, col, s.steps + 1)) {
                        work.push_back(State::new((row, col), s.steps + 1));
                    }
                }
            }
        }
    }

    let reachable = seen
        .iter()
        .filter(|(_, _, steps)| *steps == max_steps)
        .count();

    println!("Part1: {}", reachable);
}

fn part2(input: &str) {
    let (grid, start) = parse_input(input);
    let mut work = VecDeque::new();

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    work.push_back(State::new(start, 0));

    let max_steps = 26501365;

    let (x, y) = start;
    let mut seen = HashSet::from([(x, y, 0)]);

    while let Some(s) = work.pop_front() {
        if s.steps == max_steps {
            continue;
        }
        for dir in dirs.iter() {
            let row = ((s.pos.0 + dir.0).rem_euclid(grid.len() as isize)) as usize;
            let col = ((s.pos.1 + dir.1).rem_euclid(grid[0].len() as isize)) as usize;
            let c = grid[row as usize][col as usize];
            if c == '.' || c == 'S' {
                let row = s.pos.0 + dir.0;
                let col = s.pos.1 + dir.1;
                if seen.insert((row, col, s.steps + 1)) {
                    work.push_back(State::new((row, col), s.steps + 1));
                }
            }
        }
    }

    let reachable = seen
        .iter()
        .filter(|(_, _, steps)| *steps == max_steps)
        .count();

    println!("Part2: {}", reachable);
}
fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
