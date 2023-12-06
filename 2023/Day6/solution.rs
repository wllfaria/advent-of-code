fn calculate_ways_to_win(time: i64, distance: i64) -> i64 {
    let mut total: i64 = 0;

    for i in 0..time {
        let hold_time = time - i;
        let remaining_time = time - hold_time;
        let travel_distance = hold_time * remaining_time;
        if travel_distance > distance {
            total += 1;
        }
    }

    total
}

fn get_value(input: &mut &[u8]) -> i64 {
    let mut val = 0;
    while let [c, ..] = input {
        match c {
            c @ b'0'..=b'9' => {
                val = val * 10 + (c - b'0') as usize;
            }
            b'\n' => break,
            _ => {}
        }

        *input = &input[1..];
    }

    val as i64
}

fn part_one(input: &str) {
    let (time, distance) = input.split_once("\n").unwrap();
    let mut total: u32 = 1;

    let time: Vec<u32> = time
        .strip_prefix("Time: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let distance: Vec<u32> = distance
        .strip_prefix("Distance: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    total *= time
        .iter()
        .enumerate()
        .map(|(i, v)| calculate_ways_to_win(*v as i64, distance[i] as i64) as u32)
        .fold(1, |acc, x| acc * x);

    println!("Part one: {}", total);
}

fn part_two(input: &str) {
    let mut iter = input.split("\n");
    let mut time_line = iter.next().unwrap().as_bytes();
    let mut distance_line = iter.next().unwrap().as_bytes();

    let time = get_value(&mut time_line);
    let distance = get_value(&mut distance_line);

    let total = calculate_ways_to_win(time, distance);

    println!("Part two: {}", total);
}

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}
