const std = @import("std");
const Solver = @import("../solver.zig").Solver;
const SolverError = @import("../solver.zig").SolverError;

fn part1(input: []const u8, writer: std.io.AnyWriter) void {
    var buff = std.io.fixedBufferStream(input);
    const reader = buff.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var left_arr = std.ArrayList(u32).init(allocator);
    defer left_arr.deinit();

    var right_arr = std.ArrayList(u32).init(allocator);
    defer right_arr.deinit();

    while (true) {
        var int_buff: [100]u8 = undefined;

        const left = reader.readUntilDelimiter(&int_buff, ' ') catch break;
        const left_int = std.fmt.parseInt(u32, left, 10) catch unreachable;
        left_arr.append(left_int) catch unreachable;

        reader.skipBytes(2, .{}) catch unreachable;

        const right = (reader.readUntilDelimiterOrEof(&int_buff, '\n') catch break).?;
        const right_int = std.fmt.parseInt(u32, right, 10) catch unreachable;
        right_arr.append(right_int) catch unreachable;
    }

    std.mem.sort(u32, left_arr.items, {}, std.sort.asc(u32));
    std.mem.sort(u32, right_arr.items, {}, std.sort.asc(u32));

    var sum: u32 = 0;
    for (0.., left_arr.items) |i, _| {
        const l: i32 = @intCast(left_arr.items[i]);
        const r: i32 = @intCast(right_arr.items[i]);
        sum += @abs(l - r);
    }

    writer.print("{}", .{sum}) catch unreachable;
}

fn part2(input: []const u8, writer: std.io.AnyWriter) void {
    var buff = std.io.fixedBufferStream(input);
    const reader = buff.reader();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var left_arr = std.ArrayList(u32).init(allocator);
    defer left_arr.deinit();

    var right_arr = std.ArrayList(u32).init(allocator);
    defer right_arr.deinit();

    while (true) {
        var int_buff: [100]u8 = undefined;

        const left = reader.readUntilDelimiter(&int_buff, ' ') catch break;
        const left_int = std.fmt.parseInt(u32, left, 10) catch unreachable;
        left_arr.append(left_int) catch unreachable;

        reader.skipBytes(2, .{}) catch unreachable;

        const right = (reader.readUntilDelimiterOrEof(&int_buff, '\n') catch break).?;
        const right_int = std.fmt.parseInt(u32, right, 10) catch unreachable;
        right_arr.append(right_int) catch unreachable;
    }

    var map = std.AutoHashMap(u32, u32).init(allocator);
    defer map.deinit();

    for (right_arr.items) |r| {
        const entry = map.getOrPut(r) catch unreachable;
        if (!entry.found_existing) {
            entry.value_ptr.* = 0;
        }
        entry.value_ptr.* += 1;
    }

    var sum: u32 = 0;
    for (left_arr.items) |l| {
        const found = map.get(l) orelse continue;
        sum += l * found;
    }

    writer.print("{}", .{sum}) catch unreachable;
}

pub const solver = Solver{
    .part1 = part1,
    .part2 = part2,
};
