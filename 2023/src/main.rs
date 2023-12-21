mod solver;

fn main() {
    let base = "./input";
    let solvers: Vec<Box<dyn solver::Solver>> = Vec::new();

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

    #[test]
    fn test_solvers() {
        let base = "./test_data";
        let solvers: Vec<Box<dyn solver::Solver>> = Vec::new();

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
