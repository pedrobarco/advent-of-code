use super::Solver;

pub struct Day06 {}

impl Solver for Day06 {
    fn p1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let res: usize = lines
            .next()
            .unwrap()
            .split(" ")
            .filter_map(|f| f.parse::<i32>().ok())
            .zip(
                lines
                    .next()
                    .unwrap()
                    .split(" ")
                    .filter_map(|f| f.parse::<i32>().ok()),
            )
            .map(|(ms, mm)| (0..=ms).filter(|delay| (delay * (ms - delay)) > mm).count())
            .product();

        return res.to_string();
    }

    fn p2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let ms: usize = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|f| f.parse::<usize>().is_ok())
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();
        let mm: usize = lines
            .next()
            .unwrap()
            .split(" ")
            .filter(|f| f.parse::<usize>().is_ok())
            .collect::<Vec<&str>>()
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();

        fn beats_record(ms: usize, mm: usize, delay: usize) -> bool {
            delay < ms && (delay * (ms - delay)) > mm
        }

        let range = (0..=ms).collect::<Vec<_>>();
        let res = range.partition_point(|&x| beats_record(ms, mm, x))
            - range.partition_point(|&x| !beats_record(ms, mm, x));

        return res.to_string();
    }
}
