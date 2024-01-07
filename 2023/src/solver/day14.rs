use std::collections::HashMap;

use super::Solver;

pub struct Day14 {}

fn move_rocks(
    grid: &mut Vec<Vec<char>>,
    mut n: usize,
    direction: char,
    row_or_col: usize,
    start: usize,
    end: usize,
) {
    let range: Vec<usize> = if start < end {
        (start..end).collect()
    } else {
        (end..=start).rev().collect()
    };

    for k in range {
        let ele = match direction {
            'N' | 'S' => &mut grid[k][row_or_col],
            'W' | 'E' => &mut grid[row_or_col][k],
            _ => panic!("unknown direction"),
        };

        if ele == &'#' {
            continue;
        }

        if n > 0 {
            *ele = 'O';
            n -= 1;
            continue;
        }

        *ele = '.';
    }
}

fn tilt_grid(grid: &mut Vec<Vec<char>>, direction: char) {
    let inner_range: Vec<usize> = match direction {
        'N' => (0..grid.len()).collect(),
        'S' => (0..grid.len()).rev().collect(),
        'W' => (0..grid[0].len()).collect(),
        'E' => (0..grid[0].len()).rev().collect(),
        _ => panic!("unknown direction"),
    };

    let outer_range: Vec<usize> = match direction {
        'N' | 'S' => (0..grid[0].len()).collect(),
        'E' | 'W' => (0..grid.len()).collect(),
        _ => panic!("unknown direction"),
    };

    let end = match direction {
        'N' | 'W' => inner_range.last().unwrap().to_owned() + 1,
        'S' | 'E' => 0,
        _ => panic!("unknown direction"),
    };

    for &i in &outer_range {
        let mut rounded_rocks = 0;
        let mut start = inner_range.first().unwrap().to_owned();
        for &j in &inner_range {
            let pos = match direction {
                'N' | 'S' => (j, i),
                'E' | 'W' => (i, j),
                _ => panic!("unknown direction"),
            };

            let rock = grid[pos.0][pos.1];
            match rock {
                'O' => rounded_rocks += 1,
                '#' => {
                    if rounded_rocks > 0 {
                        move_rocks(grid, rounded_rocks, direction, i, start, j);
                    }
                    start = match direction {
                        'N' | 'W' => j + 1,
                        'S' | 'E' => {
                            if j == 0 {
                                j
                            } else {
                                j - 1
                            }
                        }
                        _ => panic!("unknown direction"),
                    };
                    rounded_rocks = 0;
                }
                _ => {}
            }
        }

        if rounded_rocks > 0 {
            move_rocks(grid, rounded_rocks, direction, i, start, end);
        }
    }
}

impl Solver for Day14 {
    fn p1(&self, input: &str) -> String {
        let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

        tilt_grid(&mut grid, 'N');
        let res: usize = grid
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (grid.len() - i))
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
        let mut grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

        let mut turns = 0;
        cache.insert(grid.clone(), turns);

        for cycle in ["NWSE"].iter().cycle() {
            turns += 1;

            for direction in cycle.chars() {
                tilt_grid(&mut grid, direction);
            }

            if let Some(prev_turn) = cache.get(&grid) {
                let n = 1_000_000_000;
                let j = prev_turn + ((n - prev_turn) % (turns - prev_turn));
                grid = cache
                    .iter()
                    .find_map(|(k, &v)| if v == j { Some(k) } else { None })
                    .unwrap()
                    .to_vec();
                break;
            }

            cache.insert(grid.clone(), turns);
        }

        let res: usize = grid
            .iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (grid.len() - i))
            .sum();

        return res.to_string();
    }
}
