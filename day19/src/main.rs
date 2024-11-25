use std::{env,fs,process};
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    Term(u8),
    Rec(Vec<Vec<usize>>),
}

// If the string matches the rule {rule}, return the indexes up to which it matches (if any) 
fn match_rule(string: &[u8], start: usize, rule_id: &usize, rules: &HashMap<usize, Rule>) -> Vec<usize> {
    let mut matches = vec![];
    if start < string.len() {
        let rule = rules.get(rule_id).unwrap();
        match rule {
            Rule::Term(b) => { if string[start] == *b {
                matches.push(start+1);
            }
            },
            Rule::Rec(options) => {
                for seq in options {
                    let mut tmp_matches = vec![start];
                    for rule_id in seq {
                        if tmp_matches.len() == 0 {
                            break;
                        }
                        tmp_matches = tmp_matches.into_iter().map(|s| match_rule(string, s, rule_id, rules)).flatten().collect();
                    }
                    matches.append(&mut tmp_matches);
                }
            },
        }
    }
    matches
}

fn run1(input: &str) -> usize {
    let (rules_str, msgs_str) = input.split_once("\n\n").unwrap();
    let rules: HashMap<usize, Rule> = rules_str.lines().map(|line| {
        let (id, rule) = line.split_once(": ").unwrap();
        let id = id.parse().unwrap();
        let rule = if let Some(s) = rule.strip_prefix('"') {
            Rule::Term(s.as_bytes()[0])
        } else {
            let rec = rule.split('|').map(|s| s.split_whitespace().map(|t| t.parse().unwrap()).collect()).collect();
            Rule::Rec(rec)
        };
        (id, rule)
    }).collect();
    
    msgs_str.lines().filter(|line| {
        let matches = match_rule(line.as_bytes(), 0, &0, &rules);
        matches.into_iter().any(|l| l == line.len())
    }).count()
}

fn run2(input: &str) -> usize {
    let (rules_str, msgs_str) = input.split_once("\n\n").unwrap();
    let mut rules: HashMap<usize, Rule> = rules_str.lines().map(|line| {
        let (id, rule) = line.split_once(": ").unwrap();
        let id = id.parse().unwrap();
        let rule = if let Some(s) = rule.strip_prefix('"') {
            Rule::Term(s.as_bytes()[0])
        } else {
            let rec = rule.split('|').map(|s| s.split_whitespace().map(|t| t.parse().unwrap()).collect()).collect();
            Rule::Rec(rec)
        };
        (id, rule)
    }).collect();

    rules.insert(8, Rule::Rec(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Rec(vec![vec![42, 31], vec![42, 11, 31]]));
    
    msgs_str.lines().filter(|line| {
        let matches = match_rule(line.as_bytes(), 0, &0, &rules);
        matches.into_iter().any(|l| l == line.len())
    }).count()
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
    assert_eq!(res, 2);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 107);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 3);
}

#[test]
fn example3() {
    let input = fs::read_to_string("test2.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 12);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 321);
}
