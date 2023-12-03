use std::str;

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

#[derive(Clone)]
struct Part {
    adjacent_symbol: Symbol,
    number: u32,
}

#[derive(PartialEq, Clone)]
struct Symbol {
    row: usize,
    column: usize,
    val: char,
}

impl Symbol {
    pub fn new(row: usize, column: usize, val: char) -> Symbol {
        Symbol { row, column, val }
    }

    pub fn is_gear(&self) -> bool {
        self.val == '*'
    }
}

impl Part {
    pub fn new(adjacent_symbol: Symbol, number: u32) -> Part {
        Part {
            adjacent_symbol,
            number,
        }
    }

    pub fn is_gear(&self) -> bool {
        self.adjacent_symbol.is_gear()
    }

    pub fn get_num(&self) -> u32 {
        self.number
    }

    pub fn get_symbol(&self) -> &Symbol {
        &self.adjacent_symbol
    }
}

fn has_surrounding_symbol(i: usize, j: usize, schematic: &Vec<Vec<char>>) -> Option<Symbol> {
    let row_min = i.saturating_sub(1);
    let row_max = usize::min(i + 1, schematic.len().saturating_sub(1));

    let col_min = j.saturating_sub(1);
    let col_max = usize::min(j + 1, schematic[0].len().saturating_sub(1));

    for row_idx in row_min..=row_max {
        for col_idx in col_min..=col_max {
            if row_idx == i && col_idx == j {
                continue;
            }

            if let Some(&val) = schematic.get(row_idx).and_then(|row| row.get(col_idx)) {
                if is_symbol(val) {
                    return Some(Symbol::new(row_idx, col_idx, val));
                }
            }
        }
    }

    None
}

fn try_parse_number(i: usize, j: usize, schematic: &Vec<Vec<char>>) -> Option<(usize, u32)> {
    let mut digits = String::new();
    let mut start = j as i32;

    while start >= 0 && (schematic[i][start as usize]).is_digit(10) {
        start -= 1;
    }

    let start = (start + 1) as usize;

    for col in start..schematic[i].len() {
        if schematic[i][col].is_digit(10) {
            digits.push(schematic[i][col]);
        } else {
            break;
        }
    }

    if digits.is_empty() {
        None
    } else {
        Some((start + digits.len(), digits.parse::<u32>().unwrap()))
    }
}

fn get_parts(schematic: &Vec<Vec<char>>) -> Vec<Part> {
    let mut parts = Vec::new();
    for i in 0..schematic.len() {
        let mut j = 0;
        while j < schematic[0].len() {
            if schematic[i][j].is_digit(10) {
                if let Some(symb) = has_surrounding_symbol(i, j, schematic) {
                    if let Some((new_j, num)) = try_parse_number(i, j, schematic) {
                        let part = Part::new(symb, num);
                        parts.push(part);
                        j = new_j;
                        continue;
                    }
                }
            }
            j += 1;
        }
    }
    parts
}

fn part1(schematic: &Vec<Vec<char>>) {
    let parts = get_parts(schematic);
    let sum: u32 = parts.iter().map(|p| p.get_num()).sum();

    println!("Part1: {}", sum);
}

fn part2(schematic: &Vec<Vec<char>>) {
    let parts = get_parts(schematic);
    let parts: Vec<&Part> = parts.iter().filter(|p| p.is_gear()).collect();

    let mut sum = 0;

    for i in 0..parts.len() {
        let part = parts[i];
        for &part2 in parts.iter().skip(i + 1) {
            if part2.get_symbol() == part.get_symbol() {
                sum += part2.get_num() * part.get_num();
            }
        }
    }

    println!("Part2: {}", sum);
}

fn main() {
    let input = str::from_utf8(include_bytes!("../input.txt")).unwrap();

    let mut schematic = Vec::new();
    for l in input.lines() {
        schematic.push(l.chars().collect());
    }

    part1(&schematic);
    part2(&schematic);
}
