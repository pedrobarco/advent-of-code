use std::{collections::HashMap, usize};

use num::Integer;

use super::Solver;

pub struct Day10 {}

const MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const PIPES: [char; 6] = ['|', '-', 'L', 'J', 'F', '7'];

fn bfs(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> HashMap<(usize, usize), bool> {
    fn next_pipes(m: &(i32, i32)) -> Vec<char> {
        match m {
            (1, 0) => vec!['|', 'L', 'J'],
            (-1, 0) => vec!['|', '7', 'F'],
            (0, 1) => vec!['-', '7', 'J'],
            (0, -1) => vec!['-', 'L', 'F'],
            _ => panic!("no such move"),
        }
    }

    fn next_moves(c: &char) -> Vec<(i32, i32)> {
        match c {
            '|' => vec![(1, 0), (-1, 0)],
            '-' => vec![(0, 1), (0, -1)],
            'L' => vec![(-1, 0), (0, 1)],
            'J' => vec![(-1, 0), (0, -1)],
            '7' => vec![(1, 0), (0, -1)],
            'F' => vec![(1, 0), (0, 1)],
            '.' => vec![],
            'S' => MOVES.to_vec(),
            _ => panic!("invalid character"),
        }
    }

    let mut visited: HashMap<(usize, usize), bool> = HashMap::new();
    let mut to_visit = vec![start];
    while to_visit.len() > 0 {
        let curr = to_visit.pop().unwrap();
        if *visited.get(&curr).unwrap_or(&false) {
            continue;
        }
        visited.insert(curr, true);

        let curr_char = grid[curr.0][curr.1];
        let mut s_moves: Vec<(i32, i32)> = Vec::new();

        next_moves(&curr_char)
            .iter()
            .filter(|&m| {
                let next = (curr.0 as i32 + m.0, curr.1 as i32 + m.1);
                next.0 >= 0
                    && next.0 < grid.len() as i32
                    && next.1 >= 0
                    && next.1 < grid[0].len() as i32
            })
            .map(|&m| {
                let next = (
                    (curr.0 as i32 + m.0) as usize,
                    (curr.1 as i32 + m.1) as usize,
                );
                (m, next)
            })
            .filter(|(m, next)| next_pipes(m).contains(&grid[next.0][next.1]))
            .for_each(|(m, next)| {
                if curr_char == 'S' {
                    s_moves.push(m);
                }
                to_visit.push(next);
            });

        if s_moves.len() > 0 {
            grid[curr.0][curr.1] = s_moves.iter().fold(PIPES.clone().to_vec(), |acc, &m| {
                let next = next_pipes(&(m.0 * -1, m.1 * -1));
                acc.into_iter().filter(|&c| next.contains(&c)).collect()
            })[0];
        }
    }

    return visited;
}

impl Solver for Day10 {
    fn p1(&self, input: &str) -> String {
        let mut start = (0, 0);
        let mut grid: Vec<Vec<char>> = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                if let Some(j) = l.find('S') {
                    start = (i, j);
                }
                l.chars().collect::<Vec<char>>()
            })
            .collect();

        let res = bfs(&mut grid, start).len() / 2;
        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut start = (0, 0);
        let mut grid: Vec<Vec<char>> = input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                if let Some(j) = l.find('S') {
                    start = (i, j);
                }
                l.chars().collect::<Vec<char>>()
            })
            .collect();

        fn count_hits(
            grid: &Vec<Vec<char>>,
            visited: &HashMap<(usize, usize), bool>,
            point: &(usize, usize),
        ) -> usize {
            (0..point.1)
                .into_iter()
                .filter(|&k| {
                    visited.get(&(point.0, k)).is_some() && "JL|".contains(grid[point.0][k])
                })
                .count()
        }

        let visited = bfs(&mut grid, start);
        let res: usize = grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let hits = row
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| visited.get(&(i, j)).is_none())
                    .map(|(j, _)| count_hits(&grid, &visited, &(i, j)))
                    .filter(|x| x.is_odd())
                    .count();
                return hits;
            })
            .sum();
        return res.to_string();
    }
}
