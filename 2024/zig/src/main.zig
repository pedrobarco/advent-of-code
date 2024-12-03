const std = @import("std");
const Solver = @import("solver.zig").Solver;
const registry = @import("solver_registry.zig");

pub fn runSolver(solver: Solver, inputFile: []const u8) void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var arr = std.ArrayList(u8).init(allocator);
    defer arr.deinit();
    const writer = arr.writer().any();

    var buffer: [1024 * 1024]u8 = undefined;
    const input = readFile(&buffer, inputFile) catch return;

    solver.part1(input, writer);
    std.debug.print("Part 1: {s}\n", .{arr.items});

    arr.clearAndFree();
    solver.part2(input, writer);
    std.debug.print("Part 2: {s}\n", .{arr.items});
}

pub fn readFile(buffer: []u8, filePath: []const u8) ![]const u8 {
    const file = std.fs.cwd().openFile(filePath, std.fs.File.OpenFlags{}) catch |err| {
        std.debug.print("Unable to open file {s}: {}\n", .{ filePath, err });
        return err;
    };
    defer file.close();

    file.seekTo(0) catch |err| {
        std.debug.print("Unable to seek to beginning of file: {}\n", .{err});
        return err;
    };
    const bytes_read = file.readAll(buffer) catch |err| {
        std.debug.print("Unable to read file: {}\n", .{err});
        return err;
    };

    return buffer[0..bytes_read];
}

pub fn getInputFile(buffer: []u8, day: u32) []const u8 {
    return std.fmt.bufPrint(buffer, "inputs/day{d:0>2}.in", .{day}) catch unreachable;
}

pub fn getTestInputFile(buffer: []u8, day: u32) []const u8 {
    return std.fmt.bufPrint(buffer, "test_data/day{d:0>2}.in", .{day}) catch unreachable;
}

pub fn getTestOutputFile(buffer: []u8, day: u32, part: u32) []const u8 {
    return std.fmt.bufPrint(buffer, "test_data/day{d:0>2}p{d}.out", .{ day, part }) catch unreachable;
}

pub fn main() !void {
    const args = std.os.argv;

    if (args.len > 2) {
        std.debug.print("Usage: {s} [<day>]\n", .{args[0]});
        return std.process.exit(1);
    }

    var inputFileBuffer: [32]u8 = undefined;

    if (args.len == 2) {
        const str: []u8 = std.mem.span(args[1]);
        const day: u32 = try std.fmt.parseInt(u32, str, 10);
        const solver = try registry.get_solver(day);
        const inputFile = getInputFile(&inputFileBuffer, day);
        std.debug.print("Day {}\n", .{day});
        runSolver(solver, inputFile);
        return;
    }

    for (0.., registry.solvers) |i, solver| {
        const day = i + 1;
        const inputFile = getInputFile(&inputFileBuffer, @intCast(day));
        std.debug.print("Day {}\n", .{day});
        runSolver(solver, inputFile);
    }
}

pub fn test_solver(comptime day: u32) !void {
    const allocator = std.testing.allocator;

    var arr = std.ArrayList(u8).init(allocator);
    defer arr.deinit();
    const writer = arr.writer().any();

    const solver = try registry.get_solver(day);
    var filePathBuffer: [32]u8 = undefined;
    var fileBuffer: [1024 * 1024]u8 = undefined;
    var outputBuffer: [1024 * 1024]u8 = undefined;

    const inputFile = getTestInputFile(&filePathBuffer, day);
    const input = try readFile(&fileBuffer, inputFile);

    solver.part1(input, writer);
    const outputPart1File = getTestOutputFile(&filePathBuffer, day, 1);
    var outputPart1 = try readFile(&outputBuffer, outputPart1File);
    if (outputPart1.len != 0) {
        outputPart1 = outputPart1[0 .. outputPart1.len - 1];
    }
    try std.testing.expectEqualStrings(outputPart1, arr.items);

    arr.clearAndFree();

    solver.part2(input, writer);
    const outputPart2File = getTestOutputFile(&filePathBuffer, day, 2);
    var outputPart2 = try readFile(&outputBuffer, outputPart2File);
    if (outputPart2.len != 0) {
        outputPart2 = outputPart1[0 .. outputPart2.len - 1];
    }
    try std.testing.expectEqualStrings(outputPart2, arr.items);
}

test "solver" {
    try test_solver(1);
}
