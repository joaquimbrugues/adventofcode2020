use std::{env,fs,process};
use std::ops::{Add, AddAssign};
use std::collections::HashSet;

// REFERENCE:
// E = (1, 0)
// NE = (0, 1)
// NW = (-1, 1)
// W = (-1, 0)
// SW = (0, -1)
// SE = (1, -1)

#[derive(Clone,Copy)]
struct Arrow {
    inner: (i32, i32),
}

impl Arrow {
    fn try_parse(string: &str) -> (Self, &str) {
        if let Some(res) = string.strip_prefix("ne") {
            (Self::from((0,1)), res)
        } else if let Some(res) = string.strip_prefix("nw") {
            (Self::from((-1, 1)), res)
        } else if let Some(res) = string.strip_prefix("se") {
            (Self::from((1, -1)), res)
        } else if let Some(res) = string.strip_prefix("sw") {
            (Self::from((0, -1)), res)
        } else if let Some(res) = string.strip_prefix('e') {
            (Self::from((1, 0)), res)
        } else if let Some(res) = string.strip_prefix('w') {
            (Self::from((-1, 0)), res)
        } else {
            panic!("Unexpected character! {}", &string[0..1]);
        }
    }

    fn basis() -> [Self; 6] {
        [
            Self::from((1,0)),  // E
            Self::from((0,1)),  // NE
            Self::from((-1,1)),  // NW
            Self::from((-1,0)), // W
            Self::from((0,-1)), // SW
            Self::from((1,-1)), // SE
        ]
    }
}

impl From<(i32, i32)> for Arrow {
    fn from(f: (i32, i32)) -> Self {
        Self { inner: f }
    }
}

impl Add for Arrow {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let a = (self.inner.0 + rhs.inner.0, self.inner.1 + rhs.inner.1);
        Self::from(a)
    }
}

impl AddAssign for Arrow {
    fn add_assign(&mut self, rhs: Self) {
        self.inner.0 += rhs.inner.0;
        self.inner.1 += rhs.inner.1;
    }
}

#[derive(PartialEq,Eq,Hash,Clone,Copy)]
struct Point {
    inner: (i32, i32),
}

impl Add<Arrow> for Point {
    type Output = Self;

    fn add(self, rhs: Arrow) -> Self {
        let a = (self.inner.0 + rhs.inner.0, self.inner.1 + rhs.inner.1);
        Self::from(a)
    }
}

impl AddAssign<Arrow> for Point {
    fn add_assign(&mut self, rhs: Arrow) {
        self.inner.0 += rhs.inner.0;
        self.inner.1 += rhs.inner.1;
    }
}

impl From<(i32, i32)> for Point {
    fn from(f: (i32, i32)) -> Self {
        Self { inner: f }
    }
}

impl Point {
    fn neighbours(&self) -> [Self; 6] {
        let basis = Arrow::basis();
        [
            *self + basis[0],
            *self + basis[1],
            *self + basis[2],
            *self + basis[3],
            *self + basis[4],
            *self + basis[5],
        ]
    }
}

fn run1(input: &str) -> usize {
    let mut black = HashSet::new();
    for line in input.lines() {
        let mut rem = line;
        let mut arrow = Arrow::from((0,0));
        while rem.len() > 0 {
            let ta;
            (ta, rem) = Arrow::try_parse(rem);
            arrow += ta;
        }
        let point = Point::from((0,0)) + arrow;
        if black.contains(&point) {
            black.remove(&point);
        } else {
            black.insert(point);
        }
    }
    black.len()
}

fn flip(black: HashSet<Point>) -> HashSet<Point> {
    let mut res = black.clone();
    for point in black.union(&black.iter().map(|p| p.neighbours()).flatten().collect()) {
        if black.contains(point) {
            // Tile is black: flip only if 0 or more than 2 black tiles are adjacent
            let black_neighbours = point.neighbours().into_iter().filter(|p| black.contains(p)).count();
            if black_neighbours == 0 || black_neighbours > 2 {
                res.remove(point);
            }
        } else {
            // Tile is white: flip only if it has exactly 2 black neighbouring tiles
            if point.neighbours().into_iter().filter(|p| black.contains(p)).count() == 2 {
                res.insert(*point);
            }
        }
    }
    res
}

fn run2(input: &str) -> usize {
    let mut black = HashSet::new();
    for line in input.lines() {
        let mut rem = line;
        let mut arrow = Arrow::from((0,0));
        while rem.len() > 0 {
            let ta;
            (ta, rem) = Arrow::try_parse(rem);
            arrow += ta;
        }
        let point = Point::from((0,0)) + arrow;
        if black.contains(&point) {
            black.remove(&point);
        } else {
            black.insert(point);
        }
    }

    // Part 2
    for _ in 0..100 {
        black = flip(black);
    }
    black.len()
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
    assert_eq!(res, 10);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 411);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2208);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res,4092);
}
