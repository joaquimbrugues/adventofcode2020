use std::{env,fs,process};
use std::collections::{HashSet, HashMap};

fn run1(input: &str) -> usize {
    // Idea: Encode the list of rules as a directed graph (a HashMap that for each key stores the
    // number of adjacent neighbours), but directly in reverse. Then, do DFS to count every
    // possible reachable color
    let mut graph: HashMap<(&str, &str), Vec<(&str, &str)>> = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once(" contain ").unwrap();
        let first = first.strip_suffix(" bags").unwrap();
        let col = first.split_once(' ').unwrap();
        if second != "no other bags." {
            for chunk in second.split(", ") {
                let aux = chunk.split_whitespace().collect::<Vec<_>>();
                let key = (aux[1], aux[2]);
                match graph.get_mut(&key) {
                    Some(v) => { v.push(col) },
                    None => { graph.insert(key, vec![col]); },
                }
            }
        }
    }
    
    let mut stack = vec![("shiny", "gold")];
    let mut found = HashSet::new();
    while let Some(col) = stack.pop() {
        if !found.contains(&col) {
            if col != ("shiny", "gold") {
                found.insert(col);
            }
            if let Some(neighs) = graph.get(&col) {
                for &n in neighs {
                    stack.push(n);
                }
            }
        }
    }
    found.len()
}

fn compute_recursive_bags(graph: &HashMap<&str, Vec<(u32, &str)>>, bag: &str) -> u32 {
    1 + graph.get(bag).unwrap().iter().map(|(num, b)| num * compute_recursive_bags(graph, b)).sum::<u32>()
}

fn run2(input: &str) -> u32 {
    // Idea: Same parsing, and a recursive function to count things
    let mut graph: HashMap<&str, Vec<(u32, &str)>> = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once(" contain ").unwrap();
        let key = first.strip_suffix(" bags").unwrap();
        if second == "no other bags." {
            graph.insert(key, vec![]);
        } else {
            let val = second.split(", ").map(|chunk| {
                let (sn, rest) = chunk.split_once(' ').unwrap();
                let num = sn.parse().unwrap();
                let (v, _) = rest.rsplit_once(' ').unwrap();
                (num, v)
            }).collect();
            graph.insert(key, val);
        }
    }
    compute_recursive_bags(&graph, "shiny gold") - 1
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
    assert_eq!(res, 4);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 370);
}

#[test]
fn example21() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 32);
}

#[test]
fn example22() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 126);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 29547);
}
