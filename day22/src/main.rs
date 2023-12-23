use std::collections::HashSet;

fn parse_input(
    input: &str,
) -> (
    Vec<Vec<(isize, isize, isize)>>,
    HashSet<(isize, isize, isize)>,
) {
    let mut bricks = Vec::new();
    for l in input.lines() {
        let parts: Vec<&str> = l.split("~").collect();
        let mut iter = parts[0]
            .split(",")
            .into_iter()
            .map(|x| x.parse::<isize>().unwrap());

        let (sx, sy, sz) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let mut iter = parts[1]
            .split(",")
            .into_iter()
            .map(|x| x.parse::<isize>().unwrap());

        let (ex, ey, ez) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let mut brick = Vec::new();
        if sx == ex && sy == ey {
            for z in sz..=ez {
                brick.push((sx, sy, z));
            }
        } else if sx == ex && sz == ez {
            for y in sy..=ey {
                brick.push((sx, y, sz));
            }
        } else if sy == ey && sz == ez {
            for x in sx..=ex {
                brick.push((x, sy, sz));
            }
        } else {
            panic!("shouldn't happen");
        }

        bricks.push(brick);
    }
    let mut cubes = HashSet::new();
    for brick in bricks.iter() {
        for &cube in brick.iter() {
            cubes.insert(cube);
        }
    }
    loop {
        let mut any_moved = false;
        for brick in bricks.iter_mut() {
            let mut can_move = true;
            for &(x, y, z) in brick.iter() {
                if z == 1 {
                    can_move = false;
                    break;
                }

                if cubes.contains(&(x, y, z - 1)) && !brick.contains(&(x, y, z - 1)) {
                    can_move = false;
                }
            }

            if can_move {
                any_moved = true;
                for (x, y, z) in brick.iter_mut() {
                    assert!(cubes.contains(&(*x, *y, *z)));
                    cubes.remove(&(*x, *y, *z));

                    *z -= 1;

                    cubes.insert((*x, *y, *z));
                }
            }
        }

        if !any_moved {
            break;
        }
    }

    (bricks, cubes)
}

fn remove_brick(
    brick_remove: &Vec<(isize, isize, isize)>,
    bricks: &Vec<Vec<(isize, isize, isize)>>,
    cubes: &HashSet<(isize, isize, isize)>,
) -> usize {
    let mut cubes = cubes.clone();
    let mut bricks = bricks.clone();

    for cube in brick_remove.iter() {
        cubes.remove(cube);
    }

    let mut moved = HashSet::new();
    loop {
        let mut any_moved = false;
        for (i, brick) in bricks.iter_mut().enumerate() {
            if brick == brick_remove {
                continue;
            }
            let mut can_move = true;
            for &(x, y, z) in brick.iter() {
                if z == 1 {
                    can_move = false;
                    break;
                }

                if cubes.contains(&(x, y, z - 1)) && !brick.contains(&(x, y, z - 1)) {
                    can_move = false;
                }
            }

            if can_move {
                moved.insert(i);
                any_moved = true;
                for (x, y, z) in brick.iter_mut() {
                    assert!(cubes.contains(&(*x, *y, *z)));
                    cubes.remove(&(*x, *y, *z));

                    *z -= 1;

                    cubes.insert((*x, *y, *z));
                }
            }
        }

        if !any_moved {
            break;
        }
    }

    moved.len()
}

fn part1(input: &str) {
    let (bricks, cubes) = parse_input(input);

    let safe_bricks = bricks
        .iter()
        .map(|b| remove_brick(b, &bricks, &cubes))
        .filter(|x| *x == 0)
        .count();

    println!("Part1: {:?}", safe_bricks);
}

fn part2(input: &str) {
    let (bricks, cubes) = parse_input(input);

    let moved_bricks: usize = bricks
        .iter()
        .map(|b| remove_brick(b, &bricks, &cubes))
        .sum();

    println!("Part2: {:?}", moved_bricks);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
