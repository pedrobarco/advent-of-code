use std::fs;

pub trait Solver {
    fn p1(&self, input: &str) -> &str;
    fn p2(&self, input: &str) -> &str;
}

pub fn nth_input(base: &str, n: &i32) -> String {
    let input = format!("{base}/day{}.in", nth_day(n));
    parse_input(&input)
}

#[cfg(test)]
pub fn nth_pi_output(base: &str, n: &i32, i: &i32) -> String {
    let input = format!("{base}/day{}p{i}.out", nth_day(n));
    parse_input(&input)
}

fn parse_input(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}

fn nth_day(n: &i32) -> String {
    format!("{:02}", n)
}
