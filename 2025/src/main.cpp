#include <iostream>
#include "solvers/registry.h"
#include "reader.h"

int main(int argc, char* argv[]) {
    auto hasArg = argc > 1;
    auto dayStr = hasArg ? argv[1] : "0";
    auto day = std::stoi(dayStr);

    if (hasArg && (day < 1 || day > 25)) {
        std::cerr << "error: day must be between 1 and 25" << std::endl;
        return 1;
    }

    if (hasArg) {
        int i = day - 1;
        auto solver = solvers[i];
        if (solver == nullptr) {
            std::cerr << "error: solver for day " << day << " is not implemented" << std::endl;
            return 1;
        }
        solver->solve(read_input(day, 1), read_input(day, 2));
        return 0;
    }

    for (auto solver : solvers) {
        if (solver == nullptr) {
            continue;
        }
        solver->solve(read_input(solver->kDay(), 1), read_input(solver->kDay(), 2));
    }
    return 0;
}

