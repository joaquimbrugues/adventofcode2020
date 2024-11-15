use std::{env,fs,process};

fn run1(input: &str) -> u32 {
    let mut trees = 0;
    let mut pos = 0;
    let mut length = None;
    for line in input.lines() {
        match length {
            None => {
                length = Some(line.chars().count());
            },
            Some(l) => {
                pos = (pos + 3) % l;
                match line.chars().nth(pos).unwrap() {
                    '#' => {
                        trees += 1;
                    },
                    '.' => {},
                    _ => panic!("All characters should be '.' or '#'"),
                }
            },
        }
    }
    trees
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
    assert_eq!(res, 7);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 237);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 336);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
