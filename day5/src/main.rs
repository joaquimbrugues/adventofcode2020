use std::{env,fs,process};

// Take a boarding pass and return its (row, column, id)
fn parse_boarding_pass(line: &str) -> (u16, u16, u16) {
    assert_eq!(line.len(), 10);
    let chs: Vec<char> = line.chars().collect();
    let row = chs[0..7].iter().fold(0,
        |acc, c| {
            acc * 2 + match c {
                'F' => 0,
                'B' => 1,
                _ => panic!("Unexpected character"),
            }
        });
    let column = chs[7..10].iter().fold(0,
        |acc, c| {
            acc * 2 + match c {
                'L' => 0,
                'R' => 1,
                _ => panic!("Unexpected character"),
            }
        });
    (row, column, row * 8 + column)
}

fn run1(input: &str) -> u16 {
    let mut max = u16::MIN;
    for line in input.lines() {
        let (_, _, id) = parse_boarding_pass(line);
        if id > max {
            max = id;
        }
    }
    max
}

fn create_sorted(it: impl Iterator<Item = u16>) -> Vec<u16> {
    let mut v = vec![];
    for n in it {
        let mut i = 0;
        while i < v.len() && v[i] < n { i += 1; }
        v.insert(i, n);
    }
    v
}

fn run2(input: &str) -> u16 {
    let ids = create_sorted(input.lines().map(|line| parse_boarding_pass(line).2));
    for (big, small) in ids.iter().skip(1).zip(ids.iter()) {
        if big - small == 2 {
            return small + 1;
        }
    }
    panic!("Unreachable!");
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
    assert_eq!(parse_boarding_pass("FBFBBFFRLR"), (44, 5, 357));
    assert_eq!(parse_boarding_pass("BFFFBBFRRR"), (70, 7, 567));
    assert_eq!(parse_boarding_pass("FFFBBBFRRR"), (14, 7, 119));
    assert_eq!(parse_boarding_pass("BBFFBBFRLL"), (102, 4, 820));
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 861);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 633);
}
