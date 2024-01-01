use std::collections::HashMap;

use num::integer::lcm;
use regex::Regex;

use super::Solver;

pub struct Day08 {}

impl Solver for Day08 {
    fn p1(&self, input: &str) -> String {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let mut lines = input.lines();
        let mut navigation = lines.next().unwrap().chars().cycle();
        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

        let mut curr = "";
        lines.filter_map(|l| re.captures(l)).for_each(|l| {
            let (_, [node, left, right]) = l.extract();
            map.insert(node, (left, right));

            if node.ends_with('A') && curr.is_empty() {
                curr = node;
            }
        });

        let mut steps = 0;
        while !curr.ends_with('Z') {
            let next_move = navigation.next().unwrap();
            let children = map.get(curr).unwrap();
            match next_move {
                'L' => curr = children.0,
                'R' => curr = children.1,
                _ => panic!("no such instruction: {}", next_move),
            }
            steps += 1;
        }

        return steps.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let mut lines = input.lines();
        let navigation = lines.next().unwrap().chars();
        let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

        lines.filter_map(|l| re.captures(l)).for_each(|l| {
            let (_, [curr, left, right]) = l.extract();
            map.insert(curr, (left, right));
        });

        let currs: Vec<&str> = map
            .keys()
            .filter(|node| node.ends_with('A'))
            .map(|&node| node)
            .collect();

        let res = currs
            .iter()
            .map(|&start| {
                let mut steps: usize = 0;
                let mut curr = start;
                let mut navigation = navigation.clone().cycle();
                while !curr.ends_with('Z') {
                    let next_move = navigation.next().unwrap();
                    let children = map.get(curr).unwrap();
                    curr = match next_move {
                        'L' => children.0,
                        'R' => children.1,
                        _ => panic!("no such instruction: {}", next_move),
                    };
                    steps += 1;
                }
                return steps;
            })
            .reduce(|acc, x| lcm(acc, x))
            .unwrap();

        return res.to_string();
    }
}
