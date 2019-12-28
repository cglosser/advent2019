use crate::solutions::Solution;
use crate::utilities::intcode;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> String {
        System::from_str(input).maximum_signal().to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("NOT IMPLEMENTED - 07b")
    }
}

struct System {
    program: Vec<i32>,
}

impl System {
    fn from_str(input: &str) -> Self {
        System {
            program: input
                .trim()
                .split(',')
                .map(|t| t.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        }
    }

    fn run(&self, phases: &[i32; 5]) -> i32 {
        let mut signal = 0;

        for phase in phases {
            let input = Some(vec![signal, *phase]);
            let mut vm = intcode::VirtualMachine::new(self.program.clone(), input);
            vm.run().unwrap();
            signal = *vm.output().first().unwrap();
        }

        signal
    }

    fn maximum_signal(&self) -> i32 {
        let mut max_signal = std::i32::MIN;

        for (i, j, k, l, m) in iproduct!(0..=4, 0..=4, 0..=4, 0..=4, 0..=4) {
            if i == j
                || i == k
                || i == l
                || i == m
                || j == k
                || j == l
                || j == m
                || k == l
                || k == m
                || l == m
            {
                continue;
            }
            max_signal = std::cmp::max(max_signal, self.run(&[i, j, k, l, m]));
        }

        max_signal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let sys = System::from_str(&"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(43210, sys.run(&[4, 3, 2, 1, 0]));
    }

    #[test]
    fn example_02() {
        let sys = System::from_str(
            &"3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
        );
        assert_eq!(54321, sys.run(&[0, 1, 2, 3, 4]));
    }

    #[test]
    fn example_03() {
        let sys = System::from_str(
            &"3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
        );
        assert_eq!(65210, sys.run(&[1, 0, 4, 3, 2]));
    }
}
