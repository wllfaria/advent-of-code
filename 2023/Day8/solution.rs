use ::std::collections::HashMap;

fn part_one(input: &str) {
    let (directions, lines) = input.split_once("\n\n").unwrap();
    let start = "AAA";
    let end = "ZZZ";
    let mut curr: &str = start;

    let directions = directions
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid direction"),
        })
        .collect::<Vec<u64>>();

    let map: HashMap<&str, Vec<&str>> = lines.lines().fold(HashMap::new(), |mut acc, line| {
        let (key, values) = line.split_once(" = ").unwrap();
        let entry = acc.entry(key).or_insert(vec![]);
        *entry = vec![&values[1..4], &values[6..9]];
        acc
    });

    let mut i: u64 = 0;
    let mut count: u64 = 0;

    loop {
        if curr == end {
            break;
        }

        curr = map.get(curr).unwrap()[directions[i as usize] as usize];
        count += 1;

        if i == (directions.len() - 1).try_into().unwrap() {
            i = 0;
        } else {
            i += 1;
        }
    }

    println!("Part 1: {}", count);
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
}
