import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/string
import solver
import utils

type BlinkDP =
    Dict(#(Stone, Steps), NStones)
type Stone = Int
type Steps = Int
type NStones = Int

fn split_string(str: String) -> List(String) {
    let mid = string.length(str) / 2
    let str_arr = string.split(str, "")
    [list.take(str_arr, mid), list.drop(str_arr, mid)]
    |> list.map(string.join(_, with: ""))
}

fn blink(stone: Stone) -> List(Stone) {
    let stone_str = int.to_string(stone)
    let is_even = stone_str |> string.length |> int.is_even
    case stone {
        0 -> [1]
        _ if is_even ->
            split_string(stone_str) |> list.map(utils.parse_unsafe_int)
        x -> [x * 2024]
    }
}

fn blink_dp_stone(stone: Stone, steps: Steps, cache: BlinkDP) -> #(BlinkDP, NStones) {
    case steps {
        0 -> #(cache, 1)
        _ -> {
            case dict.get(cache, #(stone, steps)) {
                Ok(nstones) -> #(cache, nstones)
                _ -> {
                    let next_stones = blink(stone)
                    let #(next_cache, next_nstones) = blink_dp(next_stones, steps - 1, cache)
                    let next_new_cache = dict.insert(next_cache, #(stone, steps), next_nstones)
                    #(next_new_cache, next_nstones)
                }
            }
        }
    }
}

fn blink_dp(stones: List(Stone), steps: Steps, cache: BlinkDP) -> #(BlinkDP, NStones) {
    list.fold(stones, #(cache, 0), fn(acc, stone){
        let #(next_cache, nstones) = blink_dp_stone(stone, steps, acc.0)
        #(next_cache, acc.1 + nstones)
    })
}

fn p1(input: String) -> String {
    let stones = input |> string.split(" ") |> list.map(utils.parse_unsafe_int)
    blink_dp(stones, 25, dict.new()) |> fn(x) { x.1 } |> int.to_string
}

fn p2(input: String) -> String {
    let stones = input |> string.split(" ") |> list.map(utils.parse_unsafe_int)
    blink_dp(stones, 75, dict.new()) |> fn(x) { x.1 } |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
