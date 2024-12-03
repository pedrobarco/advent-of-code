import gleam/int

pub fn parse_unsafe_int(str: String) -> Int {
  let assert Ok(i) = int.parse(str)
  i
}
