import gleam/dict.{type Dict}
import gleam/int
import gleam/io
import gleam/list
import gleam/order
import gleam/result
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
  io.debug(
    "TO SORT: "
    <> page
    |> list.map(fn(x) { x.1 })
    |> list.map(int.to_string)
    |> string.join(", "),
  )
  let sorted =
    page
    |> list.sort(fn(a, b) {
      case dict.get(rules, a.1) {
        Error(_) -> order.Eq
        Ok(deps) -> {
          io.debug("a = " <> a.1 |> int.to_string)
          io.debug("b = " <> b.1 |> int.to_string)
          io.debug(
            "deps(a) = " <> deps |> list.map(int.to_string) |> string.join(", "),
          )
          case list.contains(deps, b.1) {
            True -> order.Lt
            False -> order.Eq
          }
        }
      }
    })
  io.debug(
    "SORTED: "
    <> sorted
    |> list.map(fn(x) { x.1 })
    |> list.map(int.to_string)
    |> string.join(", "),
  )
  io.debug("-------------------------------------")
  sorted
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
        let all_ordered =
          page
          |> list.fold(page, fn(page, _) { sort_page(rules, page) })

        io.debug(
          "OLD: "
          <> page
          |> list.map(fn(x) { x.1 })
          |> list.map(int.to_string)
          |> string.join(","),
        )
        io.debug(
          "NEW: "
          <> all_ordered
          |> list.map(fn(x) { x.1 })
          |> list.map(int.to_string)
          |> string.join(","),
        )

        all_ordered
        |> list.key_find(list.length(page) / 2)
        |> result.unwrap(0)
      }
    }
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
