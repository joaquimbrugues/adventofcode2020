use std::{env,fs,process};
use std::collections::HashSet;

struct Grid {
    height: usize,
    width: usize,
}

impl Grid {
    fn new(height: usize, width: usize) -> Self {
        Self { height, width, }
    }

    fn neighbours(&self, h: usize, w: usize) -> Vec<(usize, usize)> {
        if h >= self.height || w >= self.width {
            panic!("Requested neighbours out of bounds");
        }
        let mut vneighs = vec![h];
        if h > 0 {
            vneighs.push(h-1);
        }
        if h + 1 < self.height {
            vneighs.push(h+1);
        }
        let mut hneighs = vec![w];
        if w > 0 {
            hneighs.push(w-1);
        }
        if w + 1 < self.width {
            hneighs.push(w+1);
        }
        let mut neighs = vec![];
        for &vn in vneighs.iter() {
            for &hn in hneighs.iter() {
                if vn != h || hn != w {
                    neighs.push((vn, hn));
                }
            }
        }
        neighs
    }

    fn visible(&self, seats: &HashSet<(usize, usize)>, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut vis = vec![];
        // NW
        let mut nw = (pos.0, pos.1);
        while nw.0 > 0 && nw.1 > 0 {
            nw.0 -= 1; nw.1 -= 1;
            if seats.contains(&nw) { break; }
        }
        if nw != pos && seats.contains(&nw) {
            vis.push(nw);
        }
        // N
        let mut n = (pos.0, pos.1);
        while n.0 > 0 {
            n.0 -= 1;
            if seats.contains(&n) { break; }
        }
        if n != pos && seats.contains(&n) {
            vis.push(n);
        }
        // NE
        let mut ne = (pos.0, pos.1);
        while ne.0 > 0 && ne.1 < self.width {
            ne.0 -= 1; ne.1 += 1;
            if seats.contains(&ne) { break; }
        }
        if ne != pos && seats.contains(&ne) {
            vis.push(ne);
        }
        // E
        let mut e = (pos.0, pos.1);
        while e.1 < self.width {
            e.1 += 1;
            if seats.contains(&e) { break; }
        }
        if e != pos && seats.contains(&e) {
            vis.push(e);
        }
        // SE
        let mut se = (pos.0, pos.1);
        while se.0 < self.height && se.1 < self.width {
            se.0 += 1; se.1 += 1;
            if seats.contains(&se) { break; }
        }
        if se != pos && seats.contains(&se) {
            vis.push(se);
        }
        // S
        let mut s = (pos.0, pos.1);
        while s.0 < self.height {
            s.0 += 1;
            if seats.contains(&s) { break; }
        }
        if s != pos && seats.contains(&s) {
            vis.push(s);
        }
        // SW
        let mut sw = (pos.0, pos.1);
        while sw.0 < self.height && sw.1 > 0 {
            sw.0 += 1; sw.1 -= 1;
            if seats.contains(&sw) { break; }
        }
        if sw != pos && seats.contains(&sw) {
            vis.push(sw);
        }
        // W
        let mut w = (pos.0, pos.1);
        while w.1 > 0 {
            w.1 -= 1;
            if seats.contains(&w) { break; }
        }
        if w != pos && seats.contains(&w) {
            vis.push(w);
        }
        vis
    }
}

fn run1(input: &str) -> usize {
    let mut v = 0;
    let mut h = 0;
    let mut seats = HashSet::new();
    for line in input.lines() {
        h = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                'L' => { seats.insert((v, h)); },
                _ => unreachable!(),
            }
            h += 1;
        }
        v += 1;
    }
    let grid = Grid::new(v, h);

    let mut occupied = HashSet::new();
    loop {
        let mut new_occupied = occupied.clone();
        for seat in seats.iter() {
            if occupied.contains(&seat) {
                // If the seat is occupied and 4 or more adjacent seats are occupied, it becomes
                // empty.
                if grid.neighbours(seat.0, seat.1).iter().filter(|&n| occupied.contains(n)).count() >= 4 {
                    new_occupied.remove(&seat);
                }
            } else {
                // If the seat is free and no adjacent seat is occupied, it becomes occupied
                if grid.neighbours(seat.0, seat.1).iter().filter(|&n| occupied.contains(n)).count() == 0 {
                    new_occupied.insert(seat);
                }
            }
        }
        if new_occupied == occupied {
            return occupied.len()
        } else {
            occupied = new_occupied;
        }
    }
}

fn run2(input: &str) -> usize {
    let mut v = 0;
    let mut h = 0;
    let mut seats = HashSet::new();
    for line in input.lines() {
        h = 0;
        for c in line.chars() {
            match c {
                '.' => {},
                'L' => { seats.insert((v, h)); },
                _ => unreachable!(),
            }
            h += 1;
        }
        v += 1;
    }
    let grid = Grid::new(v, h);

    let mut occupied = HashSet::new();
    loop {
        let mut new_occupied = occupied.clone();

        for &seat in seats.iter() {
            if occupied.contains(&seat) {
                // If the seat is occupied and 5 or more adjacent seats are occupied, it becomes
                // empty.
                if grid.visible(&seats, seat).iter().filter(|&n| occupied.contains(n)).count() >= 5 {
                    new_occupied.remove(&seat);
                }
            } else {
                // If the seat is free and no adjacent seat is occupied, it becomes occupied
                if grid.visible(&seats, seat).iter().filter(|&n| occupied.contains(n)).count() == 0 {
                    new_occupied.insert(seat);
                }
            }
        }

        if new_occupied == occupied {
            return occupied.len();
        } else {
            occupied = new_occupied;
        }
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
    assert_eq!(res, 37);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2316);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 26);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2128);
}
