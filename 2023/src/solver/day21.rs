use std::collections::HashSet;

use super::Solver;

pub struct Day21 {}

const MOVES: [(i64, i64); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

impl Solver for Day21 {
    fn p1(&self, input: &str) -> String {
        const TARGET: usize = 64;

        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start = grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, _)| (grid[i][j] == 'S').then(|| (i, j)))
            })
            .unwrap();

        let mut visited: HashSet<(usize, usize)> = HashSet::from([start]);
        for _ in 0..TARGET {
            visited = visited
                .into_iter()
                .flat_map(|(i, j)| {
                    MOVES
                        .iter()
                        .map(|&(dy, dx)| ((i as i64) + dy, (j as i64) + dx))
                        .filter(|&(di, dj)| {
                            return di >= 0
                                && di < grid.len() as i64
                                && dj >= 0
                                && dj < grid[0].len() as i64;
                        })
                        .map(|(di, dj)| (di as usize, dj as usize))
                        .filter(|(di, dj)| grid[*di][*dj] != '#')
                        .collect::<Vec<_>>()
                })
                .collect();
        }

        let res = visited.len();
        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        const TARGET: i64 = 26501365;

        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start = grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(j, _)| (grid[i][j] == 'S').then(|| (i, j)))
            })
            .unwrap();

        let height = grid.len() as i64;
        let width = grid[0].len() as i64;

        let mut visited: HashSet<(i64, i64)> = HashSet::from([(start.0 as i64, start.1 as i64)]);

        let mut derivatives = Vec::new();
        let mut turn = 0;
        let mut start = 0;
        let mut prev_start = 0;

        let target_remainder = TARGET % height as i64;

        while derivatives.len() < 3 {
            turn += 1;

            visited = visited
                .into_iter()
                .flat_map(|(i, j)| {
                    MOVES
                        .iter()
                        .map(|&(dy, dx)| (i + dy, j + dx))
                        .filter(|&(di, dj)| {
                            let i = di.rem_euclid(height);
                            let j = dj.rem_euclid(width);
                            grid[i as usize][j as usize] != '#'
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            if turn >= target_remainder && (turn - target_remainder) % height == 0 {
                let curr = visited.len() as i64;
                let delta = curr - start;
                let values = [curr, delta, delta - prev_start];

                derivatives.push(values[derivatives.len()]);

                start = curr;
                prev_start = delta;
            }
        }

        let a = derivatives[2] / 2;
        let b = derivatives[1] - 3 * a;
        let c = derivatives[0] - a - b;
        let x = (1 + TARGET / height) as i64;

        let res = a * x * x + b * x + c;
        return res.to_string();
    }
}
