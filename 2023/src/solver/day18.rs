use super::Solver;

pub struct Day18 {}

fn grid_area(vertices: &Vec<(isize, isize)>, steps: usize) -> usize {
    let area = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| (a.0 * b.1) - (a.1 * b.0))
        .sum::<isize>()
        .abs()
        / 2;

    return area as usize + (steps / 2) + 1;
}

impl Solver for Day18 {
    fn p1(&self, input: &str) -> String {
        let mut curr = (0, 0);
        let mut steps = 0;
        let vertices: Vec<(isize, isize)> = input
            .lines()
            .map(|l| {
                let matches: Vec<_> = l.split(" ").collect();
                return (matches[0], matches[1].parse::<usize>().unwrap());
            })
            .map(|(dir, n)| {
                let dir = match dir {
                    "U" => (-1, 0),
                    "D" => (1, 0),
                    "R" => (0, 1),
                    "L" => (0, -1),
                    _ => panic!("unknown direction"),
                };

                steps += n;
                let n = n as isize;
                curr = (curr.0 + (dir.0 * n), curr.1 + (dir.1 * n));
                return curr;
            })
            .collect();

        let res = grid_area(&vertices, steps);

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut curr = (0, 0);
        let mut steps = 0;
        let vertices: Vec<(isize, isize)> = input
            .lines()
            .map(|l| {
                let matches: Vec<_> = l.split(" ").collect();

                let dir = match matches[2]
                    .chars()
                    .skip_while(|x| *x != '#')
                    .skip(6)
                    .next()
                    .unwrap()
                {
                    '0' => "R",
                    '1' => "D",
                    '2' => "L",
                    '3' => "U",
                    _ => panic!("unknown direction {:?}", matches[2]),
                };

                let dir: (isize, isize) = match dir {
                    "U" => (-1, 0),
                    "D" => (1, 0),
                    "R" => (0, 1),
                    "L" => (0, -1),
                    _ => panic!("unknown direction"),
                };

                let n = usize::from_str_radix(
                    &matches[2]
                        .chars()
                        .skip_while(|&x| x != '#')
                        .skip(1)
                        .take(5)
                        .collect::<String>(),
                    16,
                )
                .unwrap();

                steps += n;

                let n = n as isize;
                curr = (curr.0 + (dir.0 * n), curr.1 + (dir.1 * n));

                return curr;
            })
            .collect();

        let res = grid_area(&vertices, steps);
        return res.to_string();
    }
}
