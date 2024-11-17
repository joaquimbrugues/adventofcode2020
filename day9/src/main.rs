use std::{env,fs,process};

fn run1(input: &str, preamble_length: usize) -> u64 {
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut current = preamble_length;
    while current < numbers.len() {
        let mut searching = true;
        let mut i = current - preamble_length;
        while searching && i < current {
            let mut j = i + 1;
            while searching && j < current {
                searching = numbers[current] != numbers[i] + numbers[j];
                j += 1;
            }
            i += 1;
        }
        if searching {
            return numbers[current];
        }
        current += 1;
    }
    unreachable!()
}

fn run2(input: &str, preamble_length: usize) -> u64 {
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut current = preamble_length;
    let mut searching = false;
    // Find the non-matching number in the list
    while !searching && current < numbers.len() {
        searching = true;
        let mut i = current - preamble_length;
        while searching && i < current {
            let mut j = i + 1;
            while searching && j < current {
                searching = numbers[current] != numbers[i] + numbers[j];
                j += 1;
            }
            i += 1;
        }
        if !searching {
            current += 1;
        }
    }
    let invalid = numbers[current];

    // Find the collection of numbers that add up to the given number
    let mut min = 0;
    while min < numbers.len() - 1 {
        let mut max = min + 1;
        let mut sum = numbers[min];
        while max < numbers.len() && sum < invalid {
            sum += numbers[max];
            if sum == invalid {
                return numbers[min..=max].iter().min().unwrap() + numbers[min..=max].iter().max().unwrap();
            }
            max += 1;
        }
        min += 1;
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

    let res = run2(&input, 25);
    println!("{res}");
}

#[test]
fn example1() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run1(&input, 5);
    assert_eq!(res, 127);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input, 25);
    assert_eq!(res, 70639851);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input, 5);
    assert_eq!(res, 62);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input, 25);
    assert_eq!(res, 8249240);
}
