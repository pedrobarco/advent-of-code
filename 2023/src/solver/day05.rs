use std::cmp;

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
        let mut seeds: Vec<(i64, i64)> = lines
            .nth(0)
            .unwrap()
            .split(" ")
            .filter_map(|f| f.parse().ok())
            .collect::<Vec<_>>()
            .chunks_exact(2)
            .map(|w| (w[0], w[0] + w[1]))
            .collect();

        for l in input.split("\n\n").skip(1) {
            let ranges: Vec<Vec<i64>> = l
                .lines()
                .skip(1)
                .map(|r| r.split(" ").filter_map(|f| f.parse().ok()).collect())
                .collect();
            let mut new_seeds: Vec<(i64, i64)> = Vec::new();
            while seeds.len() > 0 {
                let seed = seeds.pop().unwrap();
                let found = ranges
                    .iter()
                    .find(|&range| {
                        let source_range = (range[1], range[1] + range[2]);
                        let target = range[0];
                        let intersection = (
                            cmp::max::<i64>(seed.0, source_range.0),
                            cmp::min::<i64>(seed.1, source_range.1),
                        );
                        if intersection.0 >= intersection.1 {
                            return false;
                        }
                        new_seeds.push((
                            intersection.0 - source_range.0 + target,
                            intersection.1 - source_range.0 + target,
                        ));
                        if intersection.0 > seed.0 {
                            seeds.push((seed.0, intersection.0));
                        }
                        if seed.1 > intersection.1 {
                            seeds.push((intersection.1, seed.1));
                        }
                        return true;
                    })
                    .is_some();
                if !found {
                    new_seeds.push(seed);
                }
            }
            seeds = new_seeds.clone();
            new_seeds.clear();
        }

        return seeds.iter().map(|f| f.0).min().unwrap_or(0).to_string();
    }
}
