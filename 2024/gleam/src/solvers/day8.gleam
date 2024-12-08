import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import solver

type Cell {
  Empty
  Antena(String)
}

type Position =
  #(Int, Int)

type Grid =
  Dict(Position, Cell)

type AntenaMap =
  Dict(Cell, List(Position))

type Antinodes =
  Set(Position)

fn compute_antinode(
  positions: #(Position, Position),
  freq_count: #(Int, Int),
) -> Set(Position) {
  let #(a, b) = positions
  let dx = b.0 - a.0
  let dy = b.1 - a.1
  list.range(freq_count.0, freq_count.1)
  |> list.map(fn(x) {
    let antinode_a = #(b.0 + dx * x, b.1 + dy * x)
    let antinode_b = #(a.0 - dx * x, a.1 - dy * x)
    set.from_list([antinode_a, antinode_b])
  })
  |> list.fold(set.new(), set.union)
}

fn compute_antinodes(
  grid: Grid,
  antena_map: AntenaMap,
  antena: Cell,
  freq_count: #(Int, Int),
) -> Antinodes {
  dict.get(antena_map, antena)
  |> result.unwrap([])
  |> list.combination_pairs
  |> list.map(compute_antinode(_, freq_count))
  |> list.fold(set.new(), set.union)
  |> set.filter(dict.has_key(grid, _))
}

fn p1(input: String) -> String {
  let raw_grid: List(#(Position, Cell)) =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.index_map(fn(cell, x) {
        let item: Cell = case cell {
          "." -> Empty
          _ -> Antena(cell)
        }
        #(#(x, y), item)
      })
    })
    |> list.flatten

  let grid: Grid = raw_grid |> dict.from_list
  let antenas: AntenaMap =
    raw_grid
    |> list.filter(fn(x) {
      case x.1 {
        Antena(_) -> True
        _ -> False
      }
    })
    |> list.group(fn(x) { x.1 })
    |> dict.map_values(fn(_, x) { x |> list.map(fn(y) { y.0 }) })

  antenas
  |> dict.keys
  |> list.map(compute_antinodes(grid, antenas, _, #(1, 1)))
  |> list.fold(set.new(), set.union)
  |> set.size
  |> int.to_string
}

fn p2(input: String) -> String {
  let raw_grid: List(#(Position, Cell)) =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.index_map(fn(cell, x) {
        let item: Cell = case cell {
          "." -> Empty
          _ -> Antena(cell)
        }
        #(#(x, y), item)
      })
    })
    |> list.flatten

  let grid: Grid = raw_grid |> dict.from_list
  let antenas: AntenaMap =
    raw_grid
    |> list.filter(fn(x) {
      case x.1 {
        Antena(_) -> True
        _ -> False
      }
    })
    |> list.group(fn(x) { x.1 })
    |> dict.map_values(fn(_, x) { x |> list.map(fn(y) { y.0 }) })

  let max_x =
    grid |> dict.keys |> list.map(fn(x) { x.0 }) |> list.fold(0, int.max)
  let max_y =
    grid |> dict.keys |> list.map(fn(x) { x.1 }) |> list.fold(0, int.max)
  let freq_count = #(0, int.max(max_x, max_y))

  antenas
  |> dict.keys
  |> list.map(compute_antinodes(grid, antenas, _, freq_count))
  |> list.fold(set.new(), set.union)
  |> set.size
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
