use std::{env,fs,process};
use std::collections::HashSet;

enum Instruction {
    Nop(isize),
    Acc(i32),
    Jmp(isize),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        match s.split_once(' ').unwrap() {
            ("nop", n) => {
                let num = n.parse().unwrap();
                Self::Nop(num)
            },
            ("acc", n) => {
                let num = n.parse().unwrap();
                Self::Acc(num)
            },
            ("jmp", n) => {
                let num = n.parse().unwrap();
                Self::Jmp(num)
            },
            _ => panic!("Unexpected string!"),
        }
    }
}

fn run1(input: &str) -> i32 {
    let instructions: Vec<_> = input.lines().map(|line| Instruction::parse(line)).collect();
    let mut acc = 0;
    let mut curr = 0;
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&curr) {
            return acc;
        } else {
            visited.insert(curr);
            match instructions[curr] {
                Instruction::Nop(_) => curr += 1,
                Instruction::Acc(a) => {
                    acc += a;
                    curr += 1;
                },
                Instruction::Jmp(j) => {
                    curr = ((curr as isize) + j) as usize;
                },
            }
        }
    }
}

// Do the same as before, but with several attempts at finishing
// Each attempt carries a tuple:
// 1- Current Acc value; 2- Current instruction; 3- Boolean controlling whether we already switched
// a nop for a jmp or biceversa; 4- HashSet counting visited instructions
fn run2(input: &str) -> i32 {
    let instructions: Vec<_> = input.lines().map(|line| Instruction::parse(line)).collect();
    let mut attempts = vec![(0, 0, false, HashSet::new())];
    while let Some((mut acc, mut curr, mut changed, mut visited)) = attempts.pop() {
        if !visited.contains(&curr) {
            if curr == instructions.len() {
                return acc;
            }
            visited.insert(curr);
            match instructions[curr] {
                Instruction::Acc(a) => {
                    acc += a;
                    curr += 1;
                    attempts.push((acc, curr, changed, visited));
                },
                Instruction::Jmp(j) => {
                    let ncurr = ((curr as isize) + j) as usize;
                    attempts.push((acc, ncurr, changed, visited.clone()));
                    if !changed {
                        // Add a branch changing this instruction to Nop
                        changed = true;
                        curr += 1;
                        attempts.push((acc, curr, changed, visited));
                    }
                },
                Instruction::Nop(j) => {
                    let ncurr = curr + 1;
                    attempts.push((acc, ncurr, changed, visited.clone()));
                    if !changed {
                        // Add a branch changing this instruction to Jmp
                        changed = true;
                        curr = ((curr as isize) + j) as usize;
                        attempts.push((acc, curr, changed, visited));
                    }
                },
            }
        }
    }
    unreachable!()
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
    assert_eq!(res, 5);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2058);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,8);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1000);
}
