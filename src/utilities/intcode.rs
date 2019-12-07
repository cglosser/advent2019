pub enum Operation {
    ADD,
    MULTIPLY,
    HALT,
}

impl Operation {
    pub fn from_i32(value: i32) -> Option<Operation> {
        match value {
            1 => Some(Operation::ADD),
            2 => Some(Operation::MULTIPLY),
            99 => Some(Operation::HALT),
            _ => None,
        }
    }
}

pub fn run(mut tape: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    loop {
        match Operation::from_i32(tape[i]) {
            Some(Operation::ADD) => {
                let left = tape[i + 1] as usize;
                let right = tape[i + 2] as usize;
                let dest = tape[i + 3] as usize;
                tape[dest] = tape[left] + tape[right];
            }
            Some(Operation::MULTIPLY) => {
                let left = tape[i + 1] as usize;
                let right = tape[i + 2] as usize;
                let dest = tape[i + 3] as usize;
                tape[dest] = tape[left] * tape[right];
            }
            Some(Operation::HALT) => break,
            None => panic!("Invalid operation"),
        }
        i += 4;
    }

    tape
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02_examples() {
        assert_eq!(run(vec![1, 0, 0, 0, 99])[0], 2);
        assert_eq!(run(vec![2, 3, 0, 3, 99])[0], 2);
        assert_eq!(run(vec![2, 4, 4, 5, 99, 0])[0], 2);
        assert_eq!(run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])[0], 30);
    }
}
