use std::collections::HashMap;

use super::Solver;

pub struct Day15 {}

impl Solver for Day15 {
    fn p1(&self, input: &str) -> String {
        let res: usize = input
            .split(",")
            .map(|word| {
                let mut hash = 0;
                word.chars().map(|c| c as usize).for_each(|b| {
                    hash += b;
                    hash *= 17;
                    hash %= 256;
                });
                return hash;
            })
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut boxes: Vec<HashMap<&str, (usize, usize)>> = vec![];

        for _ in 0..256 {
            boxes.push(HashMap::new());
        }

        let hashes = input.split(",").map(|word| {
            let mut hash = 0;

            let mut lens: Option<usize> = None;

            let label = if word.contains("-") {
                word.split("-").next().unwrap()
            } else {
                lens = word.split("=").last().unwrap().parse::<usize>().ok();
                word.split("=").next().unwrap()
            };

            label.chars().map(|c| c as usize).for_each(|b| {
                hash += b;
                hash *= 17;
                hash %= 256;
            });

            return (label, hash, lens);
        });

        for (i, (label, hash, lens)) in hashes.enumerate() {
            let container = &mut boxes[hash];
            if lens.is_none() {
                if container.get(label).is_some() {
                    container.remove(label);
                }
                continue;
            }

            if let Some(old) = container.get(label) {
                container.insert(label, (old.0, lens.unwrap()));
                continue;
            }

            container.insert(label, (i, lens.unwrap()));
        }

        let res: usize = boxes
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.is_empty())
            .map(|(i, b)| {
                let mut values: Vec<&(usize, usize)> = b.values().collect();
                values.sort();
                return values
                    .iter()
                    .enumerate()
                    .map(|(j, (_, lens))| (i + 1) * (j + 1) * lens)
                    .sum::<usize>();
            })
            .sum();

        return res.to_string();
    }
}
