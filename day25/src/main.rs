use std::{env,fs,process};

const MOD: u64 = 20201227;

fn step(value: u64, subject: u64) -> u64 {
    let tmp = value * subject;
    tmp % MOD
}

fn run1(input: &str) -> u64 {
    let data: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    assert_eq!(data.len(), 2);
    let card_public_key = data[0];
    let door_public_key = data[1];

    // Step 1: Figure out the card's loop size
    let mut card_loop_size = 0;
    let mut val = 1;
    while val != card_public_key {
        val = step(val, 7);
        card_loop_size += 1;
    }

    // Step 2: Figure out the door's loop size
    let mut door_loop_size = 0;
    let mut val = 1;
    while val != door_public_key {
        val = step(val, 7);
        door_loop_size += 1;
    }

    // Step 3: Get the encryption key!
    let mut val = 1;
    for _ in 0..card_loop_size {
        val = step(val, door_public_key);
    }
    let encryption_key = val;

    // Step 4: Assert that the computation also works backwards
    let mut val = 1;
    for _ in 0..door_loop_size {
        val = step(val, card_public_key);
    }
    assert_eq!(encryption_key, val);
    
    encryption_key
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
    assert_eq!(res, 14897079);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 15217943);
}
