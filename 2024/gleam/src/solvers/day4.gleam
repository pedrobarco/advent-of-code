import gleam/dict
import gleam/function
import gleam/int
import gleam/list
import gleam/string
import solver

type Pos =
  #(Int, Int)

type Grid =
  dict.Dict(Pos, String)

type Direction {
  Left
  Right
  Up
  Down
  UpRight
  UpLeft
  DownRight
  DownLeft
}

fn is_valid_move(prev: String, curr: String) -> Bool {
  case prev {
    "" -> curr == "X"
    "X" -> curr == "M"
    "M" -> curr == "A"
    "A" -> curr == "S"
    "S" -> True
    _ -> False
  }
}

fn next_pos(pos: Pos, dir: Direction) -> Pos {
  case dir {
    Left -> #(pos.0 - 1, pos.1)
    Right -> #(pos.0 + 1, pos.1)
    Up -> #(pos.0, pos.1 - 1)
    Down -> #(pos.0, pos.1 + 1)
    UpRight -> #(pos.0 + 1, pos.1 - 1)
    UpLeft -> #(pos.0 - 1, pos.1 - 1)
    DownRight -> #(pos.0 + 1, pos.1 + 1)
    DownLeft -> #(pos.0 - 1, pos.1 + 1)
  }
}

fn find_xmas(grid: Grid, prev: String, curr: Pos, to: Direction) -> Bool {
  let res = dict.get(grid, curr)
  case res {
    Error(_) -> False
    Ok(v) -> {
      case is_valid_move(prev, v) {
        True -> {
          case v {
            "S" -> True
            _ -> find_xmas(grid, v, next_pos(curr, to), to)
          }
        }
        False -> False
      }
    }
  }
}

fn p1(input: String) -> String {
  let grid: Grid =
    input
    |> string.split(on: "\n")
    |> list.map(string.split(_, on: ""))
    |> list.index_map(fn(row, y) {
      row
      |> list.index_map(fn(v, x) { #(#(x, y), v) })
    })
    |> list.flatten
    |> dict.from_list

  grid
  |> dict.filter(fn(_, v) { v == "X" })
  |> dict.map_values(fn(k, _) {
    [Left, Right, Up, Down, UpRight, UpLeft, DownRight, DownLeft]
    |> list.count(find_xmas(grid, "", k, _))
  })
  |> dict.values
  |> list.fold(0, int.add)
  |> int.to_string
}

fn p2(input: String) -> String {
  let grid: Grid =
    input
    |> string.split(on: "\n")
    |> list.map(string.split(_, on: ""))
    |> list.index_map(fn(row, y) {
      row
      |> list.index_map(fn(v, x) { #(#(x, y), v) })
    })
    |> list.flatten
    |> dict.from_list

  grid
  |> dict.filter(fn(_, v) { v == "A" })
  |> dict.map_values(fn(k, _) {
    [[UpRight, DownLeft], [UpLeft, DownRight]]
    |> list.all(fn(dirs) {
      let word =
        dirs
        |> list.map(next_pos(k, _))
        |> list.filter_map(dict.get(grid, _))
        |> string.join(with: "")
      word == "MS" || word == "SM"
    })
  })
  |> dict.values
  |> list.count(function.identity)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
