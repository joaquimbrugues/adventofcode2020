use std::{env,fs,process};
use std::collections::{HashSet,HashMap};
use std::cmp;

fn run1(input: &str) -> u32 {
    let mut ingredients: HashMap<&str, u32> = HashMap::new();
    let mut possible_allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut allergens: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once("(contains ").unwrap();
        let second = second.strip_suffix(')').unwrap();
        let first = first.trim();
        let ing: HashSet<&str> = first.split_whitespace().collect();
        for i in ing.iter() {
            if let Some(count) = ingredients.get_mut(i) {
                *count += 1;
            } else {
                ingredients.insert(i, 1);
            }
        }
        for allergen in second.split(", ") {
            let ingr = ing.clone();
            if let Some(set) = possible_allergens.get_mut(allergen) {
                *set = set.intersection(&ingr).map(|&s| s).collect::<HashSet<_>>();
            } else {
                if let Some(ingredient) = allergens.get(allergen) {
                    assert!(ingr.contains(ingredient));
                } else {
                    possible_allergens.insert(allergen, ingr);
                }
            }

            if let Some(set) = possible_allergens.get(allergen) {
                if set.len() == 1 {
                    let set = set.clone();
                    let ingredient = set.iter().next().unwrap();
                    allergens.insert(allergen, ingredient);
                    possible_allergens.remove(allergen);
                    possible_allergens.iter_mut().for_each(|(_, vals)| { vals.remove(ingredient); } );
                }
            }
        }
    }

    while possible_allergens.len() > 0 {
        for (all, ing) in possible_allergens.iter().filter(|(_, set)| set.len() == 1) {
            let ing = ing.iter().next().unwrap();
            allergens.insert(all, ing);
        }
        for (all, ing) in allergens.iter() {
            possible_allergens.remove(all);
            possible_allergens.iter_mut().for_each(|(_, vals)| {vals.remove(ing); });
        }
    }

    //ingredients.difference(&allergens.values().map(|&s| s).collect()).count()
    for i in allergens.values() {
        ingredients.remove(i);
    }
    ingredients.values().sum()
}

fn order_by_first(left: &(&str, &str), right: &(&str, &str)) -> cmp::Ordering {
    left.0.cmp(right.0)
}

fn run2(input: &str) -> String {
    let mut possible_allergens: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut allergens: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let (first, second) = line.split_once("(contains ").unwrap();
        let second = second.strip_suffix(')').unwrap();
        let first = first.trim();
        let ing: HashSet<&str> = first.split_whitespace().collect();
        for allergen in second.split(", ") {
            let ingr = ing.clone();
            if let Some(set) = possible_allergens.get_mut(allergen) {
                *set = set.intersection(&ingr).map(|&s| s).collect::<HashSet<_>>();
            } else {
                if let Some(ingredient) = allergens.get(allergen) {
                    assert!(ingr.contains(ingredient));
                } else {
                    possible_allergens.insert(allergen, ingr);
                }
            }

            if let Some(set) = possible_allergens.get(allergen) {
                if set.len() == 1 {
                    let set = set.clone();
                    let ingredient = set.iter().next().unwrap();
                    allergens.insert(allergen, ingredient);
                    possible_allergens.remove(allergen);
                    possible_allergens.iter_mut().for_each(|(_, vals)| { vals.remove(ingredient); } );
                }
            }
        }
    }

    while possible_allergens.len() > 0 {
        for (all, ing) in possible_allergens.iter().filter(|(_, set)| set.len() == 1) {
            let ing = ing.iter().next().unwrap();
            allergens.insert(all, ing);
        }
        for (all, ing) in allergens.iter() {
            possible_allergens.remove(all);
            possible_allergens.iter_mut().for_each(|(_, vals)| {vals.remove(ing); });
        }
    }

    let mut ordered: Vec<(&str, &str)> = allergens.into_iter().collect();
    ordered.sort_by(order_by_first);
    ordered.into_iter().fold(String::new(), |acc, ing| {
        if acc.len() > 0 {
            format!("{acc},{}", ing.1) 
        } else {
            ing.1.to_owned()
        }
    })
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
    assert_eq!(res, 5);
}

#[test]
fn input1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run1(&input);
    assert_eq!(res, 2724);
}

#[test]
fn example2() {
    let input = fs::read_to_string("test.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, "mxmxvkd,sqjhc,fvjkl".to_owned());
}

#[test]
fn input2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res = run2(&input);
    assert_eq!(res, "xlxknk,cskbmx,cjdmk,bmhn,jrmr,tzxcmr,fmgxh,fxzh".to_owned());
}
