use std::collections::HashSet;
use std::str;

#[derive(Debug, Clone)]
struct Node {
    cur: (usize, usize),
    next: (usize, usize),
    dist: usize,
}

impl Node {
    pub fn new(cur: (usize, usize), next: (usize, usize), dist: usize) -> Node {
        Node { cur, next, dist }
    }
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                return (row, col);
            }
        }
    }

    (0, 0)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum MovingDir {
    Up,
    Down,
    Right,
    Left,
}

fn get_next_pos(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    moving_dir: MovingDir,
) -> Option<((usize, usize), MovingDir)> {
    let (row, col) = pos;

    match moving_dir {
        MovingDir::Right => {
            if col == grid[0].len() - 1 {
                return None;
            }
            let next = grid[row][col + 1];
            let new_pos = (row, col + 1);
            match next {
                '-' | 'S' => Some((new_pos, moving_dir)),
                '7' => Some((new_pos, MovingDir::Down)),
                'J' => Some((new_pos, MovingDir::Up)),
                _ => {
                    //println!("right error: {}", next);
                    return None;
                }
            }
        }
        MovingDir::Left => {
            if col == 0 {
                return None;
            }
            let next = grid[row][col - 1];
            let new_pos = (row, col - 1);
            match next {
                '-' | 'S' => Some((new_pos, moving_dir)),
                'F' => Some((new_pos, MovingDir::Down)),
                'L' => Some((new_pos, MovingDir::Up)),
                _ => {
                    //println!("left error: {}", next);
                    return None;
                }
            }
        }
        MovingDir::Down => {
            if row == grid.len() - 1 {
                return None;
            }
            let next = grid[row + 1][col];
            let new_pos = (row + 1, col);
            match next {
                'J' => Some((new_pos, MovingDir::Left)),
                'L' => Some((new_pos, MovingDir::Right)),
                '|' | 'S' => Some((new_pos, moving_dir)),
                _ => {
                    //println!("Down error: {}", next);
                    return None;
                }
            }
        }
        MovingDir::Up => {
            if row == 0 {
                return None;
            }
            let next = grid[row - 1][col];
            match next {
                'F' => Some(((row - 1, col), MovingDir::Right)),
                '7' => Some(((row - 1, col), MovingDir::Left)),
                '|' | 'S' => Some(((row - 1, col), moving_dir)),
                _ => {
                    //println!("Up error: {}", next);
                    return None;
                }
            }
        }
        _ => None,
    }
}

fn build_path(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Node> {
    let mut path = Vec::new();
    let mut dist = 0;

    let initial: Vec<((usize, usize), MovingDir)> = [
        MovingDir::Right,
        MovingDir::Left,
        MovingDir::Down,
        MovingDir::Up,
    ]
    .iter()
    .filter_map(|x| get_next_pos(grid, start, *x))
    .collect();

    let (mut pos, mut moving_dir) = initial.first().unwrap();
    path.push(Node::new(start, pos, 0));

    while pos != start {
        let (new_pos, new_moving_dir) = get_next_pos(grid, pos, moving_dir).unwrap();

        dist += 1;
        path.push(Node::new(pos, new_pos, dist));

        pos = new_pos;
        moving_dir = new_moving_dir;
    }

    path
}

// polygon ray casting algorithm
fn count_invs(grid: &Vec<Vec<char>>, pos: (usize, usize), path: &HashSet<(usize, usize)>) -> usize {
    let (row, end) = pos;
    let mut count = 0;
    // go to the right
    for col in 0..end {
        if !path.contains(&(row, col)) {
            continue;
        }
        let c = grid[row][col];
        if c == 'J' || c == 'L' || c == '|' {
            count += 1;
        }
    }

    count
}

fn part1(input: &str) {
    let grid = parse_grid(input);
    let start = find_start(&grid);

    let path = build_path(&grid, start);

    let max = path.iter().map(|x| x.dist).max().unwrap();

    println!("Part1: {:?}", (max + 1) / 2);
}

fn part2(input: &str) {
    let grid = parse_grid(input);
    let start = find_start(&grid);
    let path = build_path(&grid, start);
    let path: HashSet<(usize, usize)> = HashSet::from_iter(path.iter().map(|n| n.cur));

    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let pos = (row, col);
            if !path.contains(&pos) {
                if count_invs(&grid, pos, &path) % 2 == 1 {
                    total += 1;
                }
            }
        }
    }

    println!("Part2: {}", total);
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
