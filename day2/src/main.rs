use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        // FORMAT
        // {min}-{max} {char}: {pass}
        let chunks: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(chunks.len(), 3);
        let (smin, smax) = chunks[0].split_once('-').unwrap();
        let min = smin.parse().unwrap();
        let max = smax.parse().unwrap();
        let c = chunks[1].chars().nth(0).unwrap();
        let pass = chunks[2];
        let mut loc_count = 0;
        for d in pass.chars() {
            if c == d {
                loc_count += 1;
            }
            if loc_count > max {
                break;
            }
        }
        if loc_count >= min && loc_count <= max {
            count += 1;
        }
    }
    count
}

fn run2(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        // FORMAT
        // {first}-{second} {char}: {pass}
        let chunks: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(chunks.len(), 3);
        let (sfirst, ssecond) = chunks[0].split_once('-').unwrap();
        let first: usize = sfirst.parse().unwrap();
        let second: usize = ssecond.parse().unwrap();
        let c = chunks[1].chars().nth(0).unwrap();
        let c2 = match chunks[2].chars().nth(second - 1) {
            Some(d) => d,
            None => continue,
        };
        let c1 = chunks[2].chars().nth(first - 1).unwrap();
        if (c == c1 && c != c2) || (c != c1 && c == c2) {
            count += 1;
        } 
    }
    count
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
    assert_eq!(res, 2);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 506);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,443);
}
