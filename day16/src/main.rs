use std::{env,fs,process};
use std::collections::{HashSet, HashMap};

fn run1(input: &str) -> u32 {
    let mut chunks = input.split("\n\n");
    let intervals: Vec<(u32, u32)> = chunks.nth(0)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace())
        .flatten()
        .filter(|word| word.contains('-'))
        .map(|word| word.split_once('-').unwrap())
        .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
        .collect();

    let mut sum = 0;
    for n in chunks.nth(1).unwrap().lines().skip(1).map(|line| line.split(',')).flatten().map(|s| s.parse::<u32>().unwrap()) {
        let mut searching = true;
        let mut i = 0;
        while searching && i < intervals.len() {
            searching &= (n < intervals[i].0) || (n > intervals[i].1);
            i += 1;
        }
        if searching {
            sum += n;
        }
    }
    sum
}

fn run2(input: &str) -> u64 {
    let mut chunks = input.split("\n\n");
    let intervals: HashMap<&str, Vec<(u32, u32)>> = chunks.next()
        .unwrap()
        .lines()
        .map(|line| {
            let (field, rest) = line.split_once(": ").unwrap();
            let ints = rest.split_whitespace()
                .filter(|word| word.contains('-'))
                .map(|word| word.split_once('-').unwrap())
                .map(|(s1,s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
                .collect();
            (field, ints)
        })
        .collect();
    let my_ticket: Vec<u32> = chunks.next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let length = my_ticket.len();
    let mut cypher: HashMap<&str, HashSet<usize>> = intervals.keys().map(|&k| (k, (0..length).collect())).collect();

    let mut lines = chunks.next().unwrap().lines().skip(1);

    while cypher.values().any(|ind|  ind.len() > 1) {
        if let Some(line) = lines.next() {
            let ticket: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            // Only consider tickets that could match at least one of the intervals
            if ticket.iter().all(|n| intervals.values().flatten().any(|(a,b)| (a <= n) && (n <= b))) { 
                for (i, &n) in ticket.iter().enumerate() {
                    for (field, inds) in cypher.iter_mut() {
                        let ints = intervals.get(field).unwrap();
                        if ints.iter().all(|&(a,b)| (n < a) || (b < n)) {
                            inds.remove(&i);
                        }
                    }
                }

                let mut changed = true;
                while changed {
                    changed = false;
                    for i in 0..length {
                        // If some field already has only one index, remove that index from any other
                        // field
                        match cypher.values().filter(|inds| inds.contains(&i) && inds.len() == 1).count() {
                            0 => {},
                            1 => {
                                // Remove i from any other set
                                for inds in cypher.values_mut().filter(|inds| inds.len() > 1) {
                                    changed |= inds.remove(&i);
                                }
                            },
                            _ => panic!("Multiple fields map to the index {i}!"),
                        }

                        // If only one field contains an index, erase all other indices from that field
                        let mut inds: Vec<&mut HashSet<usize>> = cypher.values_mut().filter(|inds| inds.contains(&i)).collect();
                        match inds.len() {
                            0 => panic!("The index {i} does not correspond to any field!"),
                            1 => {
                                if inds[0].len() > 1 {
                                    // Remove all other indices from this set
                                    inds[0].retain(|&j| i == j);
                                    changed = true;
                                }
                            },
                            _ => {},
                        }
                    } 
                }
            }
        } else {
            unreachable!();
        }
    }
    assert!(cypher.values().all(|inds| inds.len() == 1));
    let mut prod = 1;
    for (_, ind) in cypher.iter().filter(|(k,_)| k.starts_with("departure")) {
        let ind = *ind.iter().nth(0).unwrap();
        prod *= my_ticket[ind] as u64;
    }
    prod
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
    assert_eq!(res, 71);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 23054);
}

//#[test]
//fn example2() {
    //let input = fs::read_to_string("test.txt").unwrap();
    //let res = run2(&input);
    //assert_eq!(res,42);
//}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 51240700105297);
}
