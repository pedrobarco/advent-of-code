use std::{cmp::Ordering, collections::HashMap};

use super::Solver;

pub struct Day07 {}

const CARDS_P1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDS_P2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn create_strengths(cards: &'static [char; 13]) -> HashMap<&'static char, usize> {
    let mut strengths = HashMap::new();
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .for_each(|(k, v)| {
            strengths.insert(k, v);
        });
    return strengths;
}

fn get_score(sum: usize, len: usize) -> usize {
    match len {
        0 => 0,
        1 => match sum {
            2 => 1,
            3 => 3,
            _ => sum + 1,
        },
        2 => match sum {
            5 => 4,
            4 => 2,
            _ => 0,
        },
        _ => 0,
    }
}

impl Solver for Day07 {
    fn p1(&self, input: &str) -> String {
        let strengths = create_strengths(&CARDS_P1);
        let mut hands = input
            .lines()
            .map(|l| {
                let words: Vec<_> = l.split(" ").collect();
                let mut pairs = HashMap::new();
                let cards = words[0];
                let bid: usize = words[1].parse().unwrap();

                for c in cards.chars() {
                    if pairs.get(&c).is_none() {
                        pairs.insert(c, 0);
                    }
                    *pairs.get_mut(&c).unwrap() += 1
                }

                let scores: Vec<usize> = pairs
                    .iter()
                    .filter(|(_, &v)| v > 1)
                    .map(|(_, &v)| v)
                    .collect();

                let score = get_score(scores.iter().sum(), scores.len());

                (score, cards, bid)
            })
            .collect::<Vec<(usize, &str, usize)>>();

        hands.sort_by(|a, b| {
            if a.0 != b.0 {
                return a.0.cmp(&b.0);
            }

            if let Some((sa, sb)) =
                a.1.chars()
                    .zip(b.1.chars())
                    .map(|(ca, cb)| (strengths.get(&ca).unwrap(), strengths.get(&cb).unwrap()))
                    .find(|(&sa, &sb)| sa.cmp(&sb).is_ne())
            {
                return sa.cmp(&sb);
            }

            return Ordering::Equal;
        });

        let res: usize = hands
            .iter()
            .enumerate()
            .map(|(i, (_, _, bid))| (i + 1) * bid)
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let strengths = create_strengths(&CARDS_P2);
        let mut hands = input
            .lines()
            .map(|l| {
                let words: Vec<_> = l.split(" ").collect();
                let mut pairs = HashMap::new();
                let cards = words[0];
                let bid: usize = words[1].parse().unwrap();

                let mut max_pair = ('J', 0);
                for c in cards.chars() {
                    if pairs.get(&c).is_none() {
                        pairs.insert(c, 0);
                    }

                    *pairs.get_mut(&c).unwrap() += 1;

                    let curr_pairs = pairs.get(&c).unwrap();
                    if c != 'J'
                        && (curr_pairs > &max_pair.1
                            || (curr_pairs == &max_pair.1
                                && strengths.get(&c).unwrap()
                                    > strengths.get(&max_pair.0).unwrap()))
                    {
                        max_pair = (c, *curr_pairs)
                    }
                }

                if cards.contains('J') && max_pair.0 != 'J' {
                    *pairs.get_mut(&max_pair.0).unwrap() += pairs.get(&'J').unwrap().to_owned();
                    *pairs.get_mut(&'J').unwrap() = 0;
                }

                let scores: Vec<usize> = pairs
                    .iter()
                    .filter(|(_, &v)| v > 1)
                    .map(|(_, &v)| v)
                    .collect();

                let score = get_score(scores.iter().sum(), scores.len());

                (score, cards, bid)
            })
            .collect::<Vec<(usize, &str, usize)>>();

        hands.sort_by(|a, b| {
            if a.0 != b.0 {
                return a.0.cmp(&b.0);
            }

            if let Some((sa, sb)) =
                a.1.chars()
                    .zip(b.1.chars())
                    .map(|(ca, cb)| (strengths.get(&ca).unwrap(), strengths.get(&cb).unwrap()))
                    .find(|(&sa, &sb)| sa.cmp(&sb).is_ne())
            {
                return sa.cmp(&sb);
            }

            return Ordering::Equal;
        });

        let res: usize = hands
            .iter()
            .enumerate()
            .map(|(i, (_, _, bid))| (i + 1) * bid)
            .sum();

        return res.to_string();
    }
}
