use crate::problem::Problem;
use itertools::Itertools;

pub struct Day04 {}

fn bounds(input: &str) -> [i32; 2] {
    let mut numbers = input.split('-').map(|t| t.parse::<i32>().unwrap());
    [numbers.next().unwrap(), numbers.next().unwrap()]
}

// Full-on iterator implementation (slower)
// fn valid(password: &i32) -> bool {
// let check = |(increasing, has_double), (a, b)| (increasing && a <= b, has_double || a == b);
//
// let (increasing, has_double) = password
// .to_string()
// .chars()
// .tuple_windows::<(_, _)>()
// .fold((true, false), check);
//
// increasing && has_double
// }

fn valid(password: &i32) -> bool {
    let chars = &password.to_string().chars().collect::<Vec<_>>();
    chars
        .iter()
        .group_by(|i| *i)
        .into_iter()
        .map(|(_, group)| group.count())
        .any(|length| length >= 2usize)
        && chars.iter().tuple_windows().all(|(a, b)| a <= b)
}

fn valid_without_groups(password: &i32) -> bool {
    let chars = &password.to_string().chars().collect::<Vec<_>>();
    chars
        .iter()
        .group_by(|i| *i)
        .into_iter()
        .map(|(_, group)| group.count())
        .any(|length| length == 2usize)
        && chars.iter().tuple_windows().all(|(a, b)| a <= b)
}

impl Problem for Day04 {
    fn part_one(&self, input: &str) -> String {
        let bounds = bounds(&input);
        (bounds[0]..=bounds[1]).filter(valid).count().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let bounds = bounds(&input);
        (bounds[0]..=bounds[1])
            .filter(valid_without_groups)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_from_input() {
        assert_eq!(bounds("146810-612564"), [146810, 612564]);
    }

    #[test]
    fn example_data_part_one() {
        assert!(valid(&111111));
        assert!(valid(&223450) == false);
        assert!(valid(&123789) == false);
    }
    #[test]
    fn example_data_part_two() {
        assert!(valid_without_groups(&112233));
        assert!(valid_without_groups(&123444) == false);
        assert!(valid_without_groups(&111122));
    }
}
