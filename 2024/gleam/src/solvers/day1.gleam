import gleam/dict
import gleam/int
import gleam/list
import gleam/option.{None, Some}
import gleam/result
import gleam/string
import solver
import utils

fn p1(input: String) -> String {
  input
  |> string.split(on: "\n")
  |> list.map(fn(l) {
    l
    |> string.split(on: "   ")
    |> list.map(utils.parse_unsafe_int)
  })
  |> list.transpose
  |> list.map(list.sort(_, by: int.compare))
  |> list.interleave
  |> list.sized_chunk(into: 2)
  |> list.map(list.fold(_, 0, fn(a, b) { int.absolute_value(a - b) }))
  |> list.fold(0, int.add)
  |> int.to_string
}

fn p2(input: String) -> String {
  let assert [left, right] =
    input
    |> string.split(on: "\n")
    |> list.map(fn(l) {
      l
      |> string.split(on: "   ")
      |> list.map(utils.parse_unsafe_int)
    })
    |> list.transpose

  let map =
    right
    |> list.fold(dict.new(), fn(map, x) {
      dict.upsert(map, x, fn(b) {
        case b {
          Some(i) -> i + 1
          None -> 1
        }
      })
    })

  left
  |> list.map(fn(l) {
    let r = dict.get(map, l) |> result.unwrap(0)
    l * r
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
