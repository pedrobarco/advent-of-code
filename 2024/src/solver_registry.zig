const Solver = @import("solver.zig").Solver;
const day1 = @import("solvers/day1.zig");

pub const solvers = [_]Solver{
    day1.solver,
};

pub fn get_solver(day: u32) RegistryError!Solver {
    if (day < 1 or day > solvers.len) return RegistryError.InvalidDay;
    return solvers[day - 1];
}

const RegistryError = error{
    InvalidDay,
};
