use std::{env,fs,process};
use std::collections::{VecDeque, HashSet};

fn run1(input: &str) -> u32 {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut player1: VecDeque<u32> = first.lines().skip(1).map(|line| line.parse().unwrap()).collect();
    let mut player2: VecDeque<u32> = second.lines().skip(1).map(|line| line.parse().unwrap()).collect();

    while player1.len() > 0 && player2.len() > 0 {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        if c1 > c2 {
            player1.push_back(c1);
            player1.push_back(c2);
        } else if c1 < c2 {
            player2.push_back(c2);
            player2.push_back(c1);
        } else {
            panic!("The cards are equal!");
        }
    }

    let player = if player1.len() > 0 { player1 } else { player2 };
    player.into_iter().rev().fold((0, 0), |(acc, mult), c| {
        let mult = mult + 1;
        (acc + (mult * c), mult)
    }).0
}

fn recursive_combat(player1: &mut VecDeque<usize>, player2: &mut VecDeque<usize>) -> bool {
    let mut plays: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();
    while player1.len() > 0 && player2.len() > 0 {
        if plays.contains(&(player1.clone(), player2.clone())) {
            return true;
        } else {
            plays.insert((player1.clone(), player2.clone()));
            let c1 = player1.pop_front().unwrap();
            let c2 = player2.pop_front().unwrap();
            let winner = if c1 <= player1.len() && c2 <= player2.len() {
                let mut nplayer1 = VecDeque::new();
                for i in 0..c1 { nplayer1.push_back(player1[i]); }
                let mut nplayer2 = VecDeque::new();
                for i in 0..c2 { nplayer2.push_back(player2[i]); }
                recursive_combat(&mut nplayer1, &mut nplayer2)
            } else {
                c1 > c2
            };
            if winner {
                player1.push_back(c1);
                player1.push_back(c2);
            } else {
                player2.push_back(c2);
                player2.push_back(c1);
            }
        }
    }
    player2.len() == 0
}

fn run2(input: &str) -> usize {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut player1: VecDeque<usize> = first.lines().skip(1).map(|line| line.parse().unwrap()).collect();
    let mut player2: VecDeque<usize> = second.lines().skip(1).map(|line| line.parse().unwrap()).collect();
    if recursive_combat(&mut player1, &mut player2) {
        player1.into_iter().rev().fold((0, 0), |(acc, mult), c| {
            let mult = mult + 1;
            (acc + (mult * c), mult)
        }).0
    } else {
        player2.into_iter().rev().fold((0, 0), |(acc, mult), c| {
            let mult = mult + 1;
            (acc + (mult * c), mult)
        }).0
    }
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
    assert_eq!(res, 306);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 35005);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 291);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 32751);
}
