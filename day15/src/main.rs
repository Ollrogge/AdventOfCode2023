use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str;

fn parse_input(input: &str) -> Vec<String> {
    let input = input.replace("\n", "");
    input.split(',').map(String::from).collect()
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, x| {
        let mut tmp = acc + x as u8 as u32;
        tmp *= 17;
        tmp = tmp % 256;

        tmp
    })
}

fn part2(input: &str) {
    let input = parse_input(input);
    let mut map: HashMap<u32, Vec<(String, u8)>> = HashMap::new();
    for inp in input.iter() {
        match inp.contains("=") {
            true => {
                let parts: Vec<&str> = inp.split("=").collect();
                let label = parts[0].to_string();
                let s = parts[1].parse::<u8>().unwrap();

                match map.entry(hash(&label)) {
                    Entry::Occupied(mut entry) => {
                        if let Some(pos) = entry.get().iter().position(|x| x.0 == label) {
                            entry.get_mut()[pos].1 = s;
                        } else {
                            entry.get_mut().push((label, s))
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![(label, s)]);
                    }
                }
            }
            false => {
                let parts: Vec<&str> = inp.split("-").collect();
                let label = parts[0].to_string();
                if let Some(val) = map.get_mut(&hash(&label)) {
                    if let Some(pos) = val.iter().position(|x| x.0 == label) {
                        val.remove(pos);
                    }
                }
            }
        }
    }

    let mut sum = 0x0;
    for i in 0..256 {
        if let Some(val) = map.get(&i) {
            for j in 0..val.len() {
                sum += (i + 1) * (j as u32 + 1) * val[j].1 as u32;
            }
        }
    }

    println!("Part2: {}", sum);
}

fn part1(input: &str) {
    let input = parse_input(input);
    let sum: u32 = input.iter().map(|x| hash(x)).sum();

    println!("Part1: {}", sum);
}
fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
