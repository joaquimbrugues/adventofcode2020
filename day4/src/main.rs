use std::{env,fs,process};
use std::collections::HashSet;

fn run1(input: &str) -> u32 {
    let mut valid = 0;
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for pass in input.split("\n\n") {
        let keys: HashSet<&str> = pass
            .split_whitespace()
            .map(|chunk| chunk.split_once(':').unwrap().0)
            .collect();
        if required.iter().all(|field| keys.contains(field)) {
            valid += 1;
        }
    }
    valid
}

fn run2(input: &str) -> u32 {
    0
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

    let res = run1(&input);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 213);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
