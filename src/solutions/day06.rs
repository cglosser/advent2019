use crate::solutions::Solution;
use std::collections::HashMap;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        System::from_str(input).num_orbits().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let dist = System::from_str(input).distance(&"YOU", &"SAN") - 2;
        dist.to_string()
    }
}

struct System<'a> {
    orbits: HashMap<&'a str, &'a str>,
}

impl<'a> System<'a> {
    fn from_str(input: &'a str) -> Self {
        System {
            orbits: input
                .lines()
                .map(|line| {
                    let mut t = line.split(')');
                    let parent = t.next().unwrap();
                    let child = t.next().unwrap();
                    (child, parent)
                })
                .collect::<HashMap<_, _>>(),
        }
    }

    fn num_orbits(&self) -> i32 {
        let mut count = 0;
        for parent in self.orbits.values() {
            let mut parent = Some(parent);
            while let Some(p) = parent {
                count += 1;
                parent = self.orbits.get(p);
            }
        }

        count
    }

    fn parents(&self, orbit: &'a str) -> Vec<&'a str> {
        let mut parents: Vec<&'a str> = Vec::new();

        let mut orbit = Some(orbit);
        while let Some(o) = orbit {
            parents.push(o);
            orbit = self.orbits.get(o).map(|v| *v);
        }

        parents
    }

    fn distance(&self, a: &str, b: &str) -> usize {
        let a_parents = self.parents(a);
        let b_parents = self.parents(b);

        let mut a_dist = 0;
        let mut b_dist = 0;

        for (i, a_parent) in a_parents.iter().enumerate() {
            if let Some(j) = b_parents.iter().position(|b_parent| b_parent == a_parent) {
                a_dist = i;
                b_dist = j;
                break;
            }
        }

        a_dist + b_dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";

    #[test]
    fn part_one_example() {
        assert_eq!(42, System::from_str(EXAMPLE).num_orbits());
    }

    #[test]
    fn part_two() {
        assert_eq!(4, System::from_str(EXAMPLE).distance(&"K", &"I"));
    }
}
