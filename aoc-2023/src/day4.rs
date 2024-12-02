use std::collections::HashMap;

use crate::timed;

fn find_matches(line: &str) -> i32 {
    let mut sum = 0;
    let mut found_divisor: bool = false;
    let mut map: HashMap<i32, i32> = HashMap::new();

    let numbers = &line[line.find(':').unwrap()..];

    let mut number_string = String::new();

    for c in numbers.chars() {
        if !c.is_ascii_digit() && !number_string.is_empty() {
            if !found_divisor {
                let n = number_string.parse::<i32>().unwrap();
                map.insert(n, 1);
            } else if found_divisor && map.contains_key(&number_string.parse::<i32>().unwrap()) {
                sum += 1;
            }
            number_string = String::new();
        } else if c == '|' {
            found_divisor = true;
            continue;
        } else if c.is_ascii_digit() {
            number_string.push(c);
        }
    }

    if map.contains_key(&number_string.parse::<i32>().unwrap()) {
        sum += 1;
    }

    sum
}

fn part_one(card: &str) -> i32 {
    let matches = find_matches(card);
    if matches > 1 {
        1 << (matches - 1)
    } else {
        matches
    }
}

fn part_two(cards: &str) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut current_card_id = 0;
    let mut total_cards = 0;

    for (i, card) in cards.lines().enumerate() {
        current_card_id = i as i32 + 1;
        let matches = find_matches(card);
        for j in 1..=matches {
            let copies = *map.get(&current_card_id).unwrap_or(&0);
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(current_card_id + j) {
                e.insert(1 + copies);
            } else {
                *map.get_mut(&(current_card_id + j)).unwrap() += 1 + copies;
            }
        }
    }

    for &copies in map.values() {
        total_cards += copies;
    }

    total_cards + current_card_id
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2023/day4/input.txt");

    match part.unwrap_or_default() {
        1 => timed(part_one, input, 1),
        2 => timed(part_two, input, 2),
        _ => {
            timed(part_one, input, 1);
            timed(part_two, input, 2)
        }
    };
}
