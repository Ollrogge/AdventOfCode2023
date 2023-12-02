use regex::Regex;
use std::collections::HashMap;
use std::convert::From;
use std::str;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn iterator() -> impl Iterator<Item = Color> {
        [Color::Red, Color::Green, Color::Blue].iter().copied()
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            _ => panic!("Unsupported color: {}", s),
        }
    }
}

struct Game {
    id: u32,
    subsets: Vec<(Color, u32)>,
}

impl Game {
    pub fn new(line: &str) -> Game {
        let line: Vec<&str> = line.split(':').collect();
        let subsets_str = line[1].split(";");

        let id_re = Regex::new(r"Game (\d+)").unwrap();
        let subset_re = Regex::new(r"(\d+)\s+(\w+)").unwrap();

        let id = id_re
            .captures(line[0])
            .and_then(|cap| cap.get(1))
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let mut subsets: Vec<(Color, u32)> = Vec::new();
        for subset in subsets_str {
            for cap in subset_re.captures_iter(subset) {
                let amt = cap[1].to_string().parse::<u32>().unwrap();
                let color = Color::from(&cap[2]);
                subsets.push((color, amt));
            }
        }

        Game { id, subsets }
    }

    pub fn is_valid(&self) -> bool {
        self.subsets.iter().all(|(color, amt)| match color {
            Color::Blue => *amt <= 14,
            Color::Red => *amt <= 12,
            Color::Green => *amt <= 13,
        })
    }

    pub fn fewest_cubes(&self) -> Vec<u32> {
        let mut min_vals = Vec::new();
        for c in Color::iterator() {
            min_vals.push(
                self.subsets
                    .iter()
                    .filter(|(color, _)| *color == c)
                    .map(|(_, a)| a.clone())
                    .max()
                    .unwrap(),
            );
        }

        min_vals
    }
}

fn part1(input: &str) {
    let games: Vec<Game> = input.lines().map(|l| Game::new(l)).collect();
    let amt_valid: u32 = games.iter().filter(|g| g.is_valid()).map(|g| g.id).sum();

    println!("Part1: {}", amt_valid);
}

fn part2(input: &str) {
    let games: Vec<Game> = input.lines().map(|l| Game::new(l)).collect();

    let power: u32 = games
        .iter()
        .map(|g| g.fewest_cubes().iter().product::<u32>())
        .sum();

    println!("Part2: {}", power);
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
