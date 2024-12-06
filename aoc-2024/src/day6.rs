use std::collections::HashSet;

use itertools::Itertools;

use crate::timed;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FacingDirection {
    Left,
    Down,
    #[default]
    Up,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CellState {
    Empty,
    Obstructed,
}

impl FacingDirection {
    pub fn next(&self) -> Self {
        match self {
            FacingDirection::Left => FacingDirection::Up,
            FacingDirection::Down => FacingDirection::Left,
            FacingDirection::Up => FacingDirection::Right,
            FacingDirection::Right => FacingDirection::Down,
        }
    }

    pub fn neighbor(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            FacingDirection::Left => (position.0 - 1, position.1),
            FacingDirection::Down => (position.0, position.1 + 1),
            FacingDirection::Up => (position.0, position.1 - 1),
            FacingDirection::Right => (position.0 + 1, position.1),
        }
    }
}

fn going_out_of_bounds(position: (usize, usize), maze_size: (usize, usize), facing_direction: FacingDirection) -> bool {
    match facing_direction {
        FacingDirection::Left if position.0 == 0 => true,
        FacingDirection::Down if position.1 == maze_size.1 - 1 => true,
        FacingDirection::Up if position.1 == 0 => true,
        FacingDirection::Right if position.0 == maze_size.0 - 1 => true,
        _ => false,
    }
}

pub fn part_one(input: &str) -> usize {
    let maze_size = (input.lines().next().unwrap().len(), input.lines().count());
    let mut guard_position = (0, 0);
    let mut facing_direction = FacingDirection::default();
    let mut path = HashSet::new();

    let maze = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '.' => CellState::Empty,
                    '#' => CellState::Obstructed,
                    '^' => {
                        guard_position = (col, row);
                        CellState::Empty
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    path.insert(guard_position);

    loop {
        if going_out_of_bounds(guard_position, maze_size, facing_direction) {
            break;
        }

        let (col, row) = facing_direction.neighbor(guard_position);
        let cell = maze[row][col];
        if cell == CellState::Obstructed {
            facing_direction = facing_direction.next();
            continue;
        }

        guard_position = (col, row);
        path.insert(guard_position);
    }

    path.len()
}

pub fn part_two(input: &str) -> usize {
    let maze_size = (input.lines().next().unwrap().len(), input.lines().count());
    let mut guard_position = (0, 0);
    let facing_direction = FacingDirection::default();

    let mut maze = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| match ch {
                    '.' => CellState::Empty,
                    '#' => CellState::Obstructed,
                    '^' => {
                        guard_position = (col, row);
                        CellState::Empty
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut cycles = HashSet::new();
    let mut path = HashSet::new();

    let simulate_guard = |maze: &[Vec<CellState>], path: &mut HashSet<((usize, usize), FacingDirection)>| -> bool {
        let mut current_position = guard_position;
        let mut current_direction = facing_direction;

        loop {
            if !path.insert((current_position, current_direction)) {
                return true;
            }

            if going_out_of_bounds(current_position, maze_size, current_direction) {
                return false;
            }

            let (col, row) = current_direction.neighbor(current_position);
            let cell = maze[row][col];
            if cell == CellState::Obstructed {
                current_direction = current_direction.next();
                continue;
            }

            current_position = (col, row);
        }
    };

    for row in 0..maze_size.1 {
        for col in 0..maze_size.0 {
            let cell = maze[row][col];
            if cell == CellState::Obstructed || (col, row) == guard_position {
                continue;
            }

            maze[row][col] = CellState::Obstructed;

            if simulate_guard(&maze, &mut path) {
                cycles.insert((col, row));
            }
            path.clear();
            maze[row][col] = CellState::Empty;
        }
    }

    cycles.len()
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2024/day6/input.txt");

    match part.unwrap_or_default() {
        1 => {
            timed(part_one, input, 1);
        }
        2 => {
            timed(part_two, input, 2);
        }
        _ => {
            timed(part_one, input, 1);
            timed(part_two, input, 2);
        }
    };
}
