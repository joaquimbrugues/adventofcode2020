use std::{env,fs,process};
use std::ops::Mul;
use std::collections::{HashMap, HashSet};

#[derive(Debug,Clone,Copy)]
enum Mov {
    R(u8),
    SR(u8),
}

impl Mov {
    fn all() -> [Self; 8] {
        [
            Self::R(0), Self::SR(0),
            Self::R(1), Self::SR(1),
            Self::R(2), Self::SR(2),
            Self::R(3), Self::SR(3),
        ]
    }

    fn r(&self) -> &u8 {
        match self {
            Self::R(r) => r,
            Self::SR(r) => r,
        }
    }

    fn s(&self) -> bool {
        match self {
            Self::R(_) => false,
            Self::SR(_) => true,
        }
    }
}

impl Mul for Mov {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Self::R(r), Self::R(s)) => {
                Self::R((r + s) % 4)
            },
            (Self::SR(r), Self::R(s)) => {
                Self::SR((r +s) % 4)
            },
            (Self::R(r), Self::SR(s)) => {
                let r = r % 4;
                Self::SR((s + (4 - r)) % 4)
            },
            (Self::SR(r), Self::SR(s)) => {
                let r = r % 4;
                Self::R((s + (4 - r)) % 4)
            },
        }
    }
}

fn get_index_with_bounds(y: usize, x: usize, bound: usize, mov: &Mov) -> (usize, usize) {
    let (y, mut x): (usize, usize) = match mov.r() % 4 {
        0 => (y,x),
        1 => (x, bound - y - 1),
        2 => (bound - y - 1, bound - x - 1),
        3 => (bound - x - 1, y),
        _ => unreachable!(),
    };
    if mov.s() {
        x = bound - x - 1;
    }
    (y, x)
}

fn get_index_with_bounds_i(y: isize, x: isize, bound: usize, mov: &Mov) -> (isize, isize) {
    let bound = bound as isize;
    let (y, mut x): (isize, isize) = match mov.r() % 4 {
        0 => (y,x),
        1 => (x, bound - y - 1),
        2 => (bound - y - 1, bound - x - 1),
        3 => (bound - x - 1, y),
        _ => unreachable!(),
    };
    if mov.s() {
        x = bound - x - 1;
    }
    (y, x)
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Dir { N, E, S, W, }

impl Dir {
    fn dirs() -> [Self; 4] {
        [ Self::N, Self::E, Self::S, Self::W, ]
    }
}

#[derive(Clone)]
struct Tile { inner: Vec<Vec<bool>>, mov: Mov, len: usize, }

impl From<Vec<Vec<bool>>> for Tile {
    fn from(inner: Vec<Vec<bool>>) -> Self {
        let n = inner.len();
        assert!(n > 0);
        assert!(inner.iter().all(|row| row.len() == n));
        Self { inner, mov: Mov::R(0), len: n, }
    }
}

// The Mov acts on our tile ON THE LEFT
impl Mul<Mov> for Tile {
    type Output = Self;

    fn mul(self, rhs: Mov) -> Self {
        Self { inner: self.inner.clone(), mov: rhs * self.mov, len: self.len, }
    }
}

impl Tile {
    fn get(&self, y: usize, x: usize) -> bool {
        let (y, x) = get_index_with_bounds(y, x, self.len, &self.mov);
        self.inner[y][x]
    }

    // {dir} identifies the boundary for self
    fn match_boundary(&self, other: &Self, dir: Dir) -> bool {
        assert_eq!(self.len, other.len);
        match dir {
            Dir::N => {
                (0..self.len).all(|i| self.get(0, i) == other.get(other.len - 1, i))
            },
            Dir::S => {
                (0..self.len).all(|i| self.get(self.len - 1, i) == other.get(0, i))
            },
            Dir::W => {
                (0..self.len).all(|i| self.get(i, 0) == other.get(i, other.len - 1))
            },
            Dir::E => {
                (0..self.len).all(|i| self.get(i, self.len - 1) == other.get(i, 0))
            },
        }
    }
}

fn parse_input(input: &str) -> HashMap<u64, Tile> {
    input.split("\n\n").map(|chunk| {
        let mut lines = chunk.lines();
        let first = lines.next().unwrap();
        let id = first.strip_prefix("Tile ").unwrap().strip_suffix(':').unwrap().parse().unwrap();

        let v: Vec<Vec<bool>> = lines.map(|line| {
            line.chars().map(|c| c == '#').collect()
        }).collect();
        (id, Tile::from(v))
    }).collect()
}

fn find_corners(tiles: HashMap<u64, Tile>) -> Vec<u64> {
    let mut corners = Vec::new();

    for (&id, tile) in tiles.iter() {
        if corners.len() >= 4 {
            break;
        }

        let mut matching_sides = HashSet::new();
        for tile2 in tiles.iter().filter(|(&id2, _)| id != id2).map(|e| e.1) {
            if matching_sides.len() >= 4 { break; }
            for m in Mov::all() {
                if matching_sides.len() >= 4 { break; }
                let tile2 = tile2.clone() * m;
                for s in Dir::dirs().into_iter().filter(|d| !matching_sides.contains(d)) {
                    if tile.match_boundary(&tile2, s) {
                        matching_sides.insert(s);
                        break;
                    }
                }
            }
        }

        if matching_sides.len() < 2 { panic!("Should have at least 2 matching sides!"); }
        if matching_sides.len() == 2 {
            corners.push(id);
        }
    }

    corners
}

fn run1(input: &str) -> u64 {
    find_corners(parse_input(input)).into_iter().product()
}

fn assemble_image(mut tiles: HashMap<u64, Tile>) -> HashMap<(usize, usize), Tile> {
    // First, compute the side length of the array
    let side = {
        let mut side = 0;
        while side * side < tiles.len() { side += 1; }
        assert_eq!(side * side, tiles.len());
        side
    };

    // Find the first corner
    let mut top_left = None;

    for (&id, tile) in &tiles {
        if top_left.is_some() { break; }
        for m in Mov::all() {
            if top_left.is_some() { break; }
            let tile = tile.clone() * m;
            let mut matching_sides = HashSet::new();
            for tile2 in tiles.iter().filter(|(&id2, _)| id != id2).map(|e| e.1) {
                let mut matched_tile = false;
                for m2 in Mov::all() {
                    if matched_tile { break; }
                    let tile2 = tile2.clone() * m2;
                    for s in Dir::dirs().into_iter().filter(|d| !matching_sides.contains(d)) {
                        if tile.match_boundary(&tile2, s) {
                            matched_tile = true;
                            matching_sides.insert(s);
                            break;
                        }
                    }
                }
            }

            if matching_sides.len() == 2 && matching_sides.contains(&Dir::S) && matching_sides.contains(&Dir::E) {
                top_left = Some((id, tile));
            }
        }
    }

    // Initialize
    let mut tilemap = HashMap::with_capacity(side * side);
    let (id, top_left) = top_left.unwrap();
    tiles.remove(&id);
    tilemap.insert((0,0), top_left);

    let mut stack_to_check = vec![(0,1), (1,0)];
    let neighbours = |coord: &(usize, usize)| {
        let mut res = Vec::new();
        if coord.0 > 0 { res.push(((coord.0 - 1, coord.1), Dir::N)); }
        if coord.1 > 0 { res.push(((coord.0, coord.1 - 1), Dir::W)); }
        if coord.0 < side - 1 { res.push(((coord.0 + 1, coord.1), Dir::S)); }
        if coord.1 < side - 1 { res.push(((coord.0, coord.1 + 1), Dir::E)); }
        res
    };

    let mut placed_tiles = HashSet::from([id]);
    while let Some((i, j)) = stack_to_check.pop() {
        if tilemap.contains_key(&(i,j)) {
            continue;
        }
        let mut to_place = None;
        for (id, tile) in &tiles {
            if to_place.is_some() { break; }
            if placed_tiles.contains(id) { continue; }
            for m in Mov::all() {
                if to_place.is_some() { break; }
                let tile = tile.clone() * m;
                if neighbours(&(i,j)).into_iter().all(|(n,dir)| {
                    match tilemap.get(&n) {
                        None => true,
                        Some(tile2) => tile.match_boundary(tile2, dir),
                    }
                }) {
                    // Confirm tile placement
                    to_place = Some(*id);
                    tilemap.insert((i,j), tile);
                    neighbours(&(i,j)).into_iter().filter(|(n,_)| !tilemap.contains_key(n)).for_each(|(n, _)| stack_to_check.push(n));
                }
            }
        }

        placed_tiles.insert(to_place.unwrap());
    }

    (0..side).for_each(|i| (0..side).for_each(|j| assert!(tilemap.contains_key(&(i,j)))));

    tilemap
}

fn condense_image(tilemap: HashMap<(usize, usize), Tile>) -> (HashSet<(usize, usize)>, usize) {
    let maplen = tilemap.keys().map(|e| e.0).max().unwrap() + 1;
    let tile_len = tilemap.get(&(0,0)).unwrap().len - 2;
    let side = tile_len * maplen;
    let mut res = HashSet::with_capacity(side * side);
    for y in 0..side {
        for x in 0..side {
            let outer_i = y / tile_len;
            let outer_j = x / tile_len;
            let inner_i = (y % tile_len) + 1;
            let inner_j = (x % tile_len) + 1;
            if tilemap.get(&(outer_i, outer_j)).unwrap().get(inner_i, inner_j) {
                res.insert((y, x));
            }
        }
    }
    (res, side)
}

fn run2(input: &str) -> usize {
    let (image, side) = condense_image(assemble_image(parse_input(input)));

    let sea_monster = |(y,x): (usize, usize)| {[ (y+1,x), (y+2,x+1), (y+2, x+4), (y+1,x+5), (y+1,x+6), (y+2,x+7), (y+2, x+10), (y+1,x+11), (y+1,x+12), (y+2,x+13), (y+2,x+16), (y+1,x+17), (y+1,x+18), (y,x+18), (y+1,x+19)]};
    let mut monsters = HashSet::with_capacity(side * side);

    for y in 0..(side-3) {
        for x in 0..(side-20) {
            for m in Mov::all() {
                let monster = sea_monster((y,x));
                if monster.iter().all(|&(i,j)| image.contains(&get_index_with_bounds(i, j, side, &m))) {
                    monster.iter().for_each(|&(i,j)| {monsters.insert(get_index_with_bounds(i,j, side, &m));});
                }
            }
        }
    }

    image.difference(&monsters).count()
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
    assert_eq!(res, 20899048083289);
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

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1714);
}
