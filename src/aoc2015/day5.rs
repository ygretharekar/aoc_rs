use std::collections::HashSet;
use fancy_regex::*;

pub fn run(input: Vec<String>) {
    let part1: u64 =  solve_part1(&input);
    println!("{}", part1);

    let part2: u64 =  solve_part2(&input);
    println!("{}", part2);
}

fn vowel_count(input: &[char]) -> u64 {
    let mut vowels:HashSet<char> = HashSet::new();
    vowels.insert('a');
    vowels.insert('e');
    vowels.insert('i');
    vowels.insert('o');
    vowels.insert('u');

    let mut count = 0;

    for c in input.iter() {
        if vowels.contains(c) {
            count += 1;
        }
    }

    count
}

fn solve_part1(input: &Vec<String>) -> u64 {
    let mut ans: u64 = 0;
    'outer: for line in input.iter() {
        let strng: Vec<char> = line.chars().collect();
        
        if vowel_count(&strng) < 3 {
            continue;
        }

        let mut cons: bool = false;

        for i in 0..(strng.len()-1) {
            if strng[i] == strng[i+1] {
                cons = true;
                break;
            }
        }

        if !cons {
            continue;
        }

        let bad_strings = vec!["ab", "cd", "pq", "xy"];

        for bad in bad_strings.iter() {
            if line.find(bad).is_some() {
                continue 'outer;
            }
        }

        ans += 1;
    }

    ans
}

fn solve_part2(input: &Vec<String>) -> u64 {
    let mut sum: u64 = 0;

    let rule1 = Regex::new(r"(..).*\1").unwrap();
    let rule2 = Regex::new(r"(.).\1").unwrap();

    for line in input.iter() {
        if !rule1.is_match(line).unwrap() || !rule2.is_match(line).unwrap() {
            continue;
        }

        sum += 1;
    }

    sum
}