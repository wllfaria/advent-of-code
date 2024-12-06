use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::timed;

fn part_one(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();

    rules
        .lines()
        .map(|line| line.split_once("|").unwrap())
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .for_each(|(left, right)| rules_map.entry(left).or_default().push(right));

    updates
        .lines()
        .map(|line| line.split(","))
        .map(|line| line.map(|num| num.parse::<u32>().unwrap()).collect_vec())
        .filter(|update| {
            let mut seen = HashSet::new();
            for &page in update {
                if let Some(following_pages) = rules_map.get(&page) {
                    if following_pages.iter().any(|&next| seen.contains(&next)) {
                        return false;
                    }
                }
                seen.insert(page);
            }
            true
        })
        .map(|valid_updates| *valid_updates.get(valid_updates.len() / 2).unwrap())
        .sum()
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2024/day5/input.txt");

    match part.unwrap_or_default() {
        1 => {
            timed(part_one, input, 1);
        }
        2 => {}
        _ => {
            timed(part_one, input, 1);
        }
    };
}
