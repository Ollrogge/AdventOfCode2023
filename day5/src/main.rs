use itertools::Itertools;
use regex::Regex;
use std::collections::VecDeque;
use std::{iter, str};

struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn new(mappings: Vec<Mapping>) -> Map {
        Map { mappings }
    }

    pub fn map(&self, source: Range) -> Vec<Range> {
        let mut work = Vec::new();
        let mut mapped_res = Vec::new();
        work.push(source);
        for mapping in self.mappings.iter() {
            let mut new_work = Vec::new();
            while work.len() > 0 {
                let cur = work.pop().unwrap();
                if let Some((mapped, unmapped)) = mapping.try_map(&cur) {
                    mapped_res.push(mapped);
                    if let Some(unmapped) = unmapped {
                        for x in unmapped {
                            new_work.push(x);
                        }
                    }
                } else {
                    new_work.push(cur);
                }
            }
            work = new_work;
        }

        // everything not mapped is identity mapping
        mapped_res.append(&mut work);

        mapped_res
    }
}

struct Mapping {
    source: Range,
    dest: Range,
}

impl Mapping {
    pub fn new(source: Range, dest: Range) -> Mapping {
        Mapping { source, dest }
    }

    pub fn try_map(&self, source: &Range) -> Option<(Range, Option<Vec<Range>>)> {
        // range is in source mapping
        if self.source.contains(source) {
            let start_delta = source.start() - self.source.start();
            let start = self.dest.start() + start_delta;

            let mapped = Range::new(start, source.len());
            Some((mapped, None))
        // source mapping is contained in range
        } else if source.overlaps(&self.source) {
            let (contained, not_contained) = source.map_overlap(&self.source);

            let start_delta = contained.start() - self.source.start();
            let start = self.dest.start() + start_delta;

            let mapped = Range::new(start, contained.len());

            Some((mapped, Some(not_contained)))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn new(start: usize, len: usize) -> Range {
        Range {
            start,
            end: start + len,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn map_overlap(&self, other: &Range) -> (Range, Vec<Range>) {
        let mut not_contained = Vec::new();

        // bigger on both sides
        let mapped = if self.contains(other) {
            not_contained.push(Range::new(self.start(), other.start() - self.start()));
            not_contained.push(Range::new(other.end(), self.end() - other.end()));
            Range::new(other.start(), other.len())
        // left
        } else if self.start < other.start {
            not_contained.push(Range::new(self.start(), other.start() - self.start()));
            Range::new(other.start(), self.end() - other.start())
        // right
        } else {
            not_contained.push(Range::new(other.end(), self.end() - other.end()));
            Range::new(self.start(), other.end() - self.start())
        };

        (mapped, not_contained)
    }

    pub fn left_overlap(&self, other: &Range) -> bool {
        self.start() < other.start() && self.end() >= other.start && self.end() <= other.end()
    }

    pub fn right_overlap(&self, other: &Range) -> bool {
        self.end() > other.end() && self.start() >= other.start() && self.start() <= other.end()
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.left_overlap(other) || self.right_overlap(other) || self.contains(other)
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
}

fn parse_mappings(input: &Vec<&str>, start: usize) -> (Vec<Mapping>, usize) {
    let num_re = Regex::new(r"\d+").unwrap();
    let mut idx = start;
    let mut map = Vec::new();
    while idx < input.len() && input[idx].len() > 0 {
        let numbers: Vec<usize> = num_re
            .find_iter(input[idx])
            .filter_map(|mat| mat.as_str().parse().ok())
            .collect();
        let dest = numbers[0];
        let src = numbers[1];
        let len = numbers[2];

        let src_map = Range::new(src, len);
        let dest_map = Range::new(dest, len);

        map.push(Mapping::new(src_map, dest_map));

        idx += 1;
    }

    (map, idx)
}

fn parse_seeds_part1(input: &Vec<&str>) -> Vec<Range> {
    let num_re = Regex::new(r"\d+").unwrap();
    let idx = 0;
    let seeds: Vec<usize> = num_re
        .find_iter(input[idx])
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect();

    let ranges = seeds.iter().map(|&x| Range::new(x, 0)).collect();

    ranges
}

fn parse_seeds_part2(input: &Vec<&str>) -> Vec<Range> {
    let num_re = Regex::new(r"\d+").unwrap();
    let idx = 0;
    let nums: Vec<usize> = num_re
        .find_iter(input[idx])
        .filter_map(|mat| mat.as_str().parse().ok())
        .collect();

    let mut ranges = Vec::new();

    for (&start, &len) in nums.iter().tuples() {
        ranges.push(Range::new(start, len));
    }

    ranges
}

fn parse_maps(lines: Vec<&str>) -> Vec<Map> {
    let mut idx = 1;
    let mut maps = Vec::new();
    while idx < lines.len() {
        let line = lines[idx];
        if line.contains("map") {
            let (mapppings, idx2) = parse_mappings(&lines, idx + 1);
            maps.push(Map::new(mapppings));
            idx = idx2;
        } else {
            idx += 1;
        }
    }
    maps
}

fn find_lowest_loc_num(start: Vec<Range>, maps: Vec<Map>) -> usize {
    let mut work = start;
    for map in maps {
        let mut new_work = Vec::new();
        for r in work {
            new_work.append(&mut map.map(r));
        }

        work = new_work;
    }

    work.iter().map(|x| x.start()).min().unwrap()
}

fn part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = parse_seeds_part1(&lines);
    let maps = parse_maps(lines);

    println!("Part1: {}", find_lowest_loc_num(seeds, maps));
}

fn part2(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = parse_seeds_part2(&lines);
    let maps = parse_maps(lines);

    println!("Part2: {}", find_lowest_loc_num(seeds, maps));
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}

// 137718409
// 20191102
