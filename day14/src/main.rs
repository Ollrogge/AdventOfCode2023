use std::str;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn do_north_tilt(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut old_pos = Vec::new();
        for row in (0..grid.len()).rev() {
            let c = grid[row][col];
            if c == 'O' {
                old_pos.push(row);
            } else if c == '#' {
                for &i in old_pos.iter() {
                    grid[i][col] = '.';
                }
                for i in 1..=old_pos.len() {
                    grid[row + i][col] = 'O';
                }
                old_pos.clear();
            }
        }

        for &i in old_pos.iter() {
            grid[i][col] = '.';
        }
        for i in 0..old_pos.len() {
            grid[i][col] = 'O';
        }
    }
}

fn do_south_tilt(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut old_pos = Vec::new();
        for row in 0..grid.len() {
            let c = grid[row][col];
            if c == 'O' {
                old_pos.push(row);
            } else if c == '#' {
                for &i in old_pos.iter() {
                    grid[i][col] = '.';
                }
                for i in 1..=old_pos.len() {
                    grid[row - i][col] = 'O';
                }
                old_pos.clear();
            }
        }

        for &i in old_pos.iter() {
            grid[i][col] = '.';
        }
        let rows = grid.len();
        for i in 1..=old_pos.len() {
            grid[rows - i][col] = 'O';
        }
    }
}

fn do_east_tilt(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut old_pos = Vec::new();
        for col in 0..grid[0].len() {
            let c = grid[row][col];
            if c == 'O' {
                old_pos.push(col);
            } else if c == '#' {
                for &i in old_pos.iter() {
                    grid[row][i] = '.';
                }
                for i in 1..=old_pos.len() {
                    grid[row][col - i] = 'O';
                }

                old_pos.clear();
            }
        }

        for &i in old_pos.iter() {
            grid[row][i] = '.';
        }
        let cols = grid[0].len();
        for i in 1..=old_pos.len() {
            grid[row][cols - i] = 'O';
        }
    }
}

fn do_west_tilt(grid: &mut Vec<Vec<char>>) {
    for row in 0..grid.len() {
        let mut old_pos = Vec::new();
        for col in (0..grid[0].len()).rev() {
            let c = grid[row][col];
            if c == 'O' {
                old_pos.push(col);
            } else if c == '#' {
                for &i in old_pos.iter() {
                    grid[row][i] = '.';
                }
                for i in 1..=old_pos.len() {
                    grid[row][col + i] = 'O';
                }
                old_pos.clear();
            }
        }

        for &i in old_pos.iter() {
            grid[row][i] = '.';
        }
        for i in 0..old_pos.len() {
            grid[row][i] = 'O';
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            print!("{}", grid[row][col]);
        }
        println!("")
    }
    println!("")
}

fn do_spin(grid: &mut Vec<Vec<char>>) {
    do_north_tilt(grid);
    do_west_tilt(grid);
    do_south_tilt(grid);
    do_east_tilt(grid);
}

fn get_load(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0x0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'O' {
                sum += grid.len() - row;
            }
        }
    }

    sum
}

fn part1(input: &str) {
    let mut grid = parse_input(input);
    do_north_tilt(&mut grid);

    println!("Part1: {}", get_load(&grid));
}

fn part2(input: &str) {
    let mut grid = parse_input(input);
    for _ in 0..1000 {
        do_spin(&mut grid);
    }

    println!("Part2: {}", get_load(&grid));
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
