use std::fs::File;
use std::io::Read;

const INPUT: &str = "inputs/day02_input.txt";

fn run_tape(input: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut tape = input.clone();
    tape[1] = noun;
    tape[2] = verb;

    let mut i = 0;
    loop {
        let op = &tape[i];
        match op {
            1 => {
                let left = tape[i + 1];
                let right = tape[i + 2];
                let dest = tape[i + 3];
                tape[dest] = tape[left] + tape[right];
            }
            2 => {
                let left = tape[i + 1];
                let right = tape[i + 2];
                let dest = tape[i + 3];
                tape[dest] = tape[left] * tape[right];
            }
            99 => break,
            _ => panic!("Invalid operation"),
        }

        i += 4;
    }

    tape[0]
}

fn read_input() -> Vec<usize> {
    let mut s = String::new();
    File::open(INPUT).unwrap().read_to_string(&mut s).unwrap();

    s.split(",")
        .map(|token| token.parse::<usize>().unwrap())
        .collect()
}

pub fn part1() {
    let v = read_input();
    println!("Day 02a: {}", run_tape(&v, 12, 2));
}

pub fn part2() {
    let v = read_input();
    for noun in 0..100 {
        for verb in 0..100 {
            let output = run_tape(&v, noun, verb);
            if output == 19690720usize {
                println!("Day 02b: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn program_examples() {
        assert_eq!(run_tape(&vec![1, 0, 0, 0, 99], 0, 0), 2);
        assert_eq!(run_tape(&vec![2, 3, 0, 3, 99], 3, 0), 2);
        assert_eq!(run_tape(&vec![2, 4, 4, 5, 99, 0], 4, 4), 2);
        assert_eq!(run_tape(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 1, 1), 30);
    }
}
