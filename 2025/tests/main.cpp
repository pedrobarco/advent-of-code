#include "../src/solvers/registry.h"
#include "../src/reader.h"
#include <iomanip>

int fail(int day, int part, const std::string& expected, const std::string& actual) {
    std::cerr << "FAILED: Day " << day << " Part " << part << std::endl;
    std::cerr << "  Expected: " << std::quoted(expected) << std::endl;
    std::cerr << "  Actual:   " << std::quoted(actual) << std::endl;
    return 1;
}

int main() {
    for (auto solver : solvers) {
        if (solver == nullptr) {
            continue;
        }
        auto day = solver->kDay();

        auto input_one = read_example_input(day, 1);
        auto expected_one = read_example_output(day, 1);
        auto actual_one = solver->one(input_one);
        if (actual_one != expected_one) {
            return fail(day, 1, expected_one, actual_one);
        }

        auto input_two = read_example_input(day, 2);
        auto expected_two = read_example_output(day, 2);
        auto actual_two = solver->two(input_two);
        if (actual_two != expected_two) {
            return fail(day, 2, expected_two, actual_two);
        }
    }

    std::cout << "All tests passed!" << std::endl;
    return 0;
}
