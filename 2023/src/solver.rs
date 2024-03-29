pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;

use std::fs;

pub trait Solver {
    fn p1(&self, input: &str) -> String;
    fn p2(&self, input: &str) -> String;
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
    fs::read_to_string(file_path)
        .unwrap()
        .trim_end()
        .to_string()
}

fn nth_day(n: &i32) -> String {
    format!("{:02}", n)
}
