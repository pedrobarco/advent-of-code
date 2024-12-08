import gleam/int
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/string
import solver
import utils

fn int_concat(l: Int, r: Int) -> Int {
  let l_str = l |> int.to_string
  let r_str = r |> int.to_string
  let res = utils.parse_unsafe_int(l_str <> r_str)
  res
}

fn can_compute(target: Int, numbers: List(Int), maybe_curr: Option(Int)) -> Bool {
  let curr = maybe_curr |> option.unwrap(0)
  case curr > target {
    True -> False
    False ->
      case numbers {
        [] -> curr == target
        [x, ..rest] -> {
          let next_add = maybe_curr |> option.unwrap(0) |> int.add(x) |> Some
          let next_mul =
            maybe_curr |> option.unwrap(1) |> int.multiply(x) |> Some
          can_compute(target, rest, next_add)
          || can_compute(target, rest, next_mul)
        }
      }
  }
}

fn can_compute_with_concat(
  target: Int,
  numbers: List(Int),
  maybe_curr: Option(Int),
) -> Bool {
  let curr = maybe_curr |> option.unwrap(0)
  case curr > target {
    True -> False
    False ->
      case numbers {
        [] -> curr == target
        [x, ..rest] -> {
          let next_add = maybe_curr |> option.unwrap(0) |> int.add(x) |> Some
          let next_mul =
            maybe_curr |> option.unwrap(1) |> int.multiply(x) |> Some
          let next_concat =
            maybe_curr |> option.unwrap(0) |> int_concat(x) |> Some
          can_compute_with_concat(target, rest, next_add)
          || can_compute_with_concat(target, rest, next_mul)
          || can_compute_with_concat(target, rest, next_concat)
        }
      }
  }
}

fn p1(input: String) -> String {
  input
  |> string.split(on: "\n")
  |> list.map(fn(l) {
    let assert [raw_target, raw_numbers] = l |> string.split(on: ": ")
    let target = utils.parse_unsafe_int(raw_target)
    let numbers =
      raw_numbers |> string.split(on: " ") |> list.map(utils.parse_unsafe_int)
    case can_compute(target, numbers, None) {
      True -> target
      False -> 0
    }
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

fn p2(input: String) -> String {
  input
  |> string.split(on: "\n")
  |> list.map(fn(l) {
    let assert [raw_target, raw_numbers] = l |> string.split(on: ": ")
    let target = utils.parse_unsafe_int(raw_target)
    let numbers =
      raw_numbers |> string.split(on: " ") |> list.map(utils.parse_unsafe_int)
    case can_compute_with_concat(target, numbers, None) {
      True -> target
      False -> 0
    }
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
