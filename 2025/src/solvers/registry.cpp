#include "registry.h"
#include "day01.h"
#include "day02.h"

Solver* solvers[25] = {
    new Day01Solver(),
    new Day02Solver(),
};
