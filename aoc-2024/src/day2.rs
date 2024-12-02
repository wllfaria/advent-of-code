use crate::timed;

pub fn part_one(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    reports.iter().filter(|report| is_report_safe(report)).count() as u32
}

pub fn part_two(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    reports
        .iter()
        .filter(|report| {
            if is_report_safe(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut new_report = report.to_vec();
                new_report.remove(i);
                if is_report_safe(&new_report) {
                    return true;
                }
            }

            false
        })
        .count() as u32
}

fn is_report_safe(report: &[u32]) -> bool {
    let is_increasing = report.windows(2).all(|pair| {
        let (curr, next) = (pair[0], pair[1]);
        next > curr && next.abs_diff(curr) <= 3
    });

    let is_decreasing = report.windows(2).all(|pair| {
        let (curr, next) = (pair[0], pair[1]);
        next < curr && next.abs_diff(curr) <= 3
    });

    is_increasing || is_decreasing
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2024/day2/input.txt");

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
