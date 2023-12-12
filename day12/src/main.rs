use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::iter::zip;
use std::str;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut spring_info: Vec<Vec<char>> = Vec::new();
    let mut groups = Vec::new();
    let num_re = Regex::new(r"\d+").unwrap();

    for l in input.lines() {
        let parts: Vec<&str> = l.split(" ").collect();
        spring_info.push(parts[0].chars().collect());
        groups.push(
            num_re
                .find_iter(l)
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .collect(),
        );
    }

    (spring_info, groups)
}

fn is_valid(line: Vec<char>, groups: &Vec<usize>) -> bool {
    let mut spring_groups = Vec::new();
    let mut group_size = 0x0;
    for &x in line.iter() {
        if x == '#' {
            group_size += 1;
        } else {
            if group_size > 0 {
                spring_groups.push(group_size);
                group_size = 0;
            }
        }
    }

    if group_size > 0 {
        spring_groups.push(group_size);
    }

    spring_groups == *groups
}

// index of current line, index of current block, current spring len
fn get_arrangements_line(
    line: &Vec<char>,
    groups: &Vec<usize>,
    l_i: usize,
    g_i: usize,
    s_len: usize,
    state: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&cnt) = state.get(&(l_i, g_i, s_len)) {
        return cnt;
    }
    if l_i == line.len() {
        if g_i == groups.len() && s_len == 0 {
            return 1;
        } else if g_i == groups.len() - 1 && s_len == groups[g_i] {
            return 1;
        } else {
            return 0;
        }
    }

    let mut ret = 0;

    let chars = vec!['.', '#'];
    let cur = line[l_i];

    for &c in chars.iter() {
        if cur == c || cur == '?' {
            if c == '.' && s_len == 0 {
                ret += get_arrangements_line(line, groups, l_i + 1, g_i, 0, state);
            } else if c == '.' && s_len > 0 && g_i < groups.len() && groups[g_i] == s_len {
                ret += get_arrangements_line(line, groups, l_i + 1, g_i + 1, 0, state);
            } else if c == '#' {
                ret += get_arrangements_line(line, groups, l_i + 1, g_i, s_len + 1, state)
            }
        }
    }

    state.insert((l_i, g_i, s_len), ret);

    ret
}

fn part1(input: &str) {
    let (infos, groups) = parse_input(input);

    let mut state = HashMap::new();

    let sum: usize = zip(infos, groups)
        .map(|i| {
            state.clear();
            get_arrangements_line(&i.0, &i.1, 0, 0, 0, &mut state)
        })
        .sum();

    println!("Part1: {}", sum);
}

fn part2(input: &str) {
    let (infos, groups) = parse_input(input);

    let mut sum = 0;
    let mut state = HashMap::new();

    for (info, group) in zip(infos, groups) {
        let mut new_info = Vec::new();
        let mut other = info.clone();
        other.push('?');
        for _ in 0..4 {
            new_info.extend(other.clone());
        }
        new_info.extend(info);

        let mut new_group = Vec::new();
        for _ in 0..5 {
            new_group.extend(group.clone());
        }

        sum += get_arrangements_line(&new_info, &new_group, 0, 0, 0, &mut state);
        state.clear();
    }

    println!("Part2: {}", sum);
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
