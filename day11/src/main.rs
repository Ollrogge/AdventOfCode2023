use itertools::Itertools;
use std::str;

fn parse_image(input: &str) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let img: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut empty_rows = Vec::new();
    let mut empty_columns = Vec::new();

    for row in 0..img.len() {
        if img[row].iter().all(|&x| x == '.') {
            empty_rows.push(row);
        }
    }

    for col in 0..img[0].len() {
        let mut empty = true;
        for row in 0..img.len() {
            if img[row][col] != '.' {
                empty = false;
                break;
            }
        }

        if empty {
            empty_columns.push(col);
        }
    }
    (img, empty_rows, empty_columns)
}

fn get_galaxy_coords(img: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for row in 0..img.len() {
        for col in 0..img[0].len() {
            if img[row][col] == '#' {
                galaxies.push((row, col))
            }
        }
    }

    galaxies
}

fn part1(input: &str) {
    let (img, empty_rows, empty_cols) = parse_image(input);
    let mut galaxies = get_galaxy_coords(&img);

    for g in galaxies.iter_mut() {
        let rows_smaller: usize = empty_rows.iter().filter(|&&i| i < g.0).count();
        let cols_smaller: usize = empty_cols.iter().filter(|&&i| i < g.1).count();

        g.0 += rows_smaller * 1;
        g.1 += cols_smaller * 1;
    }

    let sum = galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
        })
        .sum::<i64>();

    println!("Part1: {}", sum);
}

fn part2(input: &str) {
    let (img, empty_rows, empty_cols) = parse_image(input);
    let mut galaxies = get_galaxy_coords(&img);

    for g in galaxies.iter_mut() {
        let rows_smaller: usize = empty_rows.iter().filter(|&&i| i < g.0).count();
        let cols_smaller: usize = empty_cols.iter().filter(|&&i| i < g.1).count();

        g.0 += rows_smaller * (10_usize.pow(6) - 1);
        g.1 += cols_smaller * (10_usize.pow(6) - 1);
    }

    let sum = galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let (a, b) = (pair[0], pair[1]);
            (a.0 as i64 - b.0 as i64).abs() as usize + (a.1 as i64 - b.1 as i64).abs() as usize
        })
        .sum::<usize>();

    println!("Part2: {}", sum);
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
