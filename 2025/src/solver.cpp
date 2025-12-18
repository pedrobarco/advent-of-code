#include "solver.h"
#include <iomanip>

void Solver::solve(std::string input_one, std::string input_two) {
    std::cout << "=== Day " << this->kDay() << " ===" << std::endl;
    std::cout << "Part 1: " << std::quoted(this->one(input_one)) << std::endl;
    std::cout << "Part 2: " << std::quoted(this->two(input_two)) << std::endl;
}
