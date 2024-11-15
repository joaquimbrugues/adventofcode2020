use std::{env,fs,process};
use std::collections::{HashSet, HashMap};

fn run1(input: &str) -> u32 {
    let mut valid = 0;
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for pass in input.split("\n\n") {
        let keys: HashSet<&str> = pass
            .split_whitespace()
            .map(|chunk| chunk.split_once(':').unwrap().0)
            .collect();
        if required.iter().all(|field| keys.contains(field)) {
            valid += 1;
        }
    }
    valid
}

fn is_valid_number(s: &str, dig: usize, min: u16, max: u16) -> bool {
    assert!(min < max);
    if s.len() == dig {
        let num: u16 = s.parse().unwrap();
        num >= min && num <= max
    } else {
        false
    }
}

fn is_valid(passport: &str) -> bool {
    let pass: HashMap<&str, &str> = passport.split_whitespace().map(|chunk| chunk.split_once(':').unwrap()).collect();
    // Check birth year: four digits, from 1920 to 2002
    match pass.get("byr") {
        None => return false,
        Some(year) => {
            if !is_valid_number(year, 4, 1920, 2002) {
                return false;
            }
        },
    }
    // Check issue year: four digits, from 2010 to 2020
    match pass.get("iyr") {
        None => return false,
        Some(year) => {
            if !is_valid_number(year, 4, 2010, 2020) {
                return false;
            }
        },
    }
    // Check expiration year: four digits, from 2020 to 2030
    match pass.get("eyr") {
        None => return false,
        Some(year) => {
            if !is_valid_number(year, 4, 2020, 2030) {
                return false;
            }
        },
    }
    // Check height
    match pass.get("hgt") {
        None => return false,
        Some(hgt) => {
            match hgt.strip_suffix("cm") {
                Some(rest) => {
                    // It must be between 150 and 193
                    let height: u8 = rest.parse().unwrap();
                    if height < 150 || height > 193 {
                        return false;
                    }
                },
                None => {
                    match hgt.strip_suffix("in") {
                        None => return false,
                        Some(rest) => {
                            // It must be between 59 and 76
                            let height: u8 = rest.parse().unwrap();
                            if height < 59 || height > 76 {
                                return false;
                            }
                        }
                    }
                },
            }
        },
    }
    // Check hair color
    match pass.get("hcl") {
        None => return false,
        Some(hcl) => {
            match hcl.strip_prefix('#') {
                None => return false,
                Some(rest) => {
                    if rest.len() != 6 || !rest.chars().all(|c| c.is_ascii_hexdigit() && !(c.is_ascii_uppercase())) {
                        return false;
                    }
                },
            }
        }
    }
    // Check eye color
    match pass.get("ecl") {
        None => return false,
        Some(ecl) => {
            let colors = HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
            if !colors.contains(ecl) {
                return false;
            }
        },
    }
    // Check passport ID
    match pass.get("pid") {
        None => return false,
        Some(pid) => {
            if pid.len() != 9 || !pid.chars().all(|c| c.is_digit(10)) {
                return false;
            }
        },
    }
    // All relevant fields have been successfully accounted for
    true
}

fn run2(input: &str) -> usize {
    input.split("\n\n").filter(|pass| is_valid(pass)).count()
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
    assert_eq!(res, 213);
}

#[test]
fn invalid() {
    let input = fs::read_to_string("invalid.txt").unwrap();
    assert!(input.split("\n\n").all(|pass| !is_valid(pass)));
}

#[test]
fn valid() {
    let input = fs::read_to_string("valid.txt").unwrap();
    assert!(input.split("\n\n").all(|pass| is_valid(pass)));
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 2);
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, 147);
}
