use super::Solver;

pub struct Day11 {}

impl Solver for Day11 {
    fn p1(&self, input: &str) -> String {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().map(|c| c).collect())
            .collect();

        let empty_rows = (0..grid.len())
            .filter(|&i| grid[i].iter().find(|&&c| c == '#').is_none())
            .collect::<Vec<usize>>();

        let empty_cols = (0..grid[0].len())
            .filter(|&i| (0..grid.len()).find(|&k| grid[k][i] == '#').is_none())
            .collect::<Vec<usize>>();

        let galaxies: Vec<(usize, usize)> = grid
            .iter()
            .enumerate()
            .flat_map(|(i, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(_, &ch)| ch == '#')
                    .map(|(j, _)| (i, j))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        // println!("{} x {}", grid.len(), grid[0].len());
        // println!("empty rows: {:?}", empty_rows);
        // println!("empty cols: {:?}", empty_cols);
        // println!("galaxies: {:?}", galaxies);
        // grid.iter().for_each(|r| {
        //     println!("{:?}", r);
        // });

        let res: usize = galaxies
            .iter()
            .enumerate()
            .map(|(i, start)| {
                galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|end| {
                        let dx = empty_rows
                            .iter()
                            .filter(|&&r| r > end.0.min(start.0) && r < end.0.max(start.0))
                            .count();
                        let dy = empty_cols
                            .iter()
                            .filter(|&&c| c > end.1.min(start.1) && c < end.1.max(start.1))
                            .count();
                        let dist = (end.0.max(start.0) - end.0.min(start.0))
                            + (end.1.max(start.1) - end.1.min(start.1));
                        return dist + dx + dy;
                    })
                    .sum::<usize>()
            })
            .sum();

        return res.to_string();
    }

    fn p2(&self, _input: &str) -> String {
        return "".to_string();
    }
}
