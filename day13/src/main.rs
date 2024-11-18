use std::{env,fs,process};

// Order according to the second coordinate
fn order(iterator: impl Iterator<Item = (u32, u32)>) -> Vec<(u32, u32)> {
    let mut v: Vec<(u32, u32)> = vec![];
    for (first, second) in iterator {
        let mut i = 0;
        while i < v.len() && second > v[i].1 { i += 1; }
        v.insert(i, (first, second));
    }
    v
}

fn run1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let (line1, line2) = (lines[0], lines[1]);
    let earliest: u32 = line1.parse().unwrap();
    let ordered = order(line2.split(',')
        .filter(|&s| s != "x")
        .map(|s| {
            let n = s.parse().unwrap();
            (n, n - (earliest % n))
        }));
    let (id, wait) = ordered.first().unwrap();
    id * wait
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
    assert_eq!(res, 295);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2095);
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
