use std::collections::HashSet;

use super::Solver;

pub struct Day16 {}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn walk(
    grid: &Vec<Vec<char>>,
    mut energized_tiles: &mut HashSet<((usize, usize), u8)>,
    pos: &(usize, usize),
    dir: &Direction,
) {
    let next_dirs: Vec<(&Direction, (usize, usize))> = match grid[pos.0][pos.1] {
        '|' => match dir {
            Direction::Right | Direction::Left => vec![&Direction::Up, &Direction::Down],
            _ => vec![dir],
        },
        '-' => match dir {
            Direction::Up | Direction::Down => vec![&Direction::Left, &Direction::Right],
            _ => vec![dir],
        },
        '\\' => match dir {
            Direction::Up => vec![&Direction::Left],
            Direction::Down => vec![&Direction::Right],
            Direction::Left => vec![&Direction::Up],
            Direction::Right => vec![&Direction::Down],
        },
        '/' => match dir {
            Direction::Up => vec![&Direction::Right],
            Direction::Down => vec![&Direction::Left],
            Direction::Left => vec![&Direction::Down],
            Direction::Right => vec![&Direction::Up],
        },
        _ => vec![dir],
    }
    .iter()
    .filter_map(|&dir| {
        let next_unsafe = match dir {
            Direction::Up => (pos.0 as i32 - 1, pos.1 as i32),
            Direction::Down => (pos.0 as i32 + 1, pos.1 as i32),
            Direction::Right => (pos.0 as i32, pos.1 as i32 + 1),
            Direction::Left => (pos.0 as i32, pos.1 as i32 - 1),
        };
        let oob = next_unsafe.0 < 0
            || next_unsafe.0 > (grid.len() - 1) as i32
            || next_unsafe.1 < 0
            || next_unsafe.1 > (grid[0].len() - 1) as i32;

        return (!oob).then(|| {
            let next = (next_unsafe.0 as usize, next_unsafe.1 as usize);
            return (dir, next);
        });
    })
    .collect();

    for (dir, next) in next_dirs {
        let dir_u8 = match dir {
            Direction::Right => 0,
            Direction::Left => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        };

        if !energized_tiles.insert((next.clone(), dir_u8)) {
            continue;
        }

        walk(&grid, &mut energized_tiles, &next, &dir);
    }
}

impl Solver for Day16 {
    fn p1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start = (0, 0);
        let mut energized_tiles: HashSet<((usize, usize), u8)> = HashSet::new();
        energized_tiles.insert((start, 0));
        walk(&grid, &mut energized_tiles, &start, &Direction::Right);

        let res = energized_tiles
            .iter()
            .map(|(pos, _)| pos)
            .collect::<HashSet<&(usize, usize)>>()
            .len();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let res: usize = (0..grid.len())
            .map(|i| {
                let mut pos = vec![(i, 0), (i, grid[i].len() - 1)];
                if i == 0 || i == grid.len() - 1 {
                    (1..grid[i].len() - 1)
                        .map(|j| (i, j))
                        .for_each(|p| pos.push(p));
                }
                return pos;
            })
            .flatten()
            .map(|p| {
                let mut dirs: Vec<Direction> = vec![];

                if p.0 == 0 {
                    dirs.push(Direction::Down);
                }

                if p.0 == grid.len() - 1 {
                    dirs.push(Direction::Up);
                }

                if p.1 == 0 {
                    dirs.push(Direction::Right);
                }

                if p.1 == grid[0].len() - 1 {
                    dirs.push(Direction::Left);
                }

                return dirs
                    .iter()
                    .map(|d| {
                        let mut energized_tiles: HashSet<((usize, usize), u8)> = HashSet::new();
                        energized_tiles.insert((
                            p,
                            match d {
                                Direction::Right => 0,
                                Direction::Left => 1,
                                Direction::Up => 2,
                                Direction::Down => 3,
                            },
                        ));
                        walk(&grid, &mut energized_tiles, &p, d);
                        return energized_tiles
                            .iter()
                            .map(|(pos, _)| pos)
                            .collect::<HashSet<&(usize, usize)>>()
                            .len();
                    })
                    .max()
                    .unwrap();
            })
            .max()
            .unwrap();

        return res.to_string();
    }
}
