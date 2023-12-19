use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    category: String,
    operation: String,
    value: isize,
}

impl Rule {
    pub fn new(category: String, operation: String, value: isize) -> Rule {
        Rule {
            category,
            operation,
            value,
        }
    }

    fn apply(&self, part: &Part) -> bool {
        for (category, val) in part.categories.iter() {
            if *category == self.category {
                return match self.operation.as_str() {
                    "<" => *val < self.value,
                    ">" => *val > self.value,
                    _ => panic!("Unhandled operation in rule: {}", self.operation),
                };
            }
        }
        false
    }

    fn invert(&self) -> Rule {
        let op = match self.operation.as_str() {
            ">" => "<",
            "<" => ">",
            _ => panic!(""),
        };

        let val = match self.operation.as_str() {
            "<" => self.value - 1,
            ">" => self.value + 1,
            _ => panic!(""),
        };

        Rule {
            category: self.category.clone(),
            operation: op.to_string(),
            value: val,
        }
    }

    fn get_amount_matching(&self) -> usize {
        match self.operation.as_str() {
            "<" => self.value as usize - 1,
            ">" => 4000 - self.value as usize,
            _ => panic!(""),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<(Option<Rule>, String)>,
}

impl Workflow {
    pub fn new(name: String, rules: Vec<(Option<Rule>, String)>) -> Workflow {
        Workflow { name, rules }
    }

    fn get_next(&self, part: &Part) -> String {
        let next = self.rules.iter().find(|r| {
            if let Some(r) = &r.0 {
                r.apply(part)
            } else {
                true
            }
        });

        next.unwrap().1.clone()
    }
}

#[derive(Debug)]
struct Part {
    categories: Vec<(String, isize)>,
}

impl Part {
    pub fn new(categories: Vec<(String, isize)>) -> Part {
        Part { categories }
    }
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let condition_regex = Regex::new(r"([a-zA-Z]+)(<|>)?(\d+)?(?::([a-zA-Z]+))?").unwrap();
    let workflows: Vec<Workflow> = parts[0]
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split("{").collect();
            let name = parts[0].to_string();
            let mut rules = Vec::new();
            for caps in condition_regex.captures_iter(parts[1]) {
                let groups: Vec<String> = caps
                    .iter()
                    .skip(1)
                    .filter_map(|c| c.map(|m| m.as_str().to_string()))
                    .collect();

                if groups.len() == 4 {
                    let cat = groups[0].clone();
                    let op = groups[1].clone();
                    let val = groups[2].parse::<isize>().unwrap();
                    let next = groups[3].clone();
                    let rule = Rule::new(cat, op, val);
                    rules.push((Some(rule), next));
                } else if groups.len() == 1 {
                    let next = groups[0].clone();
                    rules.push((None, next));
                } else {
                    panic!("Unexpected match group length");
                }
            }
            Workflow::new(name, rules)
        })
        .collect();

    let parts_regex = Regex::new(r"([a-zA-Z]+)=([0-9]+)").unwrap();
    let parts: Vec<Part> = parts[1]
        .lines()
        .map(|l| {
            let mut categories = Vec::new();
            for cap in parts_regex.captures_iter(l) {
                let name = cap[1].to_string();
                let val = cap[2].parse::<isize>().unwrap();
                categories.push((name, val));
            }
            Part::new(categories)
        })
        .collect();

    let mut workflow_map = HashMap::new();
    for flow in workflows.into_iter() {
        workflow_map.insert(flow.name.clone(), flow);
    }

    (workflow_map, parts)
}

fn part1(input: &str) {
    let (workflows, parts) = parse_input(input);
    let mut sum = 0x0;
    for part in parts.iter() {
        let mut cur = &workflows["in"];
        loop {
            let next = cur.get_next(part);

            if next == "A" {
                sum += part.categories.iter().map(|c| c.1).sum::<isize>();
            }

            if next == "A" || next == "R" {
                break;
            }

            cur = &workflows[&next];
        }
    }

    println!("Part1: {}", sum);
}

fn part2(input: &str) {
    let (workflows, _) = parse_input(input);
    let mut work = VecDeque::new();
    let mut solutions: Vec<Vec<Rule>> = Vec::new();
    work.push_back((&workflows["in"], Vec::new()));
    while let Some((workflow, rules)) = work.pop_front() {
        for (rule, next) in workflow.rules.iter() {
            if let Some(rule) = &rule {
                let mut new_rules = rules.clone();
                // all rules until now did not match
                for (rule2, _) in workflow.rules.iter() {
                    let rule2 = rule2.as_ref().unwrap();
                    if *rule2 == *rule {
                        break;
                    }

                    new_rules.push(rule2.invert());
                }
                new_rules.push(rule.clone());

                if next == "A" {
                    solutions.push(new_rules);
                } else if next != "R" {
                    work.push_back((&workflows[next], new_rules))
                }
            } else {
                let mut new_rules = rules.clone();
                for (rule, _) in workflow.rules.iter() {
                    if let Some(rule) = rule {
                        new_rules.push(rule.invert())
                    }
                }
                if next == "A" {
                    solutions.push(new_rules.clone());
                } else if next != "R" {
                    // none of the rules matched
                    work.push_back((&workflows[next], new_rules))
                }
            }
        }
    }

    let mut new_solutions: Vec<Vec<usize>> = Vec::new();
    let categories = vec!["x", "m", "a", "s"];
    for sol in solutions.iter() {
        let mut solution: Vec<usize> = Vec::new();
        for &cat in categories.iter() {
            let same_category: Vec<&Rule> =
                sol.iter().filter(|&x| x.category.as_str() == cat).collect();
            if same_category.len() > 1 {
                let mut bigger: Vec<&&Rule> = same_category
                    .iter()
                    .filter(|&x| x.operation == ">")
                    .collect();

                // descending
                bigger.sort_by(|a, b| b.value.cmp(&a.value));

                let mut smaller: Vec<&&Rule> = same_category
                    .iter()
                    .filter(|&x| x.operation == "<")
                    .collect();

                // ascending
                smaller.sort_by(|a, b| a.value.cmp(&b.value));

                if bigger.len() > 0 && smaller.len() > 0 {
                    assert!(smaller[0].value > bigger[0].value);
                    solution.push((smaller[0].value - bigger[0].value - 1) as usize);
                } else if bigger.len() > 0 {
                    solution.push(bigger[0].get_amount_matching())
                } else if smaller.len() > 0 {
                    solution.push(smaller[0].get_amount_matching())
                }
            } else if same_category.len() == 1 {
                solution.push(same_category[0].get_amount_matching());
            } else {
                solution.push(4000);
            }
        }
        assert!(solution.len() == 4);
        new_solutions.push(solution)
    }

    let sum = new_solutions
        .iter()
        .map(|s| s.iter().product::<usize>())
        .sum::<usize>();

    println!("Part2: {:?}", sum);
}
fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
