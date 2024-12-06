fn main() {
    divan::main();
}

#[divan::bench]
fn part_1() -> usize {
    let input = include_str!("../../inputs/2024/day6/input.txt");
    aoc_2024::day6::part_one(input)
}

#[divan::bench]
fn part_2() -> usize {
    let input = include_str!("../../inputs/2024/day6/input.txt");
    aoc_2024::day6::part_two(input)
}
