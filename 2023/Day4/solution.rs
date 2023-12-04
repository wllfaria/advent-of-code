use std::collections::HashMap;

fn find_matches(line: &str) -> i32 {
    let mut sum = 0;
    let mut found_divisor: bool = false;
    let mut map: HashMap<i32, i32> = HashMap::new();

    let numbers = &line[line.find(':').unwrap()..];

    let mut number_string = String::new();

    for c in numbers.chars() {
        if !c.is_digit(10) && number_string.len() > 0 {
            if !found_divisor {
                let n = number_string.parse::<i32>().unwrap();
                map.insert(n, 1);
            } else if found_divisor {
                if map.contains_key(&number_string.parse::<i32>().unwrap()) {
                    sum += 1;
                }
            }
            number_string = String::new();
        } else if c == '|' {
            found_divisor = true;
            continue;
        } else if c.is_digit(10) {
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
    let result = if matches > 1 {
        1 << (matches - 1)
    } else {
        matches
    };
    result
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
            if map.contains_key(&(current_card_id + j)) {
                *map.get_mut(&(current_card_id + j)).unwrap() += 1 + copies;
            } else {
                map.insert(current_card_id + j, 1 + copies);
            }
        }
    }

    for &copies in map.values() {
        total_cards += copies;
    }

    total_cards + current_card_id
}

fn main() {
    let input = include_str!("input.txt");

    let mut part_one_total: i32 = 0;

    for line in input.lines() {
        part_one_total += part_one(line)
    }

    let part_two_total = part_two(input);

    println!("Part one: {}", part_one_total);
    println!("Part two: {}", part_two_total);
}
