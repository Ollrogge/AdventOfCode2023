use geo::{area::Area, Coordinate, LineString, Polygon};
use std::collections::HashMap;

fn parse_input(input: &str, part2: bool) -> (usize, Vec<Coordinate>) {
    let parts: Vec<Vec<&str>> = input.lines().map(|l| l.split(" ").collect()).collect();
    let mut loc = (0.0, 0.0);
    let mut coords = Vec::new();

    let mut boundary_points = 0x0;

    let dirs1: HashMap<&str, (f64, f64)> = HashMap::from([
        ("R", (0.0, 1.0)),
        ("L", (0.0, -1.0)),
        ("U", (-1.0, 0.0)),
        ("D", (1.0, 0.0)),
    ]);

    let dirs2: HashMap<&str, (f64, f64)> = HashMap::from([
        ("0", (0.0, 1.0)),
        ("2", (0.0, -1.0)),
        ("3", (-1.0, 0.0)),
        ("1", (1.0, 0.0)),
    ]);

    for part in parts.iter() {
        let (dir, amt) = match part2 {
            true => {
                let part = part[2];

                let dir = &part[part.len() - 2..part.len() - 1];
                let dir = dirs2[dir];

                let amt = &part[2..part.len() - 2];
                let amt = u64::from_str_radix(amt, 16).unwrap();

                (dir, amt as f64)
            }
            false => {
                let dir = part[0];
                let amt = part[1].parse::<f64>().unwrap();
                let dir = dirs1[dir];
                (dir, amt)
            }
        };

        boundary_points += amt as usize;

        loc = (loc.0 + dir.0 * amt, loc.1 + dir.1 * amt);

        coords.push(Coordinate { x: loc.0, y: loc.1 });
    }

    (boundary_points, coords)
}

fn part1(input: &str) {
    let (boundary_points, coords) = parse_input(&input, false);
    let poly = Polygon::new(LineString::from(coords), vec![]);
    let interior_points = poly.unsigned_area() - (boundary_points as f64 / 2.0) + 1.0;

    println!("Part1: {}", interior_points + boundary_points as f64);
}

fn part2(input: &str) {
    let (boundary_points, coords) = parse_input(&input, true);
    let poly = Polygon::new(LineString::from(coords), vec![]);
    let interior_points = poly.unsigned_area() - (boundary_points as f64 / 2.0) + 1.0;

    println!("Part2: {}", interior_points + boundary_points as f64);
}
fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
