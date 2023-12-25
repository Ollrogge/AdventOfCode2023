use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize), (isize, isize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid[0]
        .iter()
        .enumerate()
        .find(|(i, &x)| x == '.')
        .map(|(i, x)| i)
        .unwrap();
    let end = grid[grid.len() - 1]
        .iter()
        .enumerate()
        .find(|(i, &x)| x == '.')
        .map(|(i, x)| i)
        .unwrap();

    let grid_len = grid.len() as isize;

    (grid, (0, start as isize), (grid_len - 1, end as isize))
}

struct State {
    steps: usize,
    pos: (isize, isize),
    seen: HashSet<(isize, isize)>,
}

fn compress_grid(
    grid: Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
    part2: bool,
) -> HashMap<(isize, isize), HashSet<(isize, isize, usize)>> {
    let mut vertices = HashSet::new();
    let mut edges = HashMap::new();
    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let slopes = HashMap::from([('>', (0, 1)), ('<', (0, -1)), ('^', (-1, 0)), ('v', (1, 0))]);

    vertices.insert(start);
    vertices.insert(end);

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '#' {
                continue;
            }
            let mut outgoing_edges = 0x0;
            for d in dirs.iter() {
                let row = row as isize + d.0;
                let col = col as isize + d.1;
                if row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize
                {
                    if grid[row as usize][col as usize] != '#' {
                        outgoing_edges += 1;
                    }
                }
            }
            if outgoing_edges > 2 {
                vertices.insert((row as isize, col as isize));
            }
        }
    }

    for &v in vertices.iter() {
        let mut work = VecDeque::new();
        work.push_back(State {
            steps: 0,
            pos: v,
            seen: HashSet::from([v]),
        });

        while let Some(state) = work.pop_back() {
            if vertices.contains(&state.pos) && state.pos != v {
                edges.entry(v).or_insert_with(HashSet::new).insert((
                    state.pos.0,
                    state.pos.1,
                    state.steps,
                ));
                continue;
            }

            let c = grid[state.pos.0 as usize][state.pos.1 as usize];

            assert!(c != '#');

            for &d in dirs.iter() {
                let row = state.pos.0 as isize + d.0;
                let col = state.pos.1 as isize + d.1;
                if row >= 0 && row < grid.len() as isize && col >= 0 && col < grid[0].len() as isize
                {
                    if grid[row as usize][col as usize] != '#' && !state.seen.contains(&(row, col))
                    {
                        if !part2 && slopes.contains_key(&c) && d != slopes[&c] {
                            continue;
                        }
                        let mut seen = state.seen.clone();
                        seen.insert((row, col));
                        work.push_back(State {
                            steps: state.steps + 1,
                            pos: (row, col),
                            seen,
                        });
                    }
                }
            }
        }
    }

    edges
}

fn dfs(
    node: &(isize, isize),
    end: &(isize, isize),
    graph: &HashMap<(isize, isize), HashSet<(isize, isize, usize)>>,
    seen: &mut HashSet<(isize, isize)>,
    current_steps: usize,
    max_steps: &mut usize,
) {
    if node == end {
        *max_steps = (*max_steps).max(current_steps);
        return;
    }

    if !seen.insert(*node) {
        return;
    }

    if let Some(outgoing) = graph.get(node) {
        for &(next_row, next_col, dist) in outgoing {
            let next_node = (next_row, next_col);
            dfs(
                &next_node,
                end,
                graph,
                seen,
                current_steps + dist,
                max_steps,
            );
        }
    }

    seen.remove(node); // Backtrack
}

fn part1(input: &str) {
    let (grid, start, end) = parse_input(input);
    let grid = compress_grid(grid, start, end, false);
    let mut seen = HashSet::new();
    let mut max_steps = 0x0;
    dfs(&start, &end, &grid, &mut seen, 0, &mut max_steps);

    println!("Part1: {}", max_steps);
}

fn part2(input: &str) {
    let (grid, start, end) = parse_input(input);
    let grid = compress_grid(grid, start, end, true);
    let mut seen = HashSet::new();
    let mut max_steps = 0x0;
    dfs(&start, &end, &grid, &mut seen, 0, &mut max_steps);

    println!("Part2: {}", max_steps);
}

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
    part2(input);
}
