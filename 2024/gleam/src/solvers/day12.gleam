import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import solver

type Position {
  Position(x: Int, y: Int)
}

type Plant =
  String

type Grid =
  Dict(Position, Plant)

type Visited =
  Set(Position)

type RegionFound =
  #(Region, Visited)

type Region {
  Region(plant: Plant, perimeter: Int, area: Int, vertices: Int)
}

fn region_price(region: Region) -> Int {
  region.perimeter * region.area
}

fn region_bulk_price(region: Region) -> Int {
  region.area * region.vertices
}

fn neighbour(
  grid: Grid,
  position: Position,
  target: Plant,
  dir: Position,
) -> Result(Position, Position) {
  let p = Position(position.x + dir.x, position.y + dir.y)
  case dict.get(grid, p) {
    Ok(plant) if plant == target -> Ok(p)
    _ -> Error(p)
  }
}

fn neighbours(
  grid: Grid,
  position: Position,
  target: Plant,
) -> List(Result(Position, Position)) {
  [Position(1, 0), Position(-1, 0), Position(0, 1), Position(0, -1)]
  |> list.map(neighbour(grid, position, target, _))
}

fn count_vertices(grid: Grid, from: Position, target: Plant) -> Int {
  [
    #(Position(1, 0), Position(0, 1)),
    #(Position(-1, 0), Position(0, 1)),
    #(Position(1, 0), Position(0, -1)),
    #(Position(-1, 0), Position(0, -1)),
  ]
  |> list.filter(fn(x) {
    let #(dir1, dir2) = x
    let dir3 = Position(dir1.x + dir2.x, dir1.y + dir2.y)
    let assert [left, right, mid] =
      [dir1, dir2, dir3] |> list.map(neighbour(grid, from, target, _))
    let internal =
      result.is_ok(left) && result.is_ok(right) && result.is_error(mid)
    let external = result.is_error(left) && result.is_error(right)
    internal || external
  })
  |> list.length
}

fn find_region(
  grid: Grid,
  position: Position,
  target: Plant,
  visited: Visited,
  region: Region,
) -> RegionFound {
  case set.contains(visited, position) {
    True -> #(region, visited)
    False -> {
      let next_visited = set.insert(visited, position)
      let neighbours = neighbours(grid, position, target)
      let next_vertices =
        region.vertices + count_vertices(grid, position, target)
      let next_area = region.area + 1
      let next_perimeter =
        region.perimeter
        + { list.filter(neighbours, result.is_error) |> list.length }
      let next_region = Region(target, next_perimeter, next_area, next_vertices)
      neighbours
      |> list.fold(#(next_region, next_visited), fn(acc, neighbour) {
        let #(region, visited) = acc
        case neighbour {
          Ok(p) -> find_region(grid, p, target, visited, region)
          Error(_) -> acc
        }
      })
    }
  }
}

fn p1(input: String) -> String {
  let grid: Grid =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.index_map(fn(plant, x) { #(Position(x, y), plant) })
    })
    |> list.flatten
    |> dict.from_list

  grid
  |> dict.fold(#(0, set.new()), fn(acc, k, v) {
    let #(price, visited) = acc
    let #(region, next_visited) =
      find_region(grid, k, v, visited, Region(v, 0, 0, 0))
    let next_price = price + region_price(region)
    #(next_price, next_visited)
  })
  |> fn(x) { x.0 }
  |> int.to_string
}

fn p2(input: String) -> String {
  let grid: Grid =
    input
    |> string.split(on: "\n")
    |> list.index_map(fn(row, y) {
      row
      |> string.split(on: "")
      |> list.index_map(fn(plant, x) { #(Position(x, y), plant) })
    })
    |> list.flatten
    |> dict.from_list

  grid
  |> dict.fold(#(0, set.new()), fn(acc, k, v) {
    let #(price, visited) = acc
    let #(region, next_visited) =
      find_region(grid, k, v, visited, Region(v, 0, 0, 0))
    let next_price = price + region_bulk_price(region)
    #(next_price, next_visited)
  })
  |> fn(x) { x.0 }
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
