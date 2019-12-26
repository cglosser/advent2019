use crate::solutions::Solution;
use std::collections::HashMap;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> String {
        System::from_str(input).num_orbits().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        String::from("Hello from part_two")
    }
}

struct System {
    orbits: HashMap<String, String>,
}

impl System {
    fn from_str(input: &str) -> Self {
        System {
            orbits: input
                .lines()
                .map(|line| {
                    let mut t = line.split(')');
                    let parent = t.next().unwrap().to_string();
                    let child = t.next().unwrap().to_string();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let s = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        assert_eq!(42, System::from_str(&s).num_orbits());
    }

    #[test]
    fn part_two() {
        let s = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";

        assert!(false);
    }
}
