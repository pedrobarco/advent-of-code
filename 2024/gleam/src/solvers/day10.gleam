import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/set.{type Set}
import gleam/string
import solver
import utils

type Position =
  #(Int, Int)

type Height =
  Int

type TopographicMap =
  Dict(Position, Height)

type Node {
  Node(pos: Position, height: Height, children: List(Node))
}

fn create_node_tree(map: TopographicMap, curr: Position) -> Node {
  let assert Ok(curr_height) = dict.get(map, curr)
  let children =
    neighbours(map, curr)
    |> list.filter_map(fn(neighbour) {
      case neighbour.1 == curr_height + 1 {
        True -> Ok(neighbour)
        False -> Error(Nil)
      }
    })
    |> list.map(fn(neighbour) { create_node_tree(map, neighbour.0) })
  Node(curr, curr_height, children)
}

fn count_nine_leafs(node: Node) -> Int {
  case node.children == [] {
    True ->
      case node.height == 9 {
        True -> 1
        False -> 0
      }
    False ->
      node.children |> list.map(count_nine_leafs) |> list.fold(0, int.add)
  }
}

fn neighbours(map: TopographicMap, pos: Position) -> List(#(Position, Height)) {
  [
    #(pos.0 - 1, pos.1),
    #(pos.0 + 1, pos.1),
    #(pos.0, pos.1 - 1),
    #(pos.0, pos.1 + 1),
  ]
  |> list.filter_map(fn(neighbour) {
    case dict.get(map, neighbour) {
      Ok(height) -> Ok(#(neighbour, height))
      Error(_) -> Error(Nil)
    }
  })
}

fn find_nines(
  map: TopographicMap,
  curr: #(Position, Height),
  reachable_nines: Set(Position),
) -> Set(Position) {
  case curr.1 == 9 {
    True -> reachable_nines |> set.insert(curr.0)
    False -> {
      let n = neighbours(map, curr.0)
      n
      |> list.filter_map(fn(neighbour) {
        case neighbour.1 == curr.1 + 1 {
          True -> Ok(find_nines(map, neighbour, reachable_nines))
          False -> Error(Nil)
        }
      })
      |> list.map(set.to_list)
      |> list.flatten
      |> set.from_list
    }
  }
}

fn trailhead_score(map: TopographicMap, trailhead: Position) -> Int {
  find_nines(map, #(trailhead, 0), set.new()) |> set.size
}

fn p1(input: String) -> String {
  let raw_map =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.map(utils.parse_unsafe_int)
      |> list.index_map(fn(height, x) { #(#(x, y), height) })
    })
    |> list.flatten

  let map: TopographicMap = raw_map |> dict.from_list
  let trailheads =
    raw_map
    |> list.filter_map(fn(x) {
      case x.1 == 0 {
        True -> Ok(x.0)
        False -> Error(Nil)
      }
    })

  trailheads
  |> list.map(trailhead_score(map, _))
  |> list.fold(0, int.add)
  |> int.to_string
}

fn p2(input: String) -> String {
  let raw_map =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.map(utils.parse_unsafe_int)
      |> list.index_map(fn(height, x) { #(#(x, y), height) })
    })
    |> list.flatten

  let map: TopographicMap = raw_map |> dict.from_list
  let trailheads =
    raw_map
    |> list.filter_map(fn(x) {
      case x.1 == 0 {
        True -> Ok(x.0)
        False -> Error(Nil)
      }
    })

  trailheads
  |> list.map(fn(t) {
    create_node_tree(map, t)
    |> count_nine_leafs
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
