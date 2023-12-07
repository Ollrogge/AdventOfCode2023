use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::str;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Card {
    value: usize,
    is_joker: bool,
}

impl Card {
    pub fn from(c: char, part2: bool) -> Card {
        let value = match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => {
                if part2 {
                    1
                } else {
                    11
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unexpected char: {}", c),
        };

        Card {
            value,
            is_joker: c == 'J' && part2,
        }
    }

    pub fn is_joker(&self) -> bool {
        self.is_joker
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
enum CardType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    typ: CardType,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bid: usize) -> Hand {
        let mut counts = HashMap::new();
        let joker_cnt = cards.iter().filter(|x| x.is_joker()).count();
        let cards_for_typ: Vec<&Card> = cards.iter().filter(|c| !c.is_joker()).collect();

        for &item in cards_for_typ.iter() {
            *counts.entry(item).or_insert(0) += 1;
        }

        let mut count_vec: Vec<usize> = counts.values().cloned().collect();
        count_vec.sort_unstable_by(|a, b| b.cmp(a));

        if count_vec.len() == 0 {
            count_vec.push(5);
        } else {
            count_vec[0] += joker_cnt;
        }

        let typ = match (count_vec.get(0), count_vec.get(1)) {
            (Some(&5), _) => CardType::FiveOfKind,
            (Some(&4), _) => CardType::FourOfKind,
            (Some(&3), Some(&2)) => CardType::FullHouse,
            (Some(&3), _) => CardType::ThreeOfKind,
            (Some(&2), Some(&2)) => CardType::TwoPair,

            (Some(&2), _) => CardType::OnePair,
            _ => CardType::HighCard,
        };

        Hand { cards, bid, typ }
    }

    pub fn get_bid(&self) -> usize {
        self.bid
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.typ == other.typ
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.typ.partial_cmp(&other.typ) {
            Some(Ordering::Equal) => {
                for (a, b) in zip(&self.cards, &other.cards) {
                    match a.cmp(b) {
                        Ordering::Equal => continue,
                        non_equal => return Some(non_equal),
                    }
                }
                Some(Ordering::Greater)
            }
            other => other,
        }
    }
}

fn parse_hands(input: &str, part2: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    for l in input.lines() {
        let parts: Vec<&str> = l.split_whitespace().take(2).collect();
        let bid = parts[1].parse::<usize>().unwrap();
        let cards: Vec<Card> = parts[0].chars().map(|c| Card::from(c, part2)).collect();

        hands.push(Hand::new(cards, bid));
    }

    hands
}

fn part1(input: &str) {
    let mut hands = parse_hands(&input, false);
    hands.sort();

    let mut earnings = 0;

    for (rank, hand) in hands.iter().enumerate() {
        earnings += (rank + 1) * hand.get_bid();
    }

    println!("Part1: {}", earnings);
}

fn part2(input: &str) {
    let mut hands = parse_hands(&input, true);
    hands.sort();

    let mut earnings = 0;

    for (rank, hand) in hands.iter().enumerate() {
        earnings += (rank + 1) * hand.get_bid();
    }

    println!("Part2: {}", earnings);
}
fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    part1(input);
    part2(input);
}
