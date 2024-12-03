import gleam/int
import gleam/list
import gleam/option.{type Option, None, Some}
import gleam/string
import solver
import utils

type Sort {
  Asc
  Desc
}

fn is_safe_report(levels: List(#(Int, Int)), prev_sorting: Option(Sort)) -> Bool {
  case levels {
    [] -> True
    [x, ..next] -> {
      let dist = int.absolute_value(x.0 - x.1)
      let valid_dist = dist >= 1 && dist <= 3
      let sorting = case x.0 >= x.1 {
        True -> Asc
        _ -> Desc
      }
      let valid_sorting = case prev_sorting {
        None -> True
        Some(prev_sorting) -> prev_sorting == sorting
      }
      valid_dist && valid_sorting && is_safe_report(next, Some(sorting))
    }
  }
}

fn p1(input: String) -> String {
  let rows =
    input
    |> string.split(on: "\n")
    |> list.map(fn(l) {
      l
      |> string.split(on: " ")
      |> list.map(utils.parse_unsafe_int)
    })

  rows
  |> list.count(fn(row) {
    row
    |> list.window_by_2
    |> is_safe_report(None)
  })
  |> int.to_string
}

fn p2(input: String) -> String {
  let rows =
    input
    |> string.split(on: "\n")
    |> list.map(fn(l) {
      l
      |> string.split(on: " ")
      |> list.map(utils.parse_unsafe_int)
    })

  rows
  |> list.count(fn(row) {
    row
    |> list.index_map(fn(_, i) {
      list.append(list.take(row, i), list.drop(row, i + 1))
      |> list.window_by_2
    })
    |> list.any(is_safe_report(_, None))
  })
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
