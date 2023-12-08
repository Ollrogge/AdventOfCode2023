use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::str;

fn parse_instructions(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Unexpected instruction"),
        })
        .collect()
}

fn parse_map(lines: Vec<&str>) -> HashMap<&str, (&str, &str)> {
    let w_regex = Regex::new(r"\w+").unwrap();
    let mut map = HashMap::new();
    for l in lines.iter().skip(2) {
        let words: Vec<&str> = w_regex.find_iter(l).map(|m| m.as_str()).collect();
        map.insert(words[0], (words[1], words[2]));
    }

    map
}

fn part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let insts = parse_instructions(lines[0]);
    let map = parse_map(lines);

    let mut pos = "AAA";
    let dest = "ZZZ";
    let mut steps = 0;

    'outer: loop {
        for inst in insts.iter() {
            if *inst == 1 {
                pos = map[pos].1;
            } else {
                pos = map[pos].0;
            }

            steps += 1;

            if pos == dest {
                break 'outer;
            }
        }
    }

    println!("Part1: {}", steps);
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let insts = parse_instructions(lines[0]);
    let map = parse_map(lines);

    let positions: Vec<&str> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|&c| c)
        .collect();

    let mut steps: Vec<usize> = vec![0; positions.len()];

    for (i, start) in positions.iter().enumerate() {
        let mut pos = *start;
        'outer: loop {
            for inst in insts.iter() {
                if *inst == 1 {
                    pos = map[pos].1;
                } else {
                    pos = map[pos].0;
                }

                steps[i] += 1;

                if pos.ends_with('Z') {
                    break 'outer;
                }
            }
        }
    }
    let steps = steps.iter().fold(1, |acc, &num| lcm(acc, num));

    println!("Part2: {}", steps);
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
