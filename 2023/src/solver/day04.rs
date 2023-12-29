use super::Solver;

pub struct Day04 {}

impl Solver for Day04 {
    fn p1(&self, input: &str) -> String {
        let res: i32 = input
            .lines()
            .map(|line| {
                let numbers = line.split("|").collect::<Vec<_>>();
                let winning_numbers: Vec<i32> = numbers[0].split(":").collect::<Vec<_>>()[1]
                    .split(" ")
                    .filter_map(|e| e.parse().ok())
                    .collect();
                let matches = numbers[1]
                    .split(" ")
                    .filter_map(|e| e.parse().ok())
                    .filter(|n| winning_numbers.contains(n))
                    .count();

                if matches == 0 {
                    return 0;
                }

                return 2_i32.pow(matches as u32 - 1);
            })
            .sum();
        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut copies: Vec<_> = vec![1; lines.len()];
        let res: usize = lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let numbers = line.split("|").collect::<Vec<_>>();
                let winning_numbers: Vec<i32> = numbers[0].split(":").collect::<Vec<_>>()[1]
                    .split(" ")
                    .filter_map(|e| e.parse().ok())
                    .collect();
                let matches = numbers[1]
                    .split(" ")
                    .filter_map(|e| e.parse().ok())
                    .filter(|n| winning_numbers.contains(n))
                    .count();
                for j in i + 1..i + matches + 1 {
                    copies[j] += copies[i];
                }
                copies[i]
            })
            .sum();
        return res.to_string();
    }
}
