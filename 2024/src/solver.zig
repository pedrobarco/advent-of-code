const std = @import("std");

pub const Solver = struct {
    part1: *const fn (input: []const u8, writer: std.io.AnyWriter) void,
    part2: *const fn (input: []const u8, writer: std.io.AnyWriter) void,
};

pub const SolverError = error{
    NotImplemented,
};
