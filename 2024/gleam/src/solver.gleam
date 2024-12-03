type SolveFn =
  fn(String) -> String

pub type Solver {
  Solver(p1: SolveFn, p2: SolveFn)
}
