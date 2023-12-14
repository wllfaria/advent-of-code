use ::std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    kind: u8,
    strength: Vec<u8>,
    rank: u16,
    bid: u16,
}

fn get_strength(cards: &str, is_joker: bool) -> Vec<u8> {
    let strength: Vec<u8> = cards.chars().fold(vec![], |mut acc, c| {
        let card_strength = match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            c if is_joker && c == 'J' => 0,
            c if !is_joker && c == 'J' => 9,
            'T' => 8,
            _ => c as u8 - b'2',
        };
        acc.push(card_strength);
        acc
    });
    strength
}

fn get_kind(cards: &str) -> u8 {
    let mut kind: i8 = -1;
    let mut hand: Vec<u8> = cards
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            let amount = acc.entry(c).or_insert(0);
            *amount += 1;
            acc
        })
        .values()
        .cloned()
        .collect();
    hand.sort();
    hand.iter().for_each(|x| match *x {
        1 => kind = 0,
        c if c == 2 && kind < 1 => kind = 1,
        2 => kind = 2,
        c if c == 3 && kind < 1 => kind = 3,
        c if c == 3 && kind == 1 => kind = 4,
        4 => kind = 5,
        _ => kind = 6,
    });
    kind as u8
}

fn part_one(input: &str) {
    let mut hands: Vec<Hand> = vec![];
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .for_each(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let kind = get_kind(cards);
            let strength = get_strength(cards, false);
            hands.push(Hand {
                kind,
                strength,
                rank: 0,
                bid: bid.parse::<u16>().unwrap(),
            })
        });

    hands.sort_by_key(|x| x.kind);
    hands.sort_by(|a, b| {
        let mut i = 0;
        let mut cmp = a.kind.cmp(&b.kind);
        while cmp == std::cmp::Ordering::Equal && i < 5 {
            cmp = a.strength[i].cmp(&b.strength[i]);
            i += 1;
        }
        cmp
    });
    hands.iter_mut().enumerate().for_each(|(i, x)| {
        x.rank = (i + 1) as u16;
    });
    let sum = hands.iter().fold(0 as u64, |mut acc, x| {
        acc += (x.bid as u64 * x.rank as u64) as u64;
        acc
    });

    println!("Part 1: {}", sum);
}

fn main() {
    let input = include_str!("input.txt");

    part_one(input);
}
