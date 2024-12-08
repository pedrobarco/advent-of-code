import gleam/int
import gleam/string
import gleeunit
import gleeunit/should
import simplifile
import solver_registry

pub fn main() {
  gleeunit.main()
}

const day = 7

const part1 = "PART 1: "

const part2 = "PART 2: "

pub fn solvers_test() {
  let day_str =
    "day"
    <> int.to_string(day)
    |> string.pad_start(to: 2, with: "0")
  let in1_path = "test_data/" <> day_str <> "p1.in"
  let in2_path = "test_data/" <> day_str <> "p2.in"
  let out1_path = "test_data/" <> day_str <> "p1.out"
  let out2_path = "test_data/" <> day_str <> "p2.out"

  let solver = solver_registry.get_solver(day) |> should.be_ok
  let in1 = simplifile.read(in1_path) |> should.be_ok
  let in2 = simplifile.read(in2_path) |> should.be_ok

  let out1 =
    simplifile.read(out1_path) |> should.be_ok |> string.append(part1, _)
  let out2 =
    simplifile.read(out2_path) |> should.be_ok |> string.append(part2, _)

  in1 |> solver.p1 |> string.append(part1, _) |> should.equal(out1)
  in2 |> solver.p2 |> string.append(part2, _) |> should.equal(out2)
}
