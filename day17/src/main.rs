use std::{env,fs,process};
use std::collections::HashSet;

fn neighbours(point: (i32, i32, i32)) -> [(i32, i32, i32); 26] {
    [
        (point.0 - 1, point.1 - 1, point.2 - 1),
        (point.0 - 1, point.1 - 1, point.2),
        (point.0 - 1, point.1 - 1, point.2 + 1),
        (point.0 - 1, point.1, point.2 - 1),
        (point.0 - 1, point.1, point.2),
        (point.0 - 1, point.1, point.2 + 1),
        (point.0 - 1, point.1 + 1, point.2 - 1),
        (point.0 - 1, point.1 + 1, point.2),
        (point.0 - 1, point.1 + 1, point.2 + 1),
        (point.0, point.1 - 1, point.2 - 1),
        (point.0, point.1 - 1, point.2),
        (point.0, point.1 - 1, point.2 + 1),
        (point.0, point.1, point.2 - 1),
        (point.0, point.1, point.2 + 1),
        (point.0, point.1 + 1, point.2 - 1),
        (point.0, point.1 + 1, point.2),
        (point.0, point.1 + 1, point.2 + 1),
        (point.0 + 1, point.1 - 1, point.2 - 1),
        (point.0 + 1, point.1 - 1, point.2),
        (point.0 + 1, point.1 - 1, point.2 + 1),
        (point.0 + 1, point.1, point.2 - 1),
        (point.0 + 1, point.1, point.2),
        (point.0 + 1, point.1, point.2 + 1),
        (point.0 + 1, point.1 + 1, point.2 - 1),
        (point.0 + 1, point.1 + 1, point.2),
        (point.0 + 1, point.1 + 1, point.2 + 1),
    ]
}

fn neighbours_and_self(point: (i32, i32, i32)) -> [(i32, i32, i32); 27] {
    [
        (point.0 - 1, point.1 - 1, point.2 - 1),
        (point.0 - 1, point.1 - 1, point.2),
        (point.0 - 1, point.1 - 1, point.2 + 1),
        (point.0 - 1, point.1, point.2 - 1),
        (point.0 - 1, point.1, point.2),
        (point.0 - 1, point.1, point.2 + 1),
        (point.0 - 1, point.1 + 1, point.2 - 1),
        (point.0 - 1, point.1 + 1, point.2),
        (point.0 - 1, point.1 + 1, point.2 + 1),
        (point.0, point.1 - 1, point.2 - 1),
        (point.0, point.1 - 1, point.2),
        (point.0, point.1 - 1, point.2 + 1),
        (point.0, point.1, point.2 - 1),
        (point.0, point.1, point.2),
        (point.0, point.1, point.2 + 1),
        (point.0, point.1 + 1, point.2 - 1),
        (point.0, point.1 + 1, point.2),
        (point.0, point.1 + 1, point.2 + 1),
        (point.0 + 1, point.1 - 1, point.2 - 1),
        (point.0 + 1, point.1 - 1, point.2),
        (point.0 + 1, point.1 - 1, point.2 + 1),
        (point.0 + 1, point.1, point.2 - 1),
        (point.0 + 1, point.1, point.2),
        (point.0 + 1, point.1, point.2 + 1),
        (point.0 + 1, point.1 + 1, point.2 - 1),
        (point.0 + 1, point.1 + 1, point.2),
        (point.0 + 1, point.1 + 1, point.2 + 1),
    ]
}

fn run1(input: &str) -> usize {
    let mut active_cubes = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((j as i32, i as i32, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut new_active_cubes = HashSet::new();
        for p in active_cubes.iter().map(|&q| neighbours_and_self(q)).flatten() {
            if active_cubes.contains(&p) {
                // Active
                let actives = neighbours(p).iter().filter(|q| active_cubes.contains(q)).count();
                if actives == 2 || actives == 3 {
                    new_active_cubes.insert(p);
                }
            } else {
                // Inactive
                let actives = neighbours(p).iter().filter(|q| active_cubes.contains(q)).count();
                if actives == 3 {
                    new_active_cubes.insert(p);
                }
            }
        }
        active_cubes = new_active_cubes;
    }

    active_cubes.len()
}

fn neighbours_and_self4(point: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut v = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for t in -1..=1 {
                    v.push((point.0 + x,
                            point.1 + y,
                            point.2 + z,
                            point.3 + t,
                            ));
                }
            }
        }
    }
    assert_eq!(v.len(), 81);
    v
}

fn neighbours4(point: (i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let mut v = neighbours_and_self4(point);
    assert_eq!(point, v.swap_remove(40));
    v
}

fn run2(input: &str) -> usize {
    let mut active_cubes = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                active_cubes.insert((j as i32, i as i32, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        let mut new_active_cubes = HashSet::new();
        for p in active_cubes.iter().map(|&q| neighbours_and_self4(q)).flatten() {
            if active_cubes.contains(&p) {
                // Active
                let actives = neighbours4(p).iter().filter(|q| active_cubes.contains(q)).count();
                if actives == 2 || actives == 3 {
                    new_active_cubes.insert(p);
                }
            } else {
                // Inactive
                let actives = neighbours4(p).iter().filter(|q| active_cubes.contains(q)).count();
                if actives == 3 {
                    new_active_cubes.insert(p);
                }
            }
        }
        active_cubes = new_active_cubes;
    }

    active_cubes.len()
}

fn main() {
    let mut args = env::args();
    let filepath;
    args.next();
    if let Some(s) = args.next() {
        filepath = s;
    }
    else {
        eprintln!("Give me a file name! I must feeds on files! Aaargh!");
        process::exit(1);
    }

    let input = fs::read_to_string(filepath).unwrap();

    let res = run2(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 112);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 333);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 848);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2676);
}
