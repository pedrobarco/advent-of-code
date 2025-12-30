#include "registry.h"
#include "day01.h"
#include "day02.h"
#include "day03.h"
#include "day04.h"
#include "day05.h"
#include "day06.h"

Solver* solvers[25] = {
    new Day01Solver(),
    new Day02Solver(),
    new Day03Solver(),
    new Day04Solver(),
    new Day05Solver(),
    new Day06Solver(),
};
