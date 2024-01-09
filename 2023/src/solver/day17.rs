use std::collections::{BinaryHeap, HashMap};

use super::Solver;

pub struct Day17 {}

const MOVES: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    pos: (usize, usize),
    dir: (i32, i32),
    steps: usize,
}

impl Node {
    fn new(pos: (usize, usize), dir: (i32, i32), steps: usize) -> Self {
        Self { pos, dir, steps }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct QueueItem {
    cost: usize,
    node: Node,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grid: &Vec<Vec<usize>>,
    start: (usize, usize),
    goal: (usize, usize),
    is_goal: fn(node: &Node, goal: &(usize, usize)) -> bool,
    min_steps: usize,
    max_steps: usize,
) -> usize {
    fn neighbors(
        grid: &Vec<Vec<usize>>,
        node: &Node,
        min_steps: usize,
        max_steps: usize,
    ) -> Vec<Node> {
        let mut neighbors = Vec::new();
        for d in MOVES.iter() {
            let next_pos = (node.pos.0 as i32 + d.0, node.pos.1 as i32 + d.1);
            if next_pos.0 < 0
                || next_pos.0 > grid.len() as i32 - 1
                || next_pos.1 < 0
                || next_pos.1 > grid[0].len() as i32 - 1
            {
                continue;
            }

            let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

            if d == &(-node.dir.0, -node.dir.1) {
                continue;
            } else if d != &node.dir && &node.steps >= &min_steps {
                neighbors.push(Node::new(next_pos, *d, 1));
            } else if d == &node.dir && node.steps < max_steps {
                neighbors.push(Node::new(next_pos, *d, node.steps + 1));
            }
        }
        return neighbors;
    }

    let starting_nodes = [
        (Node::new(start.clone(), (1, 0), 0), 0),
        (Node::new(start.clone(), (0, 1), 0), 0),
    ];

    let mut costs = HashMap::from(starting_nodes.clone());
    let mut queue: BinaryHeap<QueueItem> = starting_nodes
        .iter()
        .map(|k| QueueItem {
            cost: k.1,
            node: k.0.clone(),
        })
        .collect();

    while let Some(QueueItem { cost, node }) = queue.pop() {
        if is_goal(&node, &goal) {
            return cost;
        }

        for next in neighbors(grid, &node, min_steps, max_steps) {
            let next_cost = cost + grid[next.pos.0][next.pos.1];
            if let Some(&best_cost) = costs.get(&next) {
                if next_cost >= best_cost {
                    continue;
                }
            }

            costs.insert(next.clone(), next_cost);
            queue.push(QueueItem {
                cost: next_cost,
                node: next,
            });
        }
    }

    return 0;
}

impl Solver for Day17 {
    fn p1(&self, input: &str) -> String {
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let start = (0, 0);
        let goal = (grid.len() - 1, grid[0].len() - 1);

        fn is_goal(node: &Node, goal: &(usize, usize)) -> bool {
            return &node.pos == goal;
        }

        let res = dijkstra(&grid, start, goal, is_goal, 1, 3);
        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let grid: Vec<Vec<usize>> = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let start = (0, 0);
        let goal = (grid.len() - 1, grid[0].len() - 1);

        fn is_goal(node: &Node, goal: &(usize, usize)) -> bool {
            return &node.pos == goal && node.steps >= 4;
        }

        let res = dijkstra(&grid, start, goal, is_goal, 4, 10);
        return res.to_string();
    }
}
