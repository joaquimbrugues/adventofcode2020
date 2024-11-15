use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines().filter(|l| !l.is_empty()).map(|s| s.parse().unwrap()).collect();
    for n1 in nums.iter() {
        for n2 in nums.iter() {
            if n1 + n2 == 2020 {
                return n1 * n2;
            }
        }
    }
    panic!("Should not reach here")
}

fn run2(input: &str) -> u32 {
    let nums: Vec<u32> = input.lines().filter(|l| !l.is_empty()).map(|s| s.parse().unwrap()).collect();
    for n1 in nums.iter() {
        for n2 in nums.iter() {
            for n3 in nums.iter() {
                if n1 + n2 + n3 == 2020 {
                    return n1 * n2 * n3;
                }
            }
        }
    }
    panic!("Should not reach here")
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
    assert_eq!(res, 514579);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 181044);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 241861950);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 82660352);
}
