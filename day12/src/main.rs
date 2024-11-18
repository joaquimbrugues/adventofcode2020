use std::{env,fs,process};

// Idea: Cardinal points as integers: N = 0, E = 1, S = 2, W = 3
fn add(pos: (i32, i32), dist: u32, dir: u8) -> (i32, i32) {
    match dir {
        0 => {
            (pos.0 - (dist as i32), pos.1)
        },
        1 => {
            (pos.0, pos.1 + (dist as i32))
        },
        2 => {
            (pos.0 + (dist as i32), pos.1)
        },
        3 => {
            (pos.0, pos.1 - (dist as i32))
        },
        _ => unreachable!(),
    }
}

fn l1norm(pos: (i32, i32)) -> u32 {
    pos.0.unsigned_abs() + pos.1.unsigned_abs()
}

// Got this from here:
// https://stackoverflow.com/questions/71628761/how-to-split-a-string-into-the-first-character-and-the-rest
fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    chars.next().map(|c| (c, chars.as_str()))
}

fn run1(input: &str) -> u32 {
    let mut position = (0,0);
    let mut direction = 1;
    for line in input.lines() {
        let (c, r) = split_first_char(line).unwrap();
        let n: u16 = r.parse().unwrap();
        match c {
            'N' => {
                position = add(position, n as u32, 0);
            },
            'E' => {
                position = add(position, n as u32, 1);
            },
            'S' => {
                position = add(position, n as u32, 2);
            },
            'W' => {
                position = add(position, n as u32, 3);
            },
            'F' => {
                position = add(position, n as u32, direction);
            },
            'R' => {
                direction = (direction + ((n / 90) as u8)) % 4;
            },
            'L' => {
                direction = (direction + 4 - ((n / 90) as u8)) % 4;
            },
            _ => unreachable!(),
        }
    }
    l1norm(position)
}

fn run2(input: &str) -> u32 {
    let mut position = (0,0);
    let mut waypoint = (-1, 10);
    for line in input.lines() {
        let (c, r) = split_first_char(line).unwrap();
        let n: i32 = r.parse().unwrap();
        let mut rotation = 0;
        match c {
            'N' => {
                waypoint = (waypoint.0 - n, waypoint.1);
            },
            'E' => {
                waypoint = (waypoint.0, waypoint.1 + n);
            },
            'S' => {
                waypoint = (waypoint.0 + n, waypoint.1);
            },
            'W' => {
                waypoint = (waypoint.0, waypoint.1 - n);
            },
            'F' => {
                position = (position.0 + n * waypoint.0, position.1 + n * waypoint.1);
            },
            'R' => {
                rotation = n / 90;
            },
            'L' => {
                rotation = 4 - ( n / 90 );
            },
            _ => unreachable!(),
        }
        match rotation {
            0 => {},
            1 => {
                waypoint = (waypoint.1, - waypoint.0);
            },
            2 => {
                waypoint = (- waypoint.0, - waypoint.1);
            },
            3 => {
                waypoint = (- waypoint.1, waypoint.0);
            },
            _ => unreachable!(),
        }
    }
    l1norm(position)
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
    assert_eq!(res, 25);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 879);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 286);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 18107);
}
