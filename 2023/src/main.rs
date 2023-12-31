use crate::solver::day01::Day01;
use crate::solver::day02::Day02;
use crate::solver::day03::Day03;
use crate::solver::day04::Day04;
use crate::solver::day05::Day05;
use crate::solver::day06::Day06;

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
