use std::{env,fs,process};
use std::collections::{HashSet, HashMap};

fn one_move(wheel: &Vec<usize>) -> Vec<usize> {
    let current = wheel[0];
    let mut dest = current - 1;
    while dest == 0 || wheel[..4].contains(&dest) {
        if dest == 0 {
            dest = wheel[4..].iter().copied().max().unwrap();
            break;
        }
        dest -= 1;
    }

    let mut res = Vec::with_capacity(wheel.len());
    for &n in &wheel[4..] {
        res.push(n);
        if n == dest {
            for &nn in &wheel[1..4] {
                res.push(nn);
            }
        }
    }
    res.push(current);
    res
}

fn reformat(mut vec: Vec<usize>) -> String {
    let mut res = String::new();
    let mut pos = 0;
    while vec[pos] != 1 {
        pos += 1;
    }
    vec.remove(pos);
    while pos < vec.len() {
        res = format!("{res}{}", vec.remove(pos));
    }
    while vec.len() > 0 {
        res = format!("{res}{}", vec.remove(0));
    }
    res
}

fn run1(input: &str, rounds: usize) -> String {
    let mut wheel: Vec<usize> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    for _ in 0..rounds {
        wheel = one_move(&wheel);
    }
    reformat(wheel)
}

fn run2(input: &str) -> u64 {
    let mut nums = input.trim().chars().map(|c| c.to_digit(10).unwrap());
    let nums2 = nums.clone();
    let nums3 = nums.clone();
    let first = nums.next().unwrap();
    let mut last = nums3.last().unwrap();
    let mut wheel: HashMap<u32, u32> = nums2.zip(nums).collect();
    wheel.reserve(1_000_000);
    let next = wheel.keys().copied().max().unwrap() + 1;
    for n in next..=1_000_000 {
        // Input values
        wheel.insert(last, n);
        last = n;
    }
    wheel.insert(1_000_000, first);

    let mut current = first;
    for _ in 0..10_000_000 {
        let n1 = wheel.remove(&current).unwrap();
        let n2 = *wheel.get(&n1).unwrap();
        let n3 = *wheel.get(&n2).unwrap();
        let n4 = wheel.remove(&n3).unwrap();
        let next = [n1, n2, n3];
        let mut dest = current - 1;
        while dest <= 0 || next.contains(&dest) {
            if dest <= 0 {
                dest = 1_000_000;
            } else {
                dest -= 1;
            }
        }
        let ddest = wheel.remove(&dest).unwrap();
        wheel.insert(dest, n1);
        wheel.insert(n3, ddest);
        wheel.insert(current, n4);

        current = n4;
    }
    let a = *wheel.get(&1).unwrap();
    (a as u64) * (*wheel.get(&a).unwrap() as u64)
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
fn example1_10() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input, 10);
    assert_eq!(res, "92658374".to_owned());
}

#[test]
fn example1_100() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input, 100);
    assert_eq!(res, "67384529".to_owned());
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input, 100);
    assert_eq!(res, "82934675".to_owned());
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 149245887792);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 474600314018);
}
