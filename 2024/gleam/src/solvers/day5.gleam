import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/result
import gleam/set.{type Set}
import gleam/string
import solver
import utils

type PageOrderingRules =
  Dict(Int, List(Int))

type PageOrder =
  Dict(Int, List(Int))

fn is_page_ordered(
  rules: PageOrderingRules,
  page_order: PageOrder,
  page: Int,
  page_i: Int,
) -> Bool {
  case dict.get(rules, page) {
    Error(_) -> True
    Ok(deps) -> {
      case deps {
        [] -> True
        deps ->
          deps
          |> list.all(fn(d) {
            case dict.get(page_order, d) {
              Error(_) -> True
              Ok(dep_orders) ->
                dep_orders
                |> list.all(fn(i) { page_i < i })
            }
          })
      }
    }
  }
}

fn sort_page(
  rules: PageOrderingRules,
  page: List(#(Int, Int)),
) -> List(#(Int, Int)) {
  page
  |> list.map(fn(curr) {
    let deps_rules = dict.get(rules, curr.1) |> result.unwrap([])
    let deps = page |> list.filter(fn(x) { list.contains(deps_rules, x.1) })
    #(curr, deps)
  })
  |> dict.from_list()
  |> topo_sort(page, set.new(), [])
}

type Node =
  #(Int, Int)

type Graph =
  Dict(Node, List(Node))

fn topo_sort(
  graph: Graph,
  to_visit: List(Node),
  visited: Set(Node),
  path: List(Node),
) -> List(Node) {
  case to_visit {
    [] -> path
    [node, ..rest] -> {
      case set.contains(visited, node) {
        True -> topo_sort(graph, rest, visited, path)
        False -> {
          let #(visited, path) = dfs(graph, node, visited, path)
          topo_sort(graph, rest, visited, path)
        }
      }
    }
  }
}

fn dfs(
  graph: Graph,
  src: Node,
  visited: Set(Node),
  path: List(Node),
) -> #(Set(Node), List(Node)) {
  case set.contains(visited, src) {
    True -> #(visited, path)
    False -> {
      let deps = dict.get(graph, src) |> result.unwrap([])
      let #(visited, path) =
        list.fold(deps, #(visited, path), fn(acc, x) {
          dfs(graph, x, acc.0, acc.1)
        })
      let visited = set.insert(visited, src)
      #(visited, [src, ..path])
    }
  }
}

fn p1(input: String) -> String {
  let assert [rules, updates] =
    input
    |> string.split(on: "\n\n")

  let rules: PageOrderingRules =
    rules
    |> string.split(on: "\n")
    |> list.map(fn(x) {
      let assert [a, b] =
        string.split(x, on: "|")
        |> list.map(utils.parse_unsafe_int)
      #(a, b)
    })
    |> list.group(fn(x) { x.0 })
    |> dict.map_values(fn(_k, v) {
      v
      |> list.map(fn(x) { x.1 })
    })

  updates
  |> string.split(on: "\n")
  |> list.map(fn(x) {
    let page =
      x
      |> string.split(on: ",")
      |> list.index_map(fn(x, i) { #(i, utils.parse_unsafe_int(x)) })

    let grouped_by_value =
      page
      |> list.group(fn(x) { x.1 })
      |> dict.map_values(fn(_k, v) { list.map(v, fn(x) { x.0 }) })

    let all_ordered =
      list.all(page, fn(x) {
        is_page_ordered(rules, grouped_by_value, x.1, x.0)
      })

    case all_ordered {
      True -> page |> list.key_find(list.length(page) / 2) |> result.unwrap(0)
      False -> 0
    }
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

fn p2(input: String) -> String {
  let assert [rules, updates] =
    input
    |> string.split(on: "\n\n")

  let rules: PageOrderingRules =
    rules
    |> string.split(on: "\n")
    |> list.map(fn(x) {
      let assert [a, b] =
        string.split(x, on: "|")
        |> list.map(utils.parse_unsafe_int)
      #(a, b)
    })
    |> list.group(fn(x) { x.0 })
    |> dict.map_values(fn(_k, v) {
      v
      |> list.map(fn(x) { x.1 })
    })

  updates
  |> string.split(on: "\n")
  |> list.map(fn(x) {
    let page =
      x
      |> string.split(on: ",")
      |> list.index_map(fn(x, i) { #(i, utils.parse_unsafe_int(x)) })

    let grouped_by_value =
      page
      |> list.group(fn(x) { x.1 })
      |> dict.map_values(fn(_k, v) { list.map(v, fn(x) { x.0 }) })

    let all_ordered =
      list.all(page, fn(x) {
        is_page_ordered(rules, grouped_by_value, x.1, x.0)
      })

    case all_ordered {
      True -> 0
      False -> {
        sort_page(rules, page)
        |> list.drop(list.length(page) / 2)
        |> list.first
        |> result.unwrap(#(0, 0))
        |> fn(x) { x.1 }
      }
    }
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
