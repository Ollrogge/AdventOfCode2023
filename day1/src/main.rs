use std::str; 
use regex::Regex;
use std::collections::HashMap;

fn try_parse_num(line: &str, i: usize) -> Option<u32> {

    let numbers_map: HashMap<&str, u32> = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven",7),
                    ("eight",8), ("nine", 9)].into();

    let mut ret = None;
    for &s in numbers_map.keys() {
        if i + s.len() <= line.len() {
            if line[i..i+s.len()] == *s {
                ret = Some(numbers_map[s]);
                break;
            }
        }

    }

    ret
}

fn part2(input: &str) {
    let mut sum = 0;
    for l in input.lines() {
        let mut digits = Vec::new();
        for (i, c) in l.chars().enumerate() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
            }
            else {
                if let Some(num) = try_parse_num(l, i) {
                    digits.push(num);
                }
            }
        }

        let num = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let num = num.parse::<u32>().unwrap();

        sum += num;
    }

    println!("Part2: {}", sum);

}

fn part1(input: &str) {
    let mut sum = 0;
    for l in input.lines() {
        let digits: Vec<char> = l.chars().filter(|x| x.is_digit(10)).collect();

        let num = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        let num = num.parse::<u32>().unwrap();

        sum += num;
    }

    println!("Part1: {}", sum);
}

fn main() {
    let input = include_bytes!("../input.txt");

    let input = str::from_utf8(input).unwrap();

    part1(input);
    part2(input);
}
