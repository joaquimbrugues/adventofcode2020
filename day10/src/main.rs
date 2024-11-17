use std::{env,fs,process};
use std::collections::HashMap;

// Take the numbers within an iterator and sort them as they are read into a vector
// Add a 0 at the beginning and last+3 at the end
fn collect_sorted(iterator: impl Iterator<Item = u32>) -> Vec<u32> {
    let mut res = vec![0];
    for n in iterator {
        let mut i = 0;
        while i < res.len() && n > res[i] { i += 1; }
        res.insert(i, n);
    }
    if let Some(n) = res.last() {
        res.push(n+3);
    }
    res
}

fn run1(input: &str) -> usize {
    let numbers = collect_sorted(input.lines().map(|line| line.parse().unwrap()));
    let differences: Vec<u32> = numbers.iter().skip(1).zip(numbers.iter()).map(|(a,b)| a - b).collect();
    differences.iter().filter(|&d| *d == 1).count() * differences.iter().filter(|&d| *d == 3).count()
}

struct LazyPartitions {
    lazy: HashMap<u32, u64>,
}

impl LazyPartitions {
    fn new() -> Self {
        let mut lazy = HashMap::from([(1, 1), (2, 2), (3, 4)]);
        Self { lazy, }
    }

    fn partitions(&mut self, n: u32) -> u64 {
        match self.lazy.get(&n) {
            Some(res) => *res,
            None => {
                // Recursion
                let res = self.partitions(n - 3) + self.partitions(n - 2) + self.partitions(n - 1);
                self.lazy.insert(n, res);
                res
            },
        }
    }
}

fn run2(input: &str) -> u64 {
    let numbers = collect_sorted(input.lines().map(|line| line.parse().unwrap()));
    let mut lazypart = LazyPartitions::new();
    numbers.iter()
        .skip(1)
        .zip(numbers.iter())
        .map(|(a,b)| a - b)
        .fold(vec![], |mut v, d| {
            if d == 3 {
                if v.len() == 0 || *v.last().unwrap() > 0 {
                    v.push(0);
                }
            } else {
                if let Some(c) = v.last_mut() {
                    *c += 1;
                } else {
                    v.push(1);
                }
            }
            v
        }).iter().map(|&c| c)
    .filter(|&c| c > 0).map(|c| lazypart.partitions(c)).product()
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
fn example11() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 35);
}

#[test]
fn example12() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 220);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2176);
}

#[test]
fn example21() {
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 8);
}

#[test]
fn example22() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 19208);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 18512297918464);
}
