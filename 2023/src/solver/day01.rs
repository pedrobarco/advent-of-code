use super::Solver;

pub struct Day01 {}

impl Solver for Day01 {
    fn p1(&self, input: &str) -> String {
        fn filter(c: char) -> Option<u32> {
            c.to_digit(10)
        }

        let res: u32 = input
            .lines()
            .map(|l| {
                let first = l.chars().find_map(|c| filter(c));
                let last = l.chars().rev().find_map(|c| filter(c));
                first.unwrap_or(0) * 10 + last.unwrap_or(0)
            })
            .sum();
        res.to_string()
    }

    fn p2(&self, input: &str) -> String {
        let numbers = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        fn filter(numbers: &Vec<&str>, first: bool, l: &str, i: usize, c: char) -> Option<u32> {
            let num = c.to_digit(10);
            if num.is_some() {
                return num;
            }

            return match numbers.iter().position(|&n| {
                if first {
                    l[..i + 1].contains(n)
                } else {
                    l[l.len() - 1 - i..].contains(n)
                }
            }) {
                Some(j) => Some((j + 1) as u32),
                None => None,
            };
        }

        let res: u32 = input
            .lines()
            .map(|l| {
                let first = l
                    .chars()
                    .enumerate()
                    .find_map(|(i, c)| filter(&numbers, true, &l, i, c));
                let last = l
                    .chars()
                    .rev()
                    .enumerate()
                    .find_map(|(i, c)| filter(&numbers, false, &l, i, c));
                first.unwrap_or(0) * 10 + last.unwrap_or(0)
            })
            .sum();
        res.to_string()
    }
}
