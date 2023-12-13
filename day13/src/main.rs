use itertools::Itertools;
use std::cmp;
use std::str;

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            print!("{}", grid[row][col]);
        }
        println!("");
    }
    println!("");
}

struct Grid {
    nodes: Vec<Vec<char>>,
}

impl Grid {
    fn get_vertical_reflections(&self, part2: bool) -> i32 {
        let rows = self.nodes.len() as i32;
        let cols = self.nodes[0].len() as i32;
        let mut res = 0;
        let grid = &self.nodes;

        for c in 0..cols - 1 {
            let mut diffs = 0;
            for dc in 0..cols - 1 {
                let left = c as i32 - dc as i32;
                let right = c + 1 as i32 + dc as i32;
                if left >= 0 && right < cols {
                    for r in 0..rows {
                        if grid[r as usize][left as usize] != grid[r as usize][right as usize] {
                            diffs += 1;
                        }
                    }
                }
            }

            if diffs == 0 && !part2 || diffs == 1 && part2 {
                res += c + 1;
            }
        }

        res
    }

    fn get_horizontal_reflections(&self, part2: bool) -> i32 {
        let grid = &self.nodes;
        let rows = self.nodes.len() as i32;
        let cols = self.nodes[0].len() as i32;
        let mut res = 0;

        for r in 0..rows - 1 {
            let mut diffs = 0x0;
            for dr in 0..rows - 1 {
                let above = r as i32 - dr as i32;
                let below = r + 1 as i32 + dr as i32;
                if above >= 0 && below < rows {
                    for c in 0..cols {
                        if grid[above as usize][c as usize] != grid[below as usize][c as usize] {
                            diffs += 1;
                        }
                    }
                }
            }

            if diffs == 0 && !part2 || diffs == 1 && part2 {
                res += 100 * (r + 1);
            }
        }

        res
    }
}

fn parse_input(input: &str) -> Vec<Grid> {
    let input = input.split("\n\n");
    let mut grids = Vec::new();
    for g in input {
        let nodes = g.lines().map(|l| l.chars().collect()).collect();
        grids.push(Grid { nodes })
    }

    grids
}

fn part1(input: &str) {
    let grids = parse_input(input);
    let res: i32 = grids
        .iter()
        .map(|g| {
            let mut res = 0;
            res += g.get_horizontal_reflections(false);
            res += g.get_vertical_reflections(false);

            res
        })
        .sum();

    println!("Part1: {}", res)
}

fn part2(input: &str) {
    let grids = parse_input(input);
    let res: i32 = grids
        .iter()
        .map(|g| {
            let mut res = 0;
            res += g.get_horizontal_reflections(true);
            res += g.get_vertical_reflections(true);

            res
        })
        .sum();

    println!("Part2: {}", res)
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
