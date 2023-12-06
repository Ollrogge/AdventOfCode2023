use core::num;
use regex::Regex;
use std::iter::zip;
use std::str;

#[derive(Debug)]
struct Race {
    time: usize,   // millisecs
    record: usize, // millimeter
}

impl Race {
    pub fn new(time: usize, record: usize) -> Race {
        Race { time, record }
    }
}

struct Boat {
    hold_time: usize,
    race_length: usize,
}

impl Boat {
    pub fn new(hold_time: usize, race_length: usize) -> Boat {
        Boat {
            hold_time,
            race_length,
        }
    }

    pub fn get_final_distance(&self) -> usize {
        let speed = self.hold_time;
        let time_left = self.race_length - speed;

        speed * time_left
    }
}

fn parse_records(input: &str, part2: bool) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();
    let num_re = Regex::new(r"\d+").unwrap();
    let times: Vec<usize> = num_re
        .find_iter(lines[0])
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect();

    let distances: Vec<usize> = num_re
        .find_iter(lines[1])
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect();

    if part2 {
        let time: String = times.iter().map(|x| x.to_string()).collect();
        let time = time.parse::<usize>().unwrap();

        let distance: String = distances.iter().map(|x| x.to_string()).collect();
        let distance = distance.parse::<usize>().unwrap();
        vec![Race::new(time, distance)]
    } else {
        zip(times, distances)
            .into_iter()
            .map(|(t, d)| Race::new(t, d))
            .collect()
    }
}

fn part1(input: &str) {
    let records = parse_records(input, false);
    let mut nums_per_game = Vec::new();

    for r in records {
        let mut wins = 0x0;
        for i in 0..r.time {
            let boat = Boat::new(i, r.time);
            if boat.get_final_distance() > r.record {
                wins += 1;
            }
        }

        nums_per_game.push(wins);
    }

    println!("Part1 {:?}", nums_per_game.iter().product::<i32>());
}

fn part2(input: &str) {
    let records = parse_records(input, true);
    let mut nums_per_game = Vec::new();

    for r in records {
        let mut wins = 0x0;
        for i in 0..r.time {
            let boat = Boat::new(i, r.time);
            if boat.get_final_distance() > r.record {
                wins += 1;
            }
        }

        nums_per_game.push(wins);
    }

    println!("Part2 {:?}", nums_per_game.iter().product::<i32>());
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
