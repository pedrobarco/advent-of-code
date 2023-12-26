use regex::Regex;
use std::collections::HashMap;

use super::Solver;

pub struct Day02 {}

impl Solver for Day02 {
    fn p1(&self, input: &str) -> String {
        let re = Regex::new(r"([0-9]+) (\w+) cubes").unwrap();
        let mut cubes = HashMap::new();

        let first_line = input.lines().nth(0).unwrap();
        first_line.split(",").for_each(|f| {
            for (_, [number, color]) in re.captures_iter(f).map(|c| c.extract()) {
                cubes.insert(color.to_string(), number.parse().unwrap());
            }
        });

        let res: usize = input
            .lines()
            .skip(1)
            .enumerate()
            .filter(|(_, l)| {
                let re = Regex::new(r"([0-9]+) (\w+)").unwrap();
                l.split(":")
                    .nth(1)
                    .unwrap()
                    .split(";")
                    .find(|set| {
                        set.split(",")
                            .find(|&cube| {
                                let (_, [number, color]) = re.captures(cube).unwrap().extract();
                                cubes.get(color).unwrap_or(&0) < &number.parse().unwrap()
                            })
                            .is_some()
                    })
                    .is_none()
            })
            .map(|(i, _)| i + 1)
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let res: usize = input
            .lines()
            .skip(1)
            .map(|l| {
                let re = Regex::new(r"([0-9]+) (\w+)").unwrap();
                let mut cubes: HashMap<String, i32> = HashMap::new();
                l.split(":").nth(1).unwrap().split(";").for_each(|set| {
                    set.split(",").for_each(|cube| {
                        let (_, [number, color]) = re.captures(cube).unwrap().extract();
                        if cubes.get(color).is_none()
                            || cubes.get(color).unwrap() < &number.parse().unwrap()
                        {
                            cubes.insert(color.to_string(), number.parse().unwrap());
                        }
                    });
                });

                let mut res = 1;
                for (_, v) in cubes {
                    res *= v;
                }
                return res as usize;
            })
            .sum();

        return res.to_string();
    }
}
