use itertools::Itertools;
use regex::Regex;
use std::str;

fn parse_values(input: &str) -> Vec<Vec<i64>> {
    let num_re = Regex::new(r"-?\d+").unwrap();
    input
        .lines()
        .map(|line| {
            num_re
                .find_iter(line)
                .filter_map(|m| m.as_str().parse::<i64>().ok())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn calc_next_value(l: Vec<i64>) -> i64 {
    if l.iter().all(|x| *x == 0) {
        return 0;
    }

    let diffs: Vec<i64> = l.windows(2).map(|w| w[1] - w[0]).collect();
    l.last().unwrap() + calc_next_value(diffs)
}

fn part1(input: &str) {
    let nums = parse_values(input);
    let new_vals: Vec<i64> = nums.into_iter().map(|v| calc_next_value(v)).collect();

    println!("Part1: {}", new_vals.iter().sum::<i64>())
}

fn part2(input: &str) {
    let nums = parse_values(input);
    let new_vals: Vec<i64> = nums
        .into_iter()
        .map(|mut v| {
            v.reverse();
            calc_next_value(v)
        })
        .collect();

    println!("Part1: {}", new_vals.iter().sum::<i64>())
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
