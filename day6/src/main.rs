use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk.chars().filter(|c| {
                c.is_ascii_alphabetic()
            })
            .collect::<HashSet<char>>()
            .len()
        })
        .sum()
}

fn run2(input: &str) -> usize {
    let mut sum = 0;
    for chunk in input.split("\n\n") {
        let mut lines = chunk.lines();
        let mut questions: HashSet<char> = lines.next().unwrap().chars().collect();
        for line in lines {
            let chs = line.chars().collect();
            questions = questions.intersection(&chs).map(|&c| c).collect();
        }
        sum += questions.len();
    }
    sum
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
    assert_eq!(res, 11);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 7283);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 6);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 3520);
}
