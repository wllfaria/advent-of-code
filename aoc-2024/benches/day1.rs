fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() -> u32 {
    let input = include_str!("../../inputs/2024/day1/input.txt");
    aoc_2024::day1::part_one(input)
}

#[divan::bench]
fn part_2() -> u32 {
    let input = include_str!("../../inputs/2024/day1/input.txt");
    aoc_2024::day1::part_two(input)
}

#[divan::bench]
fn part_2_hashmap() -> u32 {
    let input = include_str!("../../inputs/2024/day1/input.txt");
    aoc_2024::day1::part_two_hash(input)
}
