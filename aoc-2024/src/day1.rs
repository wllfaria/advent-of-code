use std::collections::{BinaryHeap, HashMap};

use crate::timed;

pub fn part_one(input: &str) -> u32 {
    let (left, right) = input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip::<_, _, BinaryHeap<u32>, BinaryHeap<u32>>();

    left.into_sorted_vec()
        .into_iter()
        .zip(right.into_sorted_vec())
        .fold(0, |acc, (left, right)| acc + left.abs_diff(right))
}

pub fn part_two(input: &str) -> u32 {
    let (left, right) = input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip::<_, _, Vec<u32>, Vec<u32>>();

    left.into_iter().fold(0, |acc, id| {
        acc + id * right.iter().filter(|&&v| v == id).collect::<Vec<_>>().len() as u32
    })
}

pub fn part_two_hash(input: &str) -> u32 {
    let (left, right) = input
        .lines()
        .flat_map(|line| line.split_once("   "))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip::<_, _, Vec<u32>, Vec<u32>>();

    let mut right_hash = HashMap::new();
    right.into_iter().for_each(|id| *right_hash.entry(id).or_insert(0) += 1);

    left.into_iter()
        .fold(0, |acc, id| acc + id * right_hash.get(&id).copied().unwrap_or_default())
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2024/day1/input.txt");

    match part.unwrap_or_default() {
        1 => {
            timed(part_one, input, 1);
        }
        2 => {
            timed(part_two, input, 2);
            timed(part_two_hash, input, 2);
        }
        _ => {
            timed(part_one, input, 1);
            timed(part_two, input, 2);
            timed(part_two_hash, input, 2);
        }
    };
}
