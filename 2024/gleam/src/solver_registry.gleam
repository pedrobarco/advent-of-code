import gleam/list
import gleam/result
import solver
import solvers/day1
import solvers/day10
import solvers/day11
import solvers/day2
import solvers/day3
import solvers/day4
import solvers/day5
import solvers/day6
import solvers/day7
import solvers/day8
import solvers/day9

const solvers: List(solver.Solver) = [
  day1.solver, day2.solver, day3.solver, day4.solver, day5.solver, day6.solver,
  day7.solver, day8.solver, day9.solver, day10.solver, day11.solver,
]

pub fn get_solver(day: Int) -> Result(solver.Solver, RegistryError) {
  solvers
  |> list.index_map(fn(s, i) { #(i, s) })
  |> list.key_find(day - 1)
  |> result.replace_error(InvalidDay)
}

pub type RegistryError {
  InvalidDay
}
