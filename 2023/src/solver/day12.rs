use std::collections::HashMap;

use super::Solver;

pub struct Day12 {}

fn permutations(
    springs: &str,
    groups: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if springs.len() == 0 {
        return if groups.len() == 0 { 1 } else { 0 };
    }

    if groups.len() == 0 {
        return if springs.contains('#') { 0 } else { 1 };
    }

    if let Some(&x) = cache.get(&(springs.to_string(), groups.to_vec())) {
        return x;
    }

    let mut result = 0;
    let first_spring = springs.chars().next().unwrap();
    let first_group = groups[0];

    if ".?".contains(first_spring) {
        result += permutations(&springs[1..], groups, cache)
    }

    if "#?".contains(first_spring)
        && first_group <= springs.len()
        && !&springs[..first_group].contains('.')
        && (first_group == springs.len() || springs.chars().nth(first_group).unwrap() != '#')
    {
        let next: usize = (first_group < springs.len()).into();
        result += permutations(&springs[first_group + next..], &groups[1..], cache);
    }

    cache.insert((springs.to_string(), groups.to_vec()), result);

    return result;
}

impl Solver for Day12 {
    fn p1(&self, input: &str) -> String {
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        let res: usize = input
            .lines()
            .map(|row| {
                let words: Vec<&str> = row.split(" ").collect();
                let springs = words[0];
                let groups: Vec<usize> = words[1].split(",").map(|x| x.parse().unwrap()).collect();
                permutations(springs, &groups, &mut cache)
            })
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut cache: HashMap<(String, Vec<usize>), usize> = HashMap::new();
        let res: usize = input
            .lines()
            .map(|row| {
                let words: Vec<&str> = row.split(" ").collect();
                let springs = vec![words[0]].repeat(5).join("?").to_owned();
                let groups: Vec<usize> = words[1]
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<_>>()
                    .repeat(5);
                permutations(&springs, &groups, &mut cache)
            })
            .sum();

        return res.to_string();
    }
}
