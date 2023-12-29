use super::Solver;

pub struct Day05 {}

impl Solver for Day05 {
    fn p1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut seeds: Vec<i64> = lines
            .nth(0)
            .unwrap()
            .split(" ")
            .filter_map(|f| f.parse().ok())
            .collect();
        let mut curr = vec![-1; seeds.len()];

        for l in lines {
            if l.is_empty() {
                continue;
            }

            if l.contains("map") {
                seeds = seeds
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| if curr[i] != -1 { curr[i] } else { s })
                    .collect();
                curr = vec![-1; seeds.len()];
                continue;
            }

            let ranges: Vec<i64> = l.split(" ").filter_map(|f| f.parse().ok()).collect();

            seeds.iter().enumerate().for_each(|(i, &s)| {
                if s >= ranges[1] && s < ranges[1] + ranges[2] {
                    curr[i] = ranges[0] + (s - ranges[1]);
                }
            })
        }

        seeds = seeds
            .iter()
            .enumerate()
            .map(|(i, &s)| if curr[i] != -1 { curr[i] } else { s })
            .collect();

        return seeds.iter().min().unwrap_or(&0).to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let mut seeds: Vec<i64> = lines
            .nth(0)
            .unwrap()
            .split(" ")
            .filter_map(|f| f.parse().ok())
            .collect::<Vec<i64>>()
            .windows(2)
            .step_by(2)
            .flat_map(|w| (w[0]..=w[0] + w[1] - 1).collect::<Vec<_>>())
            .collect();
        let mut curr = vec![-1; seeds.len()];

        for l in lines {
            if l.is_empty() {
                continue;
            }

            if l.contains("map") {
                seeds = seeds
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| if curr[i] != -1 { curr[i] } else { s })
                    .collect();
                curr = vec![-1; seeds.len()];
                continue;
            }

            let ranges: Vec<i64> = l.split(" ").filter_map(|f| f.parse().ok()).collect();

            seeds.iter().enumerate().for_each(|(i, &s)| {
                if s >= ranges[1] && s < ranges[1] + ranges[2] {
                    curr[i] = ranges[0] + (s - ranges[1]);
                }
            })
        }

        seeds = seeds
            .iter()
            .enumerate()
            .map(|(i, &s)| if curr[i] != -1 { curr[i] } else { s })
            .collect();

        return seeds.iter().min().unwrap_or(&0).to_string();
    }
}
