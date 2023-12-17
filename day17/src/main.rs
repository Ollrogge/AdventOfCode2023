use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as usize - 48).collect())
        .collect()
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Path {
    loss: usize,
    pos: (isize, isize),
    dir: (isize, isize),
    sid: usize,
}

impl Path {
    pub fn new(pos: (isize, isize), dir: (isize, isize), sid: usize, loss: usize) -> Path {
        Path {
            loss,
            pos,
            dir,
            sid,
        }
    }

    pub fn step(&mut self) {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;
        self.sid += 1;
    }

    pub fn rev_dir(&self) -> (isize, isize) {
        let (x, y) = self.dir;
        (-x, -y)
    }
}

fn dijkstra(grid: &Vec<Vec<usize>>, part2: bool) -> usize {
    let up = (-1, 0);
    let down = (1, 0);
    let right = (0, 1);
    let left = (0, -1);
    let dirs = vec![up, down, right, left];
    let mut loss_map = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    // min heap
    let mut work = BinaryHeap::new();

    loss_map[0][0] = 0;

    dirs.iter()
        .for_each(|&d| work.push(Reverse(Path::new((0, 0), d, 0, 0))));

    let mut seen = HashSet::new();

    while let Some(Reverse(mut path)) = work.pop() {
        path.step();

        let key = (path.pos, path.dir, path.sid);

        if path.pos.0 < 0
            || path.pos.0 >= grid.len() as isize
            || path.pos.1 < 0
            || path.pos.1 >= grid[0].len() as isize
            || part2 == false && path.sid > 3
            || part2 && path.sid > 10
            || seen.contains(&key)
        {
            continue;
        }

        seen.insert(key);

        path.loss += grid[path.pos.0 as usize][path.pos.1 as usize];
        if path.loss < loss_map[path.pos.0 as usize][path.pos.1 as usize] {
            loss_map[path.pos.0 as usize][path.pos.1 as usize] = path.loss;
        }

        for &d in dirs.iter() {
            if d != path.rev_dir() {
                if d == path.dir {
                    work.push(Reverse(path.clone()))
                } else {
                    if part2 && path.sid >= 4 {
                        work.push(Reverse(Path::new(path.pos, d, 0, path.loss)))
                    } else if !part2 {
                        work.push(Reverse(Path::new(path.pos, d, 0, path.loss)))
                    }
                }
            }
        }
    }

    loss_map[grid.len() - 1][grid[0].len() - 1]
}

fn part1(input: &str) {
    let grid = parse_input(input);
    let res = dijkstra(&grid, false);

    println!("Part1: {}", res);
}

fn part2(input: &str) {
    let grid = parse_input(input);
    let res = dijkstra(&grid, true);

    println!("Part2: {}", res);
}
fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
