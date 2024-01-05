use super::Solver;

pub struct Day13 {}

impl Solver for Day13 {
    fn p1(&self, input: &str) -> String {
        let res: usize = input
            .split("\n\n")
            .map(|pattern| {
                let lines = pattern.lines();
                let rows: Vec<String> = lines.map(|x| x.into()).collect();
                let cols: Vec<String> = (0..rows[0].len())
                    .map(|i| rows.iter().map(|r| r.chars().nth(i).unwrap()).collect())
                    .collect();

                let row_reflections = (0..rows.len())
                    .zip(1..rows.len())
                    .filter_map(|(i, j)| {
                        let max_reflect = i.min(rows.len() - 1 - j);
                        let mut left = (rows[i - max_reflect..i + 1]).to_vec();
                        let right = &rows[j..j + max_reflect + 1];
                        left.reverse();
                        return (left.eq(right)).then(|| j);
                    })
                    .max()
                    .unwrap_or(0);

                let col_reflections = (0..cols.len())
                    .zip(1..cols.len())
                    .filter_map(|(i, j)| {
                        let max_reflect = i.min(cols.len() - 1 - j);
                        let mut left = (cols[i - max_reflect..i + 1]).to_vec();
                        let right = &cols[j..j + max_reflect + 1];
                        left.reverse();
                        return (left.eq(right)).then(|| j);
                    })
                    .max()
                    .unwrap_or(0);

                return row_reflections * 100 + col_reflections;
            })
            .sum();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let res: usize = input
            .split("\n\n")
            .map(|pattern| {
                let lines = pattern.lines();
                let rows: Vec<String> = lines.map(|x| x.into()).collect();
                let cols: Vec<String> = (0..rows[0].len())
                    .map(|i| rows.iter().map(|r| r.chars().nth(i).unwrap()).collect())
                    .collect();

                let row_reflections = (0..rows.len())
                    .zip(1..rows.len())
                    .filter_map(|(i, j)| {
                        let max_reflect = i.min(rows.len() - 1 - j);
                        let mut left = (rows[i - max_reflect..i + 1]).to_vec();
                        let right = &rows[j..j + max_reflect + 1];
                        left.reverse();

                        let has_smudge = left
                            .iter()
                            .zip(right.iter())
                            .map(|(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count())
                            .sum::<usize>()
                            == 1;

                        return has_smudge.then(|| j);
                    })
                    .max()
                    .unwrap_or(0);

                let col_reflections = (0..cols.len())
                    .zip(1..cols.len())
                    .filter_map(|(i, j)| {
                        let max_reflect = i.min(cols.len() - 1 - j);
                        let mut left = (cols[i - max_reflect..i + 1]).to_vec();
                        let right = &cols[j..j + max_reflect + 1];
                        left.reverse();

                        let has_smudge = left
                            .iter()
                            .zip(right.iter())
                            .map(|(a, b)| a.chars().zip(b.chars()).filter(|(a, b)| a != b).count())
                            .sum::<usize>()
                            == 1;

                        return has_smudge.then(|| j);
                    })
                    .max()
                    .unwrap_or(0);

                return row_reflections * 100 + col_reflections;
            })
            .sum();

        return res.to_string();
    }
}
