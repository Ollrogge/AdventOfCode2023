use itertools::Itertools;
use z3::ast::{Ast, Int, Real};

fn parse_input(input: &str) -> Vec<((f64, f64, f64), (f64, f64, f64))> {
    let mut ret = Vec::new();
    for l in input.lines() {
        let l = l.replace(" ", "");
        let parts: Vec<&str> = l.split("@").collect();
        let mut iter_pos = parts[0].split(',');
        let mut iter_vel = parts[1].split(',');

        let (x, y, z) = (
            iter_pos.next().unwrap().parse::<f64>().unwrap(),
            iter_pos.next().unwrap().parse::<f64>().unwrap(),
            iter_pos.next().unwrap().parse::<f64>().unwrap(),
        );

        let (vx, vy, vz) = (
            iter_vel.next().unwrap().parse::<f64>().unwrap(),
            iter_vel.next().unwrap().parse::<f64>().unwrap(),
            iter_vel.next().unwrap().parse::<f64>().unwrap(),
        );

        ret.push(((x, y, z), (vx, vy, vz)));
    }

    ret
}

fn find_intersection_2d(
    start1: (f64, f64),
    velocity1: (f64, f64),
    start2: (f64, f64),
    velocity2: (f64, f64),
) -> Option<(f64, f64)> {
    let (x1, y1) = start1;
    let (v1_x, v1_y) = velocity1;
    let (x2, y2) = start2;
    let (v2_x, v2_y) = velocity2;

    let det = v2_x * v1_y - v2_y * v1_x;

    // Check if vectors are parallel (det is zero)
    if det.abs() < 1e-10 {
        None
    } else {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let u = (dy * v2_x - dx * v2_y) / det;
        let v = (dy * v1_x - dx * v1_y) / det;
        if u > 0.0 && v > 0.0 {
            Some((x1 + v1_x * u, y1 + v1_y * u))
        } else {
            None
        }
    }
}

fn part1(input: &str) {
    let hails = parse_input(input);
    let mut ints = 0x0;
    for pair in hails.into_iter().combinations(2) {
        let a = pair[0];
        let b = pair[1];
        let start_a = (a.0 .0, a.0 .1);
        let vel_a = (a.1 .0, a.1 .1);

        let start_b = (b.0 .0, b.0 .1);
        let vel_b = (b.1 .0, b.1 .1);

        if let Some(int) = find_intersection_2d(start_a, vel_a, start_b, vel_b) {
            if int.0 >= 200000000000000.0
                && int.0 <= 400000000000000.0
                && int.1 >= 200000000000000.0
                && int.1 <= 400000000000000.0
            {
                ints += 1;
            }
        }
    }

    println!("Part1: {}", ints);
}
fn f64_to_real(ctx: &z3::Context, value: f64) -> Real {
    let scale_factor = 1000.0;
    let numerator = (value * scale_factor).round() as i32;
    let denominator = scale_factor as i32;
    Real::from_real(&ctx, numerator, denominator)
}

fn part2(input: &str) {
    let hails = parse_input(input);
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    /*
    let hails: Vec<((i64, i64, i64), (i64, i64, i64))> = hails.iter_mut().map(|&mut ((x, y, z), (vx, vy, vz))| {
        ((x as i64, y as i64, z as i64), (vx as i64, vy as i64, vz as i64))
    }).collect();
    */

    let gc = |name: &str| -> Real<'_> { Real::new_const(&ctx, name) };

    let (x, y, z, vx, vy, vz) = (gc("x"), gc("y"), gc("z"), gc("vx"), gc("vy"), gc("vz"));

    let coeffs: Vec<Real<'_>> = (0..hails.len())
        .map(|i| gc(format!("c{}", i).as_str()))
        .collect();

    let coeffs2: Vec<Real<'_>> = (0..hails.len())
        .map(|i| gc(format!("c2{}", i).as_str()))
        .collect();

    for (i, &((hx, hy, hz), (hvx, hvy, hvz))) in hails.iter().enumerate() {
        /*
        let (hx, hy, hz, hvx, hvy, hvz) = (
            f64_to_real(&ctx, hx),
            f64_to_real(&ctx, hy),
            f64_to_real(&ctx, hz),
            f64_to_real(&ctx, hvx),
            f64_to_real(&ctx, hvy),
            f64_to_real(&ctx, hvz),
        );
        */

        let time = Real::fresh_const(&ctx, "time");
        solver.assert(&(&x + &time * &vx)._eq(
            &(Real::from_int(&Int::from_i64(&ctx, hx as i64))
                + Real::from_int(&Int::from_i64(&ctx, hvx as i64)) * &time),
        ));
        solver.assert(&(&y + &time * &vy)._eq(
            &(Real::from_int(&Int::from_i64(&ctx, hy as i64))
                + Real::from_int(&Int::from_i64(&ctx, hvy as i64)) * &time),
        ));
        solver.assert(&(&z + &time * &vz)._eq(
            &(Real::from_int(&Int::from_i64(&ctx, hz as i64))
                + Real::from_int(&Int::from_i64(&ctx, hvz as i64)) * &time),
        ));

        // pub type Z = i128;
        //pub type Q = Ratio<Z>;
        /*
        solver.assert(hx + hvx * coeffs2[i] - x + vx * coeffs[i] == 0);
        solver.assert(hy + hvy * coeffs2[i] - y + vy * coeffs[i] == 0);
        solver.assert(hz + hvz * coeffs2[i] - z + vz * coeffs[i] == 0);
        */
    }

    match solver.check() {
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();

            let mut sum = 0x0;
            for d in [&x, &y, &z] {
                let val = model.get_const_interp(d).unwrap().as_real().unwrap();
                sum += val.0 / val.1;
            }

            println!("Part2: {}", sum)
        }
        z3::SatResult::Unsat => println!("Unsatisfiable"),
        z3::SatResult::Unknown => println!("Unknown"),
    }
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
