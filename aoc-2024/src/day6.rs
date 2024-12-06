// local benchmark result because yesh
// day6       fastest       │ slowest       │ median        │ mean          │ samples │ iters
// ├─ part_1  27.79 µs      │ 66.83 µs      │ 28.83 µs      │ 30.21 µs      │ 100     │ 100
// ╰─ part_2  12.88 ms      │ 16.86 ms      │ 14.33 ms      │ 14.37 ms      │ 100     │ 100

use std::collections::{HashSet, VecDeque};
use std::sync::Mutex;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::timed;

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum FacingDirection {
    Left,
    Down,
    #[default]
    Up,
    Right,
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
    let lines = input.lines().collect_vec();
    let width = lines[0].len();
    let height = lines.len();
    let mut guard_position = (0, 0);
    let mut facing_direction = FacingDirection::default();
    let mut maze = vec![0; width * height];

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => maze[y * width + x] = 0,
                '#' => maze[y * width + x] = 1,
                '^' => {
                    guard_position = (x, y);
                    maze[y * width + x] = 0;
                }
                _ => unreachable!(),
            }
        }
    }

    let mut visited = vec![false; width * height];
    let mut path_count = 1;
    visited[guard_position.1 * width + guard_position.0] = true;

    loop {
        if going_out_of_bounds(guard_position, (width, height), facing_direction) {
            break;
        }

        let (col, row) = facing_direction.neighbor(guard_position);
        if maze[row * width + col] == 1 {
            facing_direction = facing_direction.next();
            continue;
        }

        guard_position = (col, row);
        let index = row * width + col;
        if !visited[index] {
            visited[index] = true;
            path_count += 1;
        }
    }

    path_count
}

fn find_reachable_cells(maze: &[u8], width: usize, height: usize, guard_position: (usize, usize)) -> Vec<bool> {
    let mut visited = vec![false; width * height];
    let mut queue = VecDeque::new();

    let start_index = guard_position.1 * width + guard_position.0;
    visited[start_index] = true;
    queue.push_back(guard_position);

    while let Some(position) = queue.pop_front() {
        for &direction in &[
            FacingDirection::Up,
            FacingDirection::Down,
            FacingDirection::Left,
            FacingDirection::Right,
        ] {
            let (next_col, next_row) = direction.neighbor(position);
            if !going_out_of_bounds((next_col, next_row), (width, height), direction)
                && maze[next_row * width + next_col] == 0
            {
                let next_index = next_row * width + next_col;
                if !visited[next_index] {
                    visited[next_index] = true;
                    queue.push_back((next_col, next_row));
                }
            }
        }
    }

    visited
}

pub fn part_two(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect_vec();
    let width = lines[0].len();
    let height = lines.len();
    let mut guard_position = (0, 0);

    let mut maze = vec![0; width * height];

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => maze[y * width + x] = 0,
                '#' => maze[y * width + x] = 1,
                '^' => {
                    guard_position = (x, y);
                    maze[y * width + x] = 0;
                }
                _ => unreachable!(),
            }
        }
    }

    let reachable_positions = find_reachable_cells(&maze, width, height, guard_position);

    let cycles = Mutex::new(HashSet::new());

    (0..width * height).into_par_iter().for_each(|idx| {
        let col = idx / width;
        let row = idx % width;
        let mut maze = maze.clone();

        if maze[row * width + col] == 1 || (col, row) == guard_position || !reachable_positions[row * width + col] {
            return;
        }

        maze[row * width + col] = 1;

        // * 4 is 4 directions per cell!!! (this screwed me)
        let mut path = vec![false; width * height * 4];

        let simulate_guard = |maze: &[u8], path: &mut [bool]| -> bool {
            let mut current_position = guard_position;
            let mut current_direction = FacingDirection::default();

            loop {
                let path_index = current_position.1 * width * 4
                    + current_position.0 * 4
                    + match current_direction {
                        FacingDirection::Left => 0,
                        FacingDirection::Down => 1,
                        FacingDirection::Up => 2,
                        FacingDirection::Right => 3,
                    };

                if path[path_index] {
                    return true;
                }

                path[path_index] = true;

                if going_out_of_bounds(current_position, (width, height), current_direction) {
                    return false;
                }

                let (next_col, next_row) = current_direction.neighbor(current_position);
                let cell = maze[next_row * width + next_col];
                if cell == 1 {
                    current_direction = current_direction.next();
                    continue;
                }

                current_position = (next_col, next_row);
            }
        };

        if simulate_guard(&maze, &mut path) {
            cycles.lock().unwrap().insert((col, row));
        }
    });

    let x = cycles.lock().unwrap().len();
    x + 1 // adding initial guard position
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
