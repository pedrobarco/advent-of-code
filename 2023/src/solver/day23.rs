use std::collections::{HashMap, HashSet};

use super::Solver;

pub struct Day23 {}

impl Solver for Day23 {
    fn p1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start = (0, grid[0].iter().position(|&x| x == '.').unwrap());
        let target = (
            grid.len() - 1,
            grid[grid.len() - 1].iter().position(|&x| x == '.').unwrap(),
        );

        fn neighbours(grid: &Vec<Vec<char>>, curr: (usize, usize)) -> Vec<(usize, usize)> {
            return match grid[curr.0][curr.1] {
                'v' => vec![(1, 0)],
                '^' => vec![(-1, 0)],
                '>' => vec![(0, 1)],
                '<' => vec![(0, -1)],
                '.' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
                _ => panic!("unknown tile"),
            }
            .iter()
            .map(|m| (curr.0 as i32 + m.0, curr.1 as i32 + m.1))
            .filter(|p| {
                p.0 >= 0 && p.0 < grid.len() as i32 && p.1 >= 0 && p.1 < grid[0].len() as i32
            })
            .map(|p| (p.0 as usize, p.1 as usize))
            .filter(|p| grid[p.0][p.1] != '#')
            .collect();
        }

        fn dfs(
            grid: &Vec<Vec<char>>,
            curr: (usize, usize),
            target: (usize, usize),
            cost: usize,
            neighbours: fn(grid: &Vec<Vec<char>>, curr: (usize, usize)) -> Vec<(usize, usize)>,
            visited: &mut HashSet<(usize, usize)>,
            ends: &mut Vec<usize>,
        ) {
            if curr == target {
                ends.push(cost);
                return;
            }

            visited.insert(curr);

            for next in neighbours(grid, curr) {
                if visited.contains(&next) {
                    continue;
                }

                dfs(grid, next, target, cost + 1, neighbours, visited, ends)
            }

            visited.remove(&curr);
        }

        let mut ends = Vec::new();
        let mut visited = HashSet::new();

        dfs(&grid, start, target, 0, neighbours, &mut visited, &mut ends);

        let res = ends.iter().max().unwrap();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start = (0, grid[0].iter().position(|&x| x == '.').unwrap());
        let target = (
            grid.len() - 1,
            grid[grid.len() - 1].iter().position(|&x| x == '.').unwrap(),
        );

        fn neighbours(grid: &Vec<Vec<char>>, curr: (usize, usize)) -> Vec<(usize, usize)> {
            return [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .map(|m| (curr.0 as i32 + m.0, curr.1 as i32 + m.1))
                .filter(|p| {
                    p.0 >= 0 && p.0 < grid.len() as i32 && p.1 >= 0 && p.1 < grid[0].len() as i32
                })
                .map(|p| (p.0 as usize, p.1 as usize))
                .filter(|p| grid[p.0][p.1] != '#')
                .collect();
        }

        fn dfs(
            graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
            curr: (usize, usize),
            target: (usize, usize),
            visited: &mut HashSet<(usize, usize)>,
        ) -> isize {
            if curr == target {
                return 0;
            }

            let mut steps = isize::MIN;

            visited.insert(curr);

            for (k, v) in graph.get(&curr).unwrap() {
                if visited.contains(&k) {
                    continue;
                }

                let next_steps = dfs(&graph, *k, target, visited);
                steps = steps.max(next_steps + *v as isize);
            }

            visited.remove(&curr);

            return steps;
        }

        let mut points = vec![start, target];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let curr = (i, j);
                if grid[i][j] != '#' && neighbours(&grid, curr).len() > 2 {
                    points.push(curr);
                }
            }
        }

        let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

        for &p in points.iter() {
            let mut stack: Vec<((usize, usize), usize)> = vec![(p, 0)];
            let mut visited: HashSet<(usize, usize)> = HashSet::from([p]);

            while let Some((curr, steps)) = stack.pop() {
                if steps != 0 && points.contains(&curr) {
                    graph.entry(p).or_default().insert(curr, steps);
                    continue;
                }

                for next in neighbours(&grid, curr) {
                    if visited.get(&next).is_none() {
                        stack.push((next, steps + 1));
                        visited.insert(next);
                    }
                }
            }
        }

        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        let res = dfs(&graph, start, target, &mut visited);

        return res.to_string();
    }
}
