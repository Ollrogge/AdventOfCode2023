use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> HashSet<(String, String)> {
    let mut ret = HashSet::new();
    for l in input.lines() {
        let parts: Vec<&str> = l.split(": ").collect();
        let machine = parts[0];
        let connections: Vec<&str> = parts[1].split(" ").collect();

        for x in connections {
            ret.insert((machine.to_string(), x.to_string()));
            ret.insert((x.to_string(), machine.to_string()));
        }
    }

    ret
}

fn two_groups(connections: HashMap<String, HashSet<String>>) -> Option<usize> {
    let mut work = VecDeque::from([connections.keys().next().unwrap()]);
    let mut seen = HashSet::new();
    while let Some(w) = work.pop_front() {
        seen.insert(w);
        for next in connections[w].iter() {
            if !seen.contains(next) {
                work.push_back(&next);
            }
        }
    }

    let not_in_seen = connections.keys().filter(|k| !seen.contains(k)).count();

    if not_in_seen > 0 {
        Some(not_in_seen * seen.len())
    } else {
        None
    }
}

fn dijkstra(
    map: &HashMap<String, HashSet<String>>,
    start: &String,
    end: &String,
) -> Option<Vec<(String, String)>> {
    let mut work = BinaryHeap::from([Reverse((0, start.clone(), Vec::new()))]);
    let mut sol: Option<Vec<(String, String)>> = None;
    let mut g_seen = HashSet::new();
    while let Some(Reverse((steps, pos, seen))) = work.pop() {
        if !g_seen.insert(pos.clone()) {
            continue;
        }
        if pos == *end {
            sol = Some(seen.clone());
            break;
        }
        for next in map[&pos].iter() {
            let mut seen = seen.clone();
            seen.push((pos.clone(), next.clone()));
            work.push(Reverse((steps + 1, next.clone(), seen)));
        }
    }

    sol
}

fn part1(input: &str) {
    let connections = parse_input(input);
    let mut map = HashMap::new();
    for c in connections.into_iter() {
        map.entry(c.0.clone())
            .or_insert_with(HashSet::new)
            .insert(c.1.clone());
        map.entry(c.1).or_insert_with(HashSet::new).insert(c.0);
    }

    let mut edge_counter = HashMap::new();
    for (a, b) in map.keys().tuples() {
        let sp = dijkstra(&map, a, b);
        if let Some(sp) = sp {
            for e in sp {
                let (a, b) = e;
                let edge = if a < b { (a, b) } else { (b, a) };
                *edge_counter.entry(edge).or_insert(0) += 1;
            }
        }
    }

    let mut pairs: Vec<_> = edge_counter.iter().collect();
    pairs.sort_unstable_by(|a, b| b.1.cmp(a.1));
    let mut cnt = 0x0;
    for w in pairs.windows(3) {
        cnt += 1;
        let mut map = map.clone();
        for (e, _) in w {
            let (a, b) = e;
            if let Some(set) = map.get_mut(a) {
                set.remove(b);
            }
            if let Some(set) = map.get_mut(b) {
                set.remove(a);
            }
        }
        if let Some(sol) = two_groups(map) {
            println!("Part1: {:?} {}", sol, cnt);
            break;
        }
    }
}
fn main() {
    let input = include_str!("../input.txt");

    part1(input);
}
