use std::collections::{HashMap, VecDeque};
use std::str;

struct Card {
    number: usize,
    matching: Vec<usize>,
    cnt: usize,
}

impl Card {
    pub fn new(number: usize, matching: Vec<usize>) -> Card {
        Card {
            number,
            matching,
            cnt: 1,
        }
    }

    pub fn inc_cnt(&mut self) {
        self.cnt += 1;
    }

    pub fn inc_cnt_by(&mut self, amt: usize) {
        self.cnt += amt;
    }

    pub fn get_cnt(&self) -> usize {
        self.cnt
    }
}

fn parse_numbers(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

fn part1(input: &str) {
    let mut points = 0;
    for l in input.lines() {
        let l = l.splitn(2, ':').nth(1).unwrap().trim();
        let parts: Vec<&str> = l.split("|").collect();

        let winning = parse_numbers(parts[0]);
        let i_have = parse_numbers(parts[1]);

        let i_have: Vec<&usize> = i_have.iter().filter(|x| winning.contains(x)).collect();

        if i_have.len() > 0 {
            points += 2_u32.pow(i_have.len().saturating_sub(1) as u32);
        }
    }

    println!("Part1: {}", points);
}

fn part2(input: &str) {
    let mut cards: HashMap<usize, Card> = HashMap::new();
    let mut card_queue: VecDeque<usize> = VecDeque::new();
    for l in input.lines() {
        let l: Vec<&str> = l.splitn(2, ':').collect();
        let card_num = l[0]
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse::<usize>().ok())
            .nth(0)
            .unwrap();
        let parts: Vec<&str> = l[1].trim().split("|").collect();

        let winning = parse_numbers(parts[0]);
        let i_have = parse_numbers(parts[1]);

        let matching = i_have
            .iter()
            .filter(|&x| winning.contains(x))
            .map(|&x| x)
            .collect();

        let card = Card::new(card_num, matching);

        cards.insert(card_num, card);

        card_queue.push_back(card_num);
    }

    while let Some(card_num) = card_queue.pop_front() {
        let matching_cnt = cards[&card_num].matching.len();
        if matching_cnt == 0 {
            continue;
        }
        let card_cnt = cards[&card_num].cnt;
        for match_num in (card_num + 1)..=(card_num + matching_cnt) {
            let e = cards.get_mut(&match_num).unwrap();
            e.inc_cnt_by(card_cnt);
        }
    }

    let total: usize = cards.values().map(|x| x.get_cnt()).sum();

    println!("Part2: {}", total);
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
