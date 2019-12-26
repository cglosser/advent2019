use crate::solutions::Solution;
use itertools::*;
use regex::*;
use std::num;

pub struct Day12 {}

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> String {
        String::from("Hello world")
    }

    fn part_two(&self, input: &str) -> String {
        String::from("Hello world")
    }
}

fn to_vec(input: &str) -> Option<[i32; 3]> {
    lazy_static! {
        static ref re: Regex = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    }
    let caps = re.captures(input)?;

    Some([1, 2, 3])
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Moon {
    pub position: Vector3<i32>,
    pub velocity: Vector3<i32>,
}

#[derive(Clone, Debug)]
struct System {
    moons: Vec<Moon>,
}

impl System {
    fn apply_gravity(&mut self) {
        for i in 0..(self.moons.len() - 1) {
            for j in (i + 1)..self.moons.len() {
                let dr = self.moons[j].position - self.moons[i].position;
                self.moons[i].velocity[0] += dr[0].signum();
                self.moons[i].velocity[1] += dr[1].signum();
                self.moons[i].velocity[2] += dr[2].signum();

                self.moons[j].velocity[0] -= dr[0].signum();
                self.moons[j].velocity[1] -= dr[1].signum();
                self.moons[j].velocity[2] -= dr[2].signum();
            }
        }
    }

    fn apply_velocity(&mut self) {
        for moon in &mut self.moons {
            moon.position += moon.velocity;
        }
    }

    pub fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }
}

fn input_to_vector(input: &str) -> Vector3<i32> {}

#[cfg(test)]
mod tests {
    use super::*;
}
