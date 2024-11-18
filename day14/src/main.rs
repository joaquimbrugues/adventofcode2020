use std::{env,fs,process};
use std::collections::HashMap;

fn apply_mask(num: u64, mask: &Vec<Option<u8>>) -> u64 {
    let mut temp = num;
    let mut num_vec = [0;36];
    let mut i = 35;
    while temp > 0 {
        num_vec[i] = (temp % 2) as u8;
        temp /= 2;
        i -= 1;
    }
    for j in 0..36 {
        if let Some(m) = mask[j] {
            num_vec[j] = m;
        }
    }
    num_vec.iter().fold(0, |acc, &n| {
        let temp = acc * 2;
        temp + (n as u64)
    })
}

fn run1(input: &str) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = vec![None];
    for line in input.lines() {
        let (left, right) = line.split_once(" = ").unwrap();
        if left.starts_with("mask") {
            // Update mask
            mask = right.trim().chars().map(|c| {
                match c {
                    'X' => None,
                    '0' => Some(0),
                    '1' => Some(1),
                    _ => unreachable!(),
                }
            }).collect();
        } else if left.starts_with("mem[") {
            // Update memory position using mask
            let mem_pos = left.strip_prefix("mem[").unwrap().strip_suffix(']').unwrap().parse().unwrap();
            let value = apply_mask(right.parse().unwrap(), &mask);
            if let Some(old) = memory.get_mut(&mem_pos) {
                *old = value;
            } else {
                memory.insert(mem_pos, value);
            }
        } else {
            unreachable!();
        }
    }
    memory.values().sum()
}

fn apply_mask2(num: u64, mask: &Vec<Option<u8>>) -> Vec<u64> {
    let mut temp = num;
    let mut num_vec = [0;36];
    let mut i = 35;
    while temp > 0 {
        num_vec[i] = (temp % 2) as u8;
        temp /= 2;
        i -= 1;
    }

    let mut results = vec![[0;36]];
    for j in 0..36 {
        if let Some(m) = mask[j] {
            match m {
                0 => {
                    for arr in results.iter_mut() {
                        arr[j] = num_vec[j];
                    }
                },
                1 => {
                    for arr in results.iter_mut() {
                        arr[j] = 1;
                    }
                },
                _ => unreachable!(),
            }
        } else {
            for arr in results.iter_mut() {
                arr[j] = 0;
            }
            let mut extra = results.clone();
            for arr in extra.iter_mut() {
                arr[j] = 1;
            }
            results.append(&mut extra);
        }
    }
    results.iter().map(|array| {
        array.iter().fold(0, |acc, &n| {
            let temp = acc * 2;
            temp + (n as u64)
        })
    }).collect()
}

fn run2(input: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = vec![None];
    for line in input.lines() {
        let (left, right) = line.split_once(" = ").unwrap();
        if left.starts_with("mask") {
            // Update mask
            mask = right.trim().chars().map(|c| {
                match c {
                    'X' => None,
                    '0' => Some(0),
                    '1' => Some(1),
                    _ => unreachable!(),
                }
            }).collect();
        } else if left.starts_with("mem[") {
            // Update memory position using mask
            let mem_pos_vec = apply_mask2(left.strip_prefix("mem[").unwrap().strip_suffix(']').unwrap().parse().unwrap(), &mask);
            let value: u64 = right.parse().unwrap();
            for mem_pos in mem_pos_vec {
                if let Some(old) = memory.get_mut(&mem_pos) {
                    *old = value;
                } else {
                    memory.insert(mem_pos, value);
                }
            }
        } else {
            unreachable!();
        }
    }
    memory.values().sum()
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
    let input = fs::read_to_string("test1.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 165);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 17481577045893);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 208);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 4160009892257);
}
