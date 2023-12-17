use std::cmp;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Beam {
    moving_dir: (isize, isize),
    pos: (isize, isize),
}

impl Beam {
    pub fn new(moving_dir: (isize, isize), pos: (isize, isize)) -> Beam {
        Beam { moving_dir, pos }
    }

    pub fn step(&mut self) {
        self.pos.0 += self.moving_dir.0;
        self.pos.1 += self.moving_dir.1;
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn get_energized_amount(grid: &Vec<Vec<char>>, start_beam: Beam) -> usize {
    let mut energized_map = HashSet::new();
    let mut work = VecDeque::new();
    let left = (0, -1);
    let right = (0, 1);
    let up = (-1, 0);
    let down = (1, 0);

    work.push_front(start_beam);

    let mut start = true;

    while let Some(mut beam) = work.pop_front() {
        if !start {
            beam.step();
        } else {
            start = false;
        }

        if beam.pos.0 < 0
            || beam.pos.0 >= grid.len() as isize
            || beam.pos.1 < 0
            || beam.pos.1 >= grid[0].len() as isize
        {
            continue;
        }

        // seen a beam with same pos and moving dir before so can't give us new info
        if !energized_map.insert(beam.clone()) {
            continue;
        }

        let c = grid[beam.pos.0 as usize][beam.pos.1 as usize];
        match c {
            '|' => {
                if beam.moving_dir == right || beam.moving_dir == left {
                    work.push_back(Beam::new(up, beam.pos));
                    work.push_back(Beam::new(down, beam.pos));
                } else if beam.moving_dir == down || beam.moving_dir == up {
                    work.push_back(beam);
                }
            }
            '-' => {
                if beam.moving_dir == right || beam.moving_dir == left {
                    work.push_back(beam);
                } else if beam.moving_dir == up || beam.moving_dir == down {
                    work.push_back(Beam::new(left, beam.pos));
                    work.push_back(Beam::new(right, beam.pos));
                }
            }
            '/' => {
                if beam.moving_dir == right {
                    work.push_back(Beam::new(up, beam.pos));
                } else if beam.moving_dir == left {
                    work.push_back(Beam::new(down, beam.pos));
                } else if beam.moving_dir == up {
                    work.push_back(Beam::new(right, beam.pos));
                } else {
                    work.push_back(Beam::new(left, beam.pos));
                }
            }
            '\\' => {
                if beam.moving_dir == right {
                    work.push_back(Beam::new(down, beam.pos));
                } else if beam.moving_dir == left {
                    work.push_back(Beam::new(up, beam.pos));
                } else if beam.moving_dir == up {
                    work.push_back(Beam::new(left, beam.pos));
                } else {
                    work.push_back(Beam::new(right, beam.pos));
                }
            }
            '.' => {
                work.push_back(beam);
            }
            _ => panic!("Unexpected token: {}", c),
        }
    }

    let res: HashSet<_> = energized_map.iter().map(|x| x.pos).collect();
    res.len()
}

fn part1(input: &str) {
    let grid = parse_input(input);
    let right = (0, 1);

    let beam = Beam::new(right, (0, 0));

    println!("Part1: {}", get_energized_amount(&grid, beam));
}

fn part2(input: &str) {
    let grid = parse_input(input);
    let left = (0, -1);
    let right = (0, 1);
    let up = (-1, 0);
    let down = (1, 0);
    let mut max = 0;

    // top & bottom
    for i in 0..grid[0].len() {
        max = cmp::max(
            max,
            get_energized_amount(&grid, Beam::new(down, (0, i as isize))),
        );

        max = cmp::max(
            max,
            get_energized_amount(&grid, Beam::new(up, (grid.len() as isize - 1, i as isize))),
        );
    }

    // left & right
    for i in 0..grid.len() {
        max = cmp::max(
            max,
            get_energized_amount(&grid, Beam::new(right, (i as isize, 0))),
        );

        max = cmp::max(
            max,
            get_energized_amount(
                &grid,
                Beam::new(left, (i as isize, grid.len() as isize - 1)),
            ),
        )
    }

    println!("Part2: {} ", max);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
