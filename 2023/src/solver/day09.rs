use super::Solver;

pub struct Day09 {}

impl Solver for Day09 {
    fn p1(&self, input: &str) -> String {
        let res: i64 = input
            .lines()
            .map(|l| {
                let mut nums: Vec<i64> = l.split(" ").filter_map(|n| n.parse().ok()).collect();
                let mut last_nums: Vec<i64> = Vec::new();

                while nums.iter().find(|&&n| n != 0).is_some() {
                    last_nums.push(*nums.last().unwrap());
                    nums = nums.windows(2).map(|n| n[1] - n[0]).collect();
                }

                return last_nums.iter().sum::<i64>();
            })
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let res: i64 = input
            .lines()
            .map(|l| {
                let mut nums: Vec<i64> = l.split(" ").filter_map(|n| n.parse().ok()).collect();
                let mut first_nums: Vec<i64> = Vec::new();

                while nums.iter().find(|&&n| n != 0).is_some() {
                    first_nums.push(*nums.first().unwrap());
                    nums = nums.windows(2).map(|n| n[1] - n[0]).collect();
                }

                return first_nums
                    .into_iter()
                    .rev()
                    .reduce(|acc, n| n - acc)
                    .unwrap_or(0);
            })
            .sum();

        return res.to_string();
    }
}
