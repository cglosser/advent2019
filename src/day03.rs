use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;

const INPUT: &str = "inputs/day03_input.txt";

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Step {
    direction: Direction,
    length: i32,
}

impl FromStr for Step {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes(); // Assumed safe; no UTF weirdness
        Ok(Step {
            direction: match bytes[0] as char {
                'R' => Direction::Right,
                'U' => Direction::Up,
                'L' => Direction::Left,
                'D' => Direction::Down,
                _ => Direction::Invalid,
            },

            length: std::str::from_utf8(&bytes[1..])?.parse()?,
        })
    }
}

type Point = (i32, i32);
fn advance(tail: &mut Vec<Point>, step: &Step) {
    let (x, y) = *tail.last().expect("Expected length > 0");
    let steps = (1..=step.length).filter_map(|i| match step.direction {
        Direction::Right => Some((x + i, 0)),
        Direction::Left => Some((x - i, 0)),
        Direction::Up => Some((0, y + i)),
        Direction::Down => Some((0, y - i)),
        Direction::Invalid => None,
    });

    tail.extend(steps);
}

type Steps = Vec<Step>;
fn read_input() -> Vec<Steps> {
    let file = std::fs::File::open(INPUT).expect("Could not open file");
    let br = BufReader::new(file);
    let v = br
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|t| t.parse().unwrap())
                .collect()
        })
        .collect();

    v
}

pub fn part1() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_to_step() {
        assert_eq!(
            "R18".parse::<Step>().unwrap(),
            Step {
                direction: Direction::Right,
                length: 18
            }
        );
    }

    #[test]
    fn example1() {
        let steps = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            .split_ascii_whitespace()
            .map(|line| {
                line.split(',')
                    .map(|t| t.parse::<Step>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
    }
}
