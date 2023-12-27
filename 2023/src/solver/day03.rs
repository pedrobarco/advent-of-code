use std::collections::{HashMap, HashSet};

use super::Solver;

pub struct Day03 {}

const MOVES: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Solver for Day03 {
    fn p1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let res: u32 = grid
            .iter()
            .enumerate()
            .map::<u32, _>(|(i, row)| {
                let mut num = 0;
                let mut has_symbol = false;
                row.iter()
                    .enumerate()
                    .map(|(j, c)| {
                        let digit = c.to_digit(10);

                        if digit.is_none() {
                            return 0;
                        }

                        num = (num * 10) + digit.unwrap();

                        let found_symbol = MOVES
                            .iter()
                            .find(|(dx, dy)| {
                                let x = j as i32 + *dx;
                                let y = i as i32 + *dy;
                                return 0 <= x
                                    && x <= grid.len() as i32 - 1
                                    && 0 <= y
                                    && y <= row.len() as i32 - 1
                                    && grid[y as usize][x as usize].is_ascii_punctuation()
                                    && grid[y as usize][x as usize] != '.';
                            })
                            .is_some();

                        if found_symbol && !has_symbol {
                            has_symbol = found_symbol
                        }

                        if j + 1 > row.len() - 1 || row[j + 1].to_digit(10).is_none() {
                            let curr = num;
                            let found_symbol = has_symbol;
                            num = 0;
                            has_symbol = false;
                            if found_symbol {
                                return curr;
                            }
                        }

                        return 0;
                    })
                    .sum()
            })
            .sum();
        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();
        for (i, row) in grid.iter().enumerate() {
            let mut num = 0;
            let mut symbols: HashSet<(usize, usize)> = HashSet::new();

            for (j, c) in row.iter().enumerate() {
                let digit = c.to_digit(10);

                if digit.is_none() {
                    continue;
                }

                num = (num * 10) + digit.unwrap() as i32;

                MOVES.iter().for_each(|(dx, dy)| {
                    let x = j as i32 + *dx;
                    let y = i as i32 + *dy;
                    if 0 <= x
                        && x <= grid.len() as i32 - 1
                        && 0 <= y
                        && y <= row.len() as i32 - 1
                        && grid[y as usize][x as usize] == '*'
                    {
                        let pos = (x as usize, y as usize);
                        symbols.insert(pos);
                    }
                });

                if j + 1 > row.len() - 1 || row[j + 1].to_digit(10).is_none() {
                    symbols.iter().for_each(|pos| {
                        let values = match gears.get_mut(&pos) {
                            Some(v) => v,
                            None => {
                                gears.insert(*pos, Vec::new());
                                gears.get_mut(&pos).unwrap()
                            }
                        };
                        values.push(num);
                    });
                    num = 0;
                    symbols.clear();
                }
            }
        }

        let res: i32 = gears
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v[0] * v[1])
            .sum();

        return res.to_string();
    }
}
