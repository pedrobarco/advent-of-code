#include "day02.h"
#include <cstdlib>
#include <sstream>
#include <string>

int Day02Solver::kDay() {
    return 2;
}

std::string Day02Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    long invalid_ids_sum = 0;

    while (std::getline(ss, line, ',')) {
        if (!line.empty() && line.back() == '\n') {
            line.pop_back();
        }

        int range = line.find('-');
        std::string l = line.substr(0, range);
        std::string r = line.substr(range + 1);

        long from = std::stol(l);
        long to = std::stol(r);
        if (from > to) {
            continue;
        }

        for (long i = from; i <= to; i++) {
            std::string id = std::to_string(i);

            int mid = id.length() / 2;
            if(id.substr(0, mid) == id.substr(mid)) {
                invalid_ids_sum += i;
            }
        }
    }

    return std::to_string(invalid_ids_sum);
}

std::string Day02Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    long invalid_ids_sum = 0;

    while (std::getline(ss, line, ',')) {
        if (!line.empty() && line.back() == '\n') {
            line.pop_back();
        }

        int range = line.find('-');
        std::string l = line.substr(0, range);
        std::string r = line.substr(range + 1);

        long from = std::stol(l);
        long to = std::stol(r);
        if (from > to) {
            continue;
        }

        for (long i = from; i <= to; i++) {
            std::string id = std::to_string(i);

            int mid = id.length() / 2;
            for (int j = 1; j <= mid; j++) {
                auto w = id.substr(0, j);
                // check if w repeated forms the id

                std::string pattern;
                while (pattern.length() < id.length()) {
                    pattern += w;
                }

                if (pattern == id) {
                    invalid_ids_sum += i;
                    break;
                }
            }
        }
    }

    return std::to_string(invalid_ids_sum);
}

