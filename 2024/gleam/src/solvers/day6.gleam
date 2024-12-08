import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import solver

type Cell {
  Guard(Direction)
  Obstacle
  Empty
}

type Direction {
  Up
  Down
  Left
  Right
}

type Position =
  #(Int, Int)

type Grid =
  Dict(Position, Cell)

type ObstacleMap =
  Dict(Int, List(Position))

type DistinctPositions =
  Set(Position)

type Visited =
  Set(#(Cell, Position))

fn patrol(
  grid: Grid,
  guard: Cell,
  at: Position,
  distinct: DistinctPositions,
) -> DistinctPositions {
  let dir = case guard {
    Guard(x) -> x
    _ -> panic as "Invalid cell"
  }

  let distinct = distinct |> set.insert(at)
  let next_pos = move_guard(at, dir)

  case dict.get(grid, next_pos) {
    Error(_) -> distinct
    Ok(next_cell) ->
      case next_cell {
        Obstacle -> {
          let next_dir = turn_right(dir)
          let next_guard = Guard(next_dir)
          let next_grid = grid |> dict.insert(at, next_guard)
          patrol(next_grid, next_guard, at, distinct)
        }
        Empty -> {
          let next_grid =
            grid |> dict.insert(at, Empty) |> dict.insert(next_pos, guard)
          patrol(next_grid, guard, next_pos, distinct)
        }
        Guard(_) -> panic as "Invalid cell"
      }
  }
}

fn is_pratol_loop(
  grid: Grid,
  obstacles_by_x: ObstacleMap,
  obstacles_by_y: ObstacleMap,
  guard: Cell,
  at: Position,
  visited: Visited,
) -> Bool {
  let dir = case guard {
    Guard(x) -> x
    _ -> panic as "Invalid cell"
  }

  let incoming_obstacles =
    case dir {
      Up | Down -> obstacles_by_x |> dict.get(at.0) |> result.unwrap([])
      Left | Right -> obstacles_by_y |> dict.get(at.1) |> result.unwrap([])
    }
    |> list.filter(fn(o) {
      case dir {
        Up -> o.1 < at.1
        Down -> o.1 > at.1
        Left -> o.0 < at.0
        Right -> o.0 > at.0
      }
    })

  case incoming_obstacles {
    [] -> False
    obstacles -> {
      let assert Ok(first) = obstacles |> list.first
      let next_obstacle =
        obstacles
        |> list.fold(first, fn(acc, x) {
          let x_dist =
            int.absolute_value(x.0 - at.0) + int.absolute_value(x.1 - at.1)
          let acc_dist =
            int.absolute_value(acc.0 - at.0) + int.absolute_value(acc.1 - at.1)
          case x_dist < acc_dist {
            True -> x
            False -> acc
          }
        })
      let next_pos = move_guard(next_obstacle, inverse(dir))
      let next_guard = Guard(turn_right(dir))
      let next_grid =
        grid |> dict.insert(at, Empty) |> dict.insert(next_pos, next_guard)

      let next = #(guard, next_pos)
      case visited |> set.contains(next) {
        True -> True
        False -> {
          let next_visited = visited |> set.insert(#(guard, next_pos))
          is_pratol_loop(
            next_grid,
            obstacles_by_x,
            obstacles_by_y,
            next_guard,
            next_pos,
            next_visited,
          )
        }
      }
    }
  }
}

fn move_guard(from: Position, direction: Direction) -> Position {
  let #(x, y) = from
  case direction {
    Up -> #(x, y - 1)
    Down -> #(x, y + 1)
    Left -> #(x - 1, y)
    Right -> #(x + 1, y)
  }
}

fn turn_right(from: Direction) -> Direction {
  case from {
    Up -> Right
    Right -> Down
    Down -> Left
    Left -> Up
  }
}

fn inverse(from: Direction) -> Direction {
  case from {
    Up -> Down
    Right -> Left
    Down -> Up
    Left -> Right
  }
}

fn p1(input: String) -> String {
  let raw_grid =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.index_map(fn(cell, x) {
        let item: Cell = case cell {
          "." -> Empty
          "#" -> Obstacle
          "^" -> Guard(Up)
          ">" -> Guard(Right)
          "<" -> Guard(Left)
          "v" -> Guard(Down)
          _ -> panic as "Invalid cell"
        }
        #(#(x, y), item)
      })
    })
    |> list.flatten

  let grid: Grid = raw_grid |> dict.from_list

  let assert Ok(#(start_pos, guard)) =
    raw_grid
    |> list.find(fn(x) {
      case x.1 {
        Guard(_) -> True
        _ -> False
      }
    })

  patrol(grid, guard, start_pos, set.new())
  |> set.size
  |> int.to_string
}

fn p2(input: String) -> String {
  let raw_grid =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.index_map(fn(cell, x) {
        let item: Cell = case cell {
          "." -> Empty
          "#" -> Obstacle
          "^" -> Guard(Up)
          ">" -> Guard(Right)
          "<" -> Guard(Left)
          "v" -> Guard(Down)
          _ -> panic as "Invalid cell"
        }
        #(#(x, y), item)
      })
    })
    |> list.flatten

  let grid: Grid = raw_grid |> dict.from_list

  let assert Ok(#(start_pos, guard)) =
    raw_grid
    |> list.find(fn(x) {
      case x.1 {
        Guard(_) -> True
        _ -> False
      }
    })

  let obstacles =
    raw_grid
    |> list.filter_map(fn(x) {
      case x.1 {
        Obstacle -> Ok(x.0)
        _ -> Error(Nil)
      }
    })

  patrol(grid, guard, start_pos, set.new())
  |> set.to_list
  |> list.count(fn(x) {
    let new_obstacles = obstacles |> list.append([x])
    let obstacles_by_x =
      new_obstacles
      |> list.group(fn(x) { x.0 })
    let obstacles_by_y =
      new_obstacles
      |> list.group(fn(x) { x.1 })
    let new_grid = grid |> dict.insert(x, Obstacle)
    is_pratol_loop(
      new_grid,
      obstacles_by_x,
      obstacles_by_y,
      guard,
      start_pos,
      set.new(),
    )
  })
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
