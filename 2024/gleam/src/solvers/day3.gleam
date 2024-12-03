import solver
import gleam/regexp
import gleam/option.{Some}
import utils
import gleam/int

fn scan_memory (matches: List(regexp.Match), enabled: Bool, acc: Int) -> Int {
    case matches {
        [] -> acc
        [m, ..rest] -> {
            case m.content{
                "do()" -> scan_memory(rest, True, acc)
                "don't()" -> scan_memory(rest, False, acc)
                _ -> {
                    case enabled {
                        False -> scan_memory(rest, enabled, acc)
                        _ -> {
                            let assert [_, Some(a), Some(b)] =  m.submatches
                            let num_a = utils.parse_unsafe_int(a)
                            let num_b = utils.parse_unsafe_int(b)
                            scan_memory(rest, enabled, acc + num_a * num_b)
                        }
                    }
                }
            }
        }
    }
}

fn p1(input: String) -> String {
    let options = regexp.Options(multi_line: True, case_insensitive: False)
    let assert Ok(exp) = regexp.compile("(mul\\((\\d+),(\\d+)\\))", options)

    input |> regexp.scan(exp, _)
    |> scan_memory(True, 0)
    |> int.to_string
}

fn p2(input: String) -> String {
    let options = regexp.Options(multi_line: True, case_insensitive: False)
    let assert Ok(exp) = regexp.compile("(do\\(\\)|don't\\(\\)|mul\\((\\d+),(\\d+)\\))", options)

    input |> regexp.scan(exp, _)
    |> scan_memory(True, 0)
    |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
