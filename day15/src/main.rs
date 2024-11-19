use std::{env,fs,process};
use std::collections::HashMap;

fn run(input: &str, N: usize) -> usize {
    let input = input.trim();
    let mut game: HashMap<usize, Vec<usize>> = input.split(',').map(|s| s.parse().unwrap()).enumerate().map(|(e,n)| (n, vec![e])).collect();
    let mut last: usize = input.rsplit_once(',').unwrap().1.parse().unwrap();
    let mut j = game.len();
    while j < N {
        let turns = game.get(&last).unwrap();
        //println!("{turns:?}");
        let l = turns.len();
        last = if l > 1 {
            // This number has been spoken before
            turns[l-1] - turns[l-2]
        } else {
            0
        };
        //println!("{last}");
        if let Some(v) = game.get_mut(&last) {
            if v.len() > 1 {
                v.remove(0);
            }
            v.push(j);
        } else {
            game.insert(last, vec![j]);
        }
        j += 1;
    }
    last
}

fn run1(input: &str) -> usize {
    run(input, 2020)
}

// This is absurdly slow... but good enough in memory, and works!
fn run2(input: &str) -> usize {
    run(input, 30000000)
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
    let mut lines = input.lines();
    assert_eq!(run1(lines.next().unwrap()), 436);
    assert_eq!(run1(lines.next().unwrap()), 1);
    assert_eq!(run1(lines.next().unwrap()), 10);
    assert_eq!(run1(lines.next().unwrap()), 27);
    assert_eq!(run1(lines.next().unwrap()), 78);
    assert_eq!(run1(lines.next().unwrap()), 438);
    assert_eq!(run1(lines.next().unwrap()), 1836);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 240);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let mut lines = input.lines();
    assert_eq!(run2(lines.next().unwrap()), 175594);
    assert_eq!(run2(lines.next().unwrap()), 2578);
    assert_eq!(run2(lines.next().unwrap()), 3544142);
    assert_eq!(run2(lines.next().unwrap()), 261214);
    assert_eq!(run2(lines.next().unwrap()), 6895259);
    assert_eq!(run2(lines.next().unwrap()), 18);
    assert_eq!(run2(lines.next().unwrap()), 362);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 505);
}
