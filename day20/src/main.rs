use std::{env,fs,process};
use std::collections::{HashMap, HashSet};

#[derive(Clone,Debug)]
struct Tile {
    top: Vec<bool>,
    bottom: Vec<bool>,
    left: Vec<bool>,
    right: Vec<bool>,
}

impl Tile {
    fn new(top: Vec<bool>, bottom: Vec<bool>, left: Vec<bool>, right: Vec<bool>) -> Self {
        assert_eq!(top.len(), bottom.len());
        assert_eq!(top.len(), left.len());
        assert_eq!(top.len(), right.len());
        Self { top, bottom, left, right, }
    }

    fn flip(&mut self) {
        let tmp = self.left.clone();
        self.left = self.right.clone();
        self.right = tmp;
        let mut tmp = self.bottom.clone();
        tmp.reverse();
        self.bottom.reverse();
        self.top.reverse();
    }

    fn rotate(&self) -> Self {
        let mut top = self.left.clone();
        top.reverse();
        let right = self.top.clone();
        let mut bottom = self.right.clone();
        bottom.reverse();
        let left = self.bottom.clone();
        Self { top, bottom, left, right, }
    }

    fn match_top(&self, other: &Self) -> bool {
        self.top.iter().zip(other.bottom.iter()).all(|(&b1, &b2)| b1 == b2)
    }

    fn match_left(&self, other: &Self) -> bool {
        self.left.iter().zip(other.right.iter()).all(|(&b1, &b2)| b1 == b2)
    }

    fn match_bottom(&self, other: &Self) -> bool {
        other.match_top(&self)
    }

    fn match_right(&self, other: &Self) -> bool {
        other.match_left(&self)
    }
}

enum Permutation { R(u8), SR(u8), }

impl Permutation {
    fn all() -> [Self; 8] {
        use Permutation::*;
        [ R(0), R(1), R(2), R(3),
        SR(0), SR(1), SR(2), SR(3), ]
    }

    fn permutate(&self, sq: &Tile) -> Tile {
        match self {
            Self::R(r) => {
                match r % 4 {
                    0 => sq.clone(),
                    1 | 2 | 3 => {
                        let mut tmp = sq.rotate();
                        let s = r - 1;
                        for _ in 0..s {
                            tmp = tmp.rotate();
                        }
                        tmp
                    },
                    _ => unreachable!(),
                }
            },
            &Self::SR(r) => {
                let mut tmp = Self::R(r).permutate(sq);
                tmp.flip();
                tmp
            },
        }
    }
}

fn run1(input: &str) -> u64 {
    let tiles: HashMap<u64, Tile> = input.split("\n\n").map(|chunk| {
        let mut lines = chunk.lines();
        let first = lines.next().unwrap();
        let id = first.strip_prefix("Tile ").unwrap().strip_suffix(':').unwrap().parse().unwrap();

        let top = lines.next().unwrap().chars().map(|c| c == '#').collect();
        let bottom = lines.last().unwrap().chars().map(|c| c == '#').collect();
        let left = chunk.lines().skip(1).map(|line| line.chars().next().unwrap() == '#').collect();
        let right = chunk.lines().skip(1).map(|line| line.chars().last().unwrap() == '#').collect();
        (id, Tile::new(top, bottom, left, right))
    }).collect();

    let mut side = 0;
    while side * side < tiles.len() { side += 1; }
    assert_eq!(side * side, tiles.len());

    let mut res = 1;
    let mut counter = 0;
    for (id, tile) in tiles.iter() {
        let mut matches = [false; 4];
        for (id2, tile2) in tiles.iter() {
            if id == id2 { continue; } // Do not check with the exact same tile
            if matches.iter().all(|b| *b) {break; } // Don't bother continue if we are done

            if !matches[0] {
                // We have not yet matched on the top
                matches[0] = Permutation::all().iter().any(|p| tile.match_top(&p.permutate(tile2)));
            }
            if !matches[1] {
                // We have not yet matched on the bottom
                matches[1] = Permutation::all().iter().any(|p| tile.match_bottom(&p.permutate(tile2)));
            }
            if !matches[2] {
                // We have not yet matched on the left
                matches[2] = Permutation::all().iter().any(|p| tile.match_left(&p.permutate(tile2)));
            }
            if !matches[3] {
                // We have not yet matched on the bottom
                matches[3] = Permutation::all().iter().any(|p| tile.match_right(&p.permutate(tile2)));
            }
        }
        let neighs = matches.iter().filter(|&&b| b).count();
        if neighs < 2 {
            panic!("Unexpected tile with only 0 or 1 neighbour! ID: {id}")
        }
        if neighs == 2 {
            counter += 1;
            res *= id;
        }
        if counter > 4 {
            panic!("What's going on? Too many corners");
        }
    }

    res
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
    assert_eq!(res,20899048083289);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 5966506063747);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 273);
}

//#[test]
//fn input2() {
    //let input = fs::read_to_string("input.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}
