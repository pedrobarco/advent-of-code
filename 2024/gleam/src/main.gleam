import argv
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string
import simplifile
import solver_registry

pub fn main() {
  case argv.load().arguments {
    [_, day] ->
      int.parse(day)
      |> result.unwrap(1)
      |> list.repeat(times: 1)
    [_] -> list.range(1, 25)
    [bin, ..] -> {
      io.print("Usage: " <> bin <> " [<day>]")
      []
    }
    _ -> panic as "THIS SHOULD NEVER HAPPEN"
  }
  |> list.map(run_solver)
}

type RunSolverError {
  InputError
  SolverError
}

pub fn run_solver(day: Int) {
  let input_path =
    "inputs/day"
    <> int.to_string(day)
    |> string.pad_start(to: 2, with: "0")
    <> ".in"
  let outs = {
    use solver <- result.try(
      solver_registry.get_solver(day)
      |> result.replace_error(SolverError),
    )
    use input <- result.try(
      simplifile.read(input_path)
      |> result.replace_error(InputError),
    )
    Ok(#(solver.p1(input), solver.p2(input)))
  }

  case outs {
    Ok(#(s1, s2)) -> {
      io.println("Day " <> int.to_string(day))
      io.println("Part 1: " <> s1)
      io.println("Part 2: " <> s2)
    }
    Error(SolverError) ->
      io.println("Solver not found for day " <> int.to_string(day))
    Error(InputError) ->
      io.println("Error reading input for day " <> int.to_string(day))
  }
  // use solver <- result.try(solver_registry.get_solver(day))

  //    case input {
  //        Ok(input) -> {
  //        io.println("Running solver for day " <> int.to_string(day))
  //        io.println("Input: " <> input)
  //        }
  //        Err(error) -> Nil
  //    }
}
