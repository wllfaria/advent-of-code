use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map_res, recognize};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

use crate::timed;

#[derive(Debug)]
enum Command {
    Do,
    Dont,
    Mul(u32, u32),
}

impl Command {
    pub fn reduce(&self) -> u32 {
        match self {
            Command::Do => 0,
            Command::Dont => 0,
            Command::Mul(lhs, rhs) => lhs * rhs,
        }
    }
}

fn to_digit(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn numeral(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many1(one_of("1234567890"))), to_digit)(input)
}

fn mul(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("mul(")(input)?;
    let (input, (left, _, right)) = tuple((numeral, tag(","), numeral))(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Command::Mul(left, right)))
}

fn as_do(_: &str) -> Result<Command, ()> {
    Ok(Command::Do)
}

fn as_dont(_: &str) -> Result<Command, ()> {
    Ok(Command::Dont)
}

fn inst_do(input: &str) -> IResult<&str, Command> {
    map_res(tag("do()"), as_do)(input)
}

fn inst_dont(input: &str) -> IResult<&str, Command> {
    map_res(tag("don't()"), as_dont)(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((mul, inst_dont, inst_do))(input)
}

fn parse(input: &str) -> Vec<Command> {
    let mut commands = vec![];
    let mut rest = input;

    while !rest.is_empty() {
        match parse_command(rest) {
            Ok((next, command)) => {
                commands.push(command);
                rest = next;
            }
            Err(_) => rest = &rest[1..],
        }
    }

    commands
}

fn part_one(input: &str) -> u32 {
    let commands = parse(input);
    commands.iter().fold(0, |acc, c| acc + c.reduce())
}

fn part_two(input: &str) -> u32 {
    let commands = parse(input);

    let mut ignore = false;
    commands.iter().fold(0, |acc, c| match c {
        Command::Do => {
            ignore = false;
            acc + c.reduce()
        }
        Command::Dont => {
            ignore = true;
            acc + c.reduce()
        }
        Command::Mul(_, _) => {
            if !ignore {
                acc + c.reduce()
            } else {
                acc
            }
        }
    })
}

pub fn run(part: Option<u8>) {
    let input = include_str!("../../inputs/2024/day3/input.txt");

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
