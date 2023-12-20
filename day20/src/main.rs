use num::integer::lcm;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    None,
}
struct Module {
    name: String,
    typ: ModuleType,
    dests: Vec<String>,
    inputs: HashMap<String, u8>,
    state: u8,
}

impl Module {
    pub fn new(name: String, typ: ModuleType, dests: Vec<String>) -> Module {
        Module {
            name,
            typ,
            dests,
            inputs: HashMap::new(),
            state: 0,
        }
    }

    pub fn receive(&mut self, from: String, pulse: u8, work: &mut VecDeque<(String, String, u8)>) {
        match self.typ {
            ModuleType::Broadcast => {
                for dest in self.dests.iter() {
                    work.push_back((self.name.clone(), dest.clone(), pulse));
                }
            }
            ModuleType::FlipFlop => {
                if pulse == 0 {
                    self.state ^= 1;
                    for dest in self.dests.iter() {
                        work.push_back((self.name.clone(), dest.clone(), self.state))
                    }
                }
            }
            ModuleType::Conjunction => {
                self.inputs.get_mut(&from).map(|val| *val = pulse);

                let pulse = if self.inputs.values().all(|&x| x == 1) {
                    0
                } else {
                    1
                };
                for dest in self.dests.iter() {
                    work.push_back((self.name.clone(), dest.clone(), pulse))
                }
            }
            _ => (),
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let parse_regex = Regex::new(r"([%|&]?)(\w+) -> (\w+(?:,\s*\w+)*)").unwrap();
    let mut modules: Vec<Module> = input
        .lines()
        .map(|l| {
            let caps = parse_regex.captures(l).unwrap();
            let groups: Vec<String> = caps
                .iter()
                .skip(1)
                .filter_map(|c| c.map(|m| m.as_str().to_string()))
                .collect();

            let typ = match groups[0].as_str() {
                "%" => ModuleType::FlipFlop,
                "&" => ModuleType::Conjunction,
                "" => ModuleType::Broadcast,
                _ => panic!("unexpected: {}", groups[0]),
            };
            let name = groups[1].clone();
            let dests = groups[2].split(", ").map(|x| x.to_string()).collect();

            Module::new(name, typ, dests)
        })
        .collect();

    let mut missing = Vec::new();
    for m in modules.iter() {
        for n in m.dests.iter() {
            if modules.iter().find(|x| x.name == *n).is_none() {
                missing.push(Module::new(n.clone(), ModuleType::None, Vec::new()));
            }
        }
    }

    modules.append(&mut missing);
    let mut inputs = Vec::new();
    for m in modules.iter() {
        for dest in m.dests.iter() {
            inputs.push((dest.clone(), m.name.clone()));
        }
    }
    let mut modules: HashMap<String, Module> =
        modules.into_iter().map(|m| (m.name.clone(), m)).collect();

    for (node, inp) in inputs.iter() {
        let module = modules.get_mut(node).unwrap();
        module.inputs.insert(inp.clone(), 0);
    }

    modules
}

fn part1(input: &str) {
    let mut modules = parse_input(input);

    let mut lows = 0x0;
    let mut highs = 0x0;

    let mut work = VecDeque::new();
    for _ in 0..1000 {
        work.push_back(("".to_string(), "broadcaster".to_string(), 0));
        while let Some((from, dest, pulse)) = work.pop_front() {
            if pulse == 0 {
                lows += 1;
            } else {
                highs += 1;
            }

            let module = modules.get_mut(&dest).unwrap();
            module.receive(from, pulse, &mut work);
        }
    }

    println!("Part1: {}", lows * highs);
}

fn part2(input: &str) {
    let mut modules = parse_input(input);
    let mut work = VecDeque::new();

    let mut presses = 0x0;

    // Assumption: Node before rx is a conjunction.
    // Plan: Find lcm of the presses it takes for each input module to send a high
    // pulse to the node before rx
    let mut inputs_to_prev_rx = HashMap::new();
    let prev_rx: Vec<&String> = modules[&"rx".to_string()].inputs.keys().collect();
    assert!(prev_rx.len() == 1);
    let prev_rx = &modules[prev_rx[0]];
    for inp in prev_rx.inputs.iter() {
        inputs_to_prev_rx.insert(inp.0.clone(), 0);
    }
    assert!(inputs_to_prev_rx.len() == 4);
    let prev_rx_name = prev_rx.name.clone();

    let mut total_presses = 0x0;

    'outer: loop {
        presses += 1;
        work.push_back(("".to_string(), "broadcaster".to_string(), 0));
        while let Some((from, dest, pulse)) = work.pop_front() {
            if let Some(val) = inputs_to_prev_rx.get_mut(&from) {
                if dest == prev_rx_name && pulse == 1 {
                    *val = presses;
                }
            }

            if inputs_to_prev_rx.values().all(|x| *x > 0) {
                total_presses = inputs_to_prev_rx
                    .values()
                    .fold(1 as u64, |acc, &num| lcm(acc, num));

                break 'outer;
            }

            let module = modules.get_mut(&dest).unwrap();
            module.receive(from, pulse, &mut work);
        }
    }

    println!("Part2: {}", total_presses);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
