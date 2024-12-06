use crate::timed;

const DIRECTIONS: [(isize, isize); 8] = [
    // top left
    (-1, -1),
    // top
    (0, -1),
    // top right
    (1, -1),
    // left
    (-1, 0),
    // right
    (1, 0),
    // bottom left
    (-1, 1),
    // bottom
    (0, 1),
    // bottom right
    (1, 1),
];

fn part_one(input: &str) -> u32 {
    1
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2024/day4/example.txt");

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
