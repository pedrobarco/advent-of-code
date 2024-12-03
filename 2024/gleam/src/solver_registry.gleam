import gleam/list
import gleam/result
import solver
import solvers/day1
import solvers/day2
import solvers/day3

const solvers: List(solver.Solver) = [day1.solver, day2.solver, day3.solver]

pub fn get_solver(day: Int) -> Result(solver.Solver, RegistryError) {
  solvers
  |> list.index_map(fn(s, i) { #(i, s) })
  |> list.key_find(day - 1)
  |> result.replace_error(InvalidDay)
}

pub type RegistryError {
  InvalidDay
}
