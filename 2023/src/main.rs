use crate::solver::day01::Day01;
use crate::solver::day02::Day02;
use crate::solver::day03::Day03;
use crate::solver::day04::Day04;
use crate::solver::day05::Day05;
use crate::solver::day06::Day06;
use crate::solver::day07::Day07;
use crate::solver::day08::Day08;
use crate::solver::day09::Day09;
use crate::solver::day10::Day10;
use crate::solver::day11::Day11;
use crate::solver::day12::Day12;
use crate::solver::day13::Day13;
use crate::solver::day14::Day14;
use crate::solver::day15::Day15;
use crate::solver::day16::Day16;
use crate::solver::day17::Day17;
use crate::solver::day18::Day18;
use crate::solver::day19::Day19;
use crate::solver::day20::Day20;
use crate::solver::day21::Day21;

mod solver;

fn main() {
    let base = "./input";
    let mut solvers: Vec<Box<dyn solver::Solver>> = Vec::new();
    solvers.push(Box::new(Day01 {}));
    solvers.push(Box::new(Day02 {}));
    solvers.push(Box::new(Day03 {}));
    solvers.push(Box::new(Day04 {}));
    solvers.push(Box::new(Day05 {}));
    solvers.push(Box::new(Day06 {}));
    solvers.push(Box::new(Day07 {}));
    solvers.push(Box::new(Day08 {}));
    solvers.push(Box::new(Day09 {}));
    solvers.push(Box::new(Day10 {}));
    solvers.push(Box::new(Day11 {}));
    solvers.push(Box::new(Day12 {}));
    solvers.push(Box::new(Day13 {}));
    solvers.push(Box::new(Day14 {}));
    solvers.push(Box::new(Day15 {}));
    solvers.push(Box::new(Day16 {}));
    solvers.push(Box::new(Day17 {}));
    solvers.push(Box::new(Day18 {}));
    solvers.push(Box::new(Day19 {}));
    solvers.push(Box::new(Day20 {}));
    solvers.push(Box::new(Day21 {}));

    println!("############");
    println!("# AOC 2023 #");
    println!("############");

    for (i, s) in solvers.iter().enumerate() {
        let day: i32 = (i + 1).try_into().unwrap();
        let input = solver::nth_input(base, &day);
        println!("Day {}", day);
        println!("P1: {}", s.p1(&input));
        println!("P2: {}", s.p2(&input));
    }
}

#[cfg(test)]
mod tests {
    use super::solver;
    use super::solver::day01::Day01;
    use super::solver::day02::Day02;
    use crate::solver::day03::Day03;
    use crate::solver::day04::Day04;
    use crate::solver::day05::Day05;
    use crate::solver::day06::Day06;
    use crate::solver::day07::Day07;
    use crate::solver::day08::Day08;
    use crate::solver::day09::Day09;
    use crate::solver::day10::Day10;
    use crate::solver::day11::Day11;
    use crate::solver::day12::Day12;
    use crate::solver::day13::Day13;
    use crate::solver::day14::Day14;
    use crate::solver::day15::Day15;
    use crate::solver::day16::Day16;
    use crate::solver::day17::Day17;
    use crate::solver::day18::Day18;
    use crate::solver::day19::Day19;
    use crate::solver::day20::Day20;
    use crate::solver::day21::Day21;

    #[test]
    fn test_solvers() {
        let base = "./test_data";
        let mut solvers: Vec<Box<dyn solver::Solver>> = Vec::new();
        solvers.push(Box::new(Day01 {}));
        solvers.push(Box::new(Day02 {}));
        solvers.push(Box::new(Day03 {}));
        solvers.push(Box::new(Day04 {}));
        solvers.push(Box::new(Day05 {}));
        solvers.push(Box::new(Day06 {}));
        solvers.push(Box::new(Day07 {}));
        solvers.push(Box::new(Day08 {}));
        solvers.push(Box::new(Day09 {}));
        solvers.push(Box::new(Day10 {}));
        solvers.push(Box::new(Day11 {}));
        solvers.push(Box::new(Day12 {}));
        solvers.push(Box::new(Day13 {}));
        solvers.push(Box::new(Day14 {}));
        solvers.push(Box::new(Day15 {}));
        solvers.push(Box::new(Day16 {}));
        solvers.push(Box::new(Day17 {}));
        solvers.push(Box::new(Day18 {}));
        solvers.push(Box::new(Day19 {}));
        solvers.push(Box::new(Day20 {}));
        solvers.push(Box::new(Day21 {}));

        for (i, s) in solvers.iter().enumerate() {
            let day: i32 = (i + 1).try_into().unwrap();
            let input = solver::nth_input(base, &day);

            let want = solver::nth_pi_output(base, &day, &1);
            let got = s.p1(&input);
            assert_eq!(got, want, "day {day} part 1: got {got}, wanted {want}");

            let want = solver::nth_pi_output(base, &day, &2);
            let got = s.p2(&input);
            assert_eq!(got, want, "day {day} part 2: got {got}, wanted {want}");
        }
    }
}
