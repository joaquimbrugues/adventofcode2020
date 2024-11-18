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

fn extended_euclid(a: i128, b: i128) -> (i128, (i128, i128)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - (q * r));
        (old_s, s) = (s, old_s - (q * s));
        (old_t, t) = (t, old_t - (q * t));
    }
    (old_r, (old_s, old_t))
}

fn solve_pair_cong_equation((a1, m1): (i128, i128), (a2, m2): (i128, i128)) -> i128 {
    let (gcd, (b1, b2)) = extended_euclid(m1, m2);
    assert_eq!(gcd, 1); // All modules should be coprime in this problem
    ( m1 * b1 * a2 ) + ( m2 * b2 * a1 )
}

fn solve_system_cong_equations(coef: Vec<(i128, i128)>) -> i128 {
    if let Some(&(a, m)) = coef.first() {
        let mut acc = (a, m);
        for &(a, m) in coef[1..].iter() {
            acc.0 = solve_pair_cong_equation(acc, (a, m));
            acc.1 *= m;
            acc.0 %= acc.1;
        }
        while acc.0 < 0 {
            // Solutions are defined up to modulo mcm(m1,...,mN)
            acc.0 += acc.1;
        }
        acc.0
    } else {
        panic!("We should have multiple coefficients!")
    }
}

// Idea: Chinese remainder theorem, using Bézout's identity (extended Euclid's algorithm)
fn wrapper(input: &str) -> i128 {
    let cong_system = input.split(',').enumerate().filter(|(_, chunk)| chunk != &"x").map(|(e, chunk)| (- (e as i128), chunk.parse().unwrap())).collect();
    solve_system_cong_equations(cong_system)
}

fn run2(input: &str) -> i128 {
    let string = input.lines().nth(1).unwrap();
    wrapper(string)
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
    assert_eq!(res, 295);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2095);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 1068781);
}

#[test]
fn batch() {
    assert_eq!(wrapper("17,x,13,19"), 3417);
    assert_eq!(wrapper("67,7,59,61"), 754018);
    assert_eq!(wrapper("67,x,7,59,61"), 779210);
    assert_eq!(wrapper("67,7,x,59,61"), 1261476);
    assert_eq!(wrapper("1789,37,47,1889"), 1202161486);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 598411311431841);
}
