import gleam/dict.{type Dict}
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import solver
import utils

type DiskBlock {
  File(Int)
  Free
}

type Disk =
  List(#(Int, DiskBlock))

type DiskMap =
  Dict(Int, DiskBlock)

type DiskPartition =
  #(DiskBlock, Int)

type IndexedDiskPartition =
  #(Int, DiskPartition)

type PartitionedDisk =
  List(DiskPartition)

fn sort_disk(disk_map: DiskMap, reversed_files: Disk, frees: Disk) -> DiskMap {
  case frees {
    [] -> disk_map
    [#(free_i, _), ..rest] -> {
      let assert Ok(#(file_i, file)) = reversed_files |> list.first()
      let next_frees = rest |> list.filter(fn(x) { x.0 < file_i })
      case free_i < file_i {
        True -> {
          let next_disk_map =
            disk_map |> dict.insert(free_i, file) |> dict.insert(file_i, Free)
          let next_reversed_files = reversed_files |> list.drop(1)
          sort_disk(next_disk_map, next_reversed_files, next_frees)
        }
        False -> sort_disk(disk_map, reversed_files, next_frees)
      }
    }
  }
}

fn insert_partition(
  disk: PartitionedDisk,
  partition: IndexedDiskPartition,
) -> PartitionedDisk {
  let #(i, p) = partition

  disk
  |> list.take(i)
  |> list.append([p])
  |> list.append(disk |> list.drop(i))
}

fn swap_partitions(
  disk: PartitionedDisk,
  p1: IndexedDiskPartition,
  p2: IndexedDiskPartition,
) -> PartitionedDisk {
  let #(left, right) = case p1.0 < p2.0 {
    True -> #(p1, p2)
    False -> #(p2, p1)
  }

  let first_segment =
    disk
    |> list.take(left.0)
    |> list.append([right.1])

  let second_segment =
    disk
    |> list.drop(left.0 + 1)
    |> list.take(right.0 - left.0 - 1)

  let third_segment = [left.1] |> list.append(disk |> list.drop(right.0 + 1))

  list.flatten([first_segment, second_segment, third_segment])
}

fn sort_partioned_disk(
  disk: PartitionedDisk,
  reversed_files: List(DiskPartition),
) -> PartitionedDisk {
  case reversed_files {
    [] -> disk
    [block, ..rest] -> {
      let #(file, file_size) = block
      let file_i =
        disk
        |> list.fold_until(0, fn(i, curr) {
          let #(block, _) = curr
          case block == file {
            True -> list.Stop(i)
            _ -> list.Continue(i + 1)
          }
        })

      let #(free_i, free_size) =
        disk
        |> list.fold_until(#(0, -1), fn(acc, curr) {
          let #(i, _) = acc
          let #(block, block_size) = curr

          case block {
            Free if block_size >= file_size && i < file_i ->
              list.Stop(#(i, block_size))
            _ -> list.Continue(#(i + 1, -1))
          }
        })

      case free_size == -1 {
        False -> {
          let new_free_size = free_size - file_size
          let next_disk = case new_free_size {
            0 -> {
              let free = #(Free, free_size)
              disk |> swap_partitions(#(file_i, block), #(free_i, free))
            }
            _ -> {
              let free = #(Free, file_size)
              let new_free = #(Free, new_free_size)
              disk
              |> swap_partitions(#(file_i, block), #(free_i, free))
              |> insert_partition(#(free_i + 1, new_free))
            }
          }

          sort_partioned_disk(next_disk, rest)
        }
        _ -> sort_partioned_disk(disk, rest)
      }
    }
  }
}

fn p1(input: String) -> String {
  let disk: Disk =
    input
    |> string.split(on: "")
    |> list.map(utils.parse_unsafe_int)
    |> list.sized_chunk(into: 2)
    |> list.index_map(fn(c, id) {
      let assert [file_size, free_size] = case c {
        [_, _] -> c
        [file_size] -> [file_size, 0]
        _ -> panic as "Invalid chunk"
      }
      list.repeat(File(id), times: file_size)
      |> list.append(list.repeat(Free, times: free_size))
    })
    |> list.flatten
    |> list.index_map(fn(block, i) { #(i, block) })

  let disk_map: DiskMap = disk |> dict.from_list
  let reversed_files: Disk =
    disk
    |> list.filter(fn(x) { x.1 != Free })
    |> list.reverse
  let frees: Disk =
    disk
    |> list.filter(fn(x) { x.1 == Free })

  sort_disk(disk_map, reversed_files, frees)
  |> dict.to_list
  |> list.map(fn(x) {
    case x.1 {
      File(id) -> x.0 * id
      _ -> 0
    }
  })
  |> list.fold(0, int.add)
  |> int.to_string
}

fn p2(input: String) -> String {
  let disk: PartitionedDisk =
    input
    |> string.split(on: "")
    |> list.map(utils.parse_unsafe_int)
    |> list.sized_chunk(into: 2)
    |> list.index_map(fn(c, id) {
      let assert [file_size, free_size] = case c {
        [_, _] -> c
        [file_size] -> [file_size, 0]
        _ -> panic as "Invalid chunk"
      }
      [#(File(id), file_size), #(Free, free_size)]
    })
    |> list.flatten

  let reversed_files: PartitionedDisk =
    disk
    |> list.filter(fn(x) { x.0 != Free })
    |> list.reverse

  sort_partioned_disk(disk, reversed_files)
  |> list.fold([0, 0], fn(acc, x) {
    let assert [sum, i] = acc
    let #(block, size) = x
    let next_i = i + size
    let next_sum =
      sum
      + case block {
        File(id) -> {
          list.range(i, next_i - 1)
          |> list.fold(0, fn(mini_acc, x) { mini_acc + { id * x } })
        }
        _ -> 0
      }
    [next_sum, next_i]
  })
  |> list.first()
  |> result.unwrap(0)
  |> int.to_string
}

pub const solver = solver.Solver(p1: p1, p2: p2)
