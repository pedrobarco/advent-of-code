#include "day03.h"
#include <algorithm>
#include <cstddef>
#include <cstdlib>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

int Day03Solver::kDay() {
    return 3;
}

namespace {

    class Battery {
        public:
            int index;
            int digit;
    };

    class Bank {
        public:
            std::size_t size;
            std::vector<Battery> batteries;
            Bank(std::size_t size) : size(size), batteries(size) {}

            static long joltage(const std::vector<Battery>& batteries) {
                std::vector<int> order(batteries.size());
                std::iota(order.begin(), order.end(), 0);

                std::sort(order.begin(), order.end(),
                        [&](int a, int b) {
                        return batteries[a].index < batteries[b].index;
                        });

                long total = 0;
                for (int idx : order) {
                    if (batteries[idx].digit == 0 ) {
                        continue;
                    }
                    total = total * 10 + batteries[idx].digit;
                }
                return total;
            }
    };

}

std::string Day03Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    long total_joltage = 0;

    while (std::getline(ss, line, '\n')) {
        if (line.length() < 2) {
            throw std::runtime_error("invalid bank length");
        }

        Bank bank = Bank(2);

        for (int i = 0; i < (int)line.length(); i++) {
            Battery curr{i, line[i] - '0'};

            if (i < (int)bank.size) {
                bank.batteries[i] = curr;
                continue;
            }

            long max_joltage = Bank::joltage(bank.batteries);
            int best = -1;
            for (int j = 0; j < (int)bank.size; j++) {
                Battery temp = bank.batteries[j];
                bank.batteries[j] = curr;

                long voltage = Bank::joltage(bank.batteries);
                if (voltage > max_joltage) {
                    max_joltage = voltage;
                    best = j;
                }

                bank.batteries[j] = temp;
            }

            if (best != -1) {
                bank.batteries[best] = curr;
            }
        }

        total_joltage += Bank::joltage(bank.batteries);
    }

    return std::to_string(total_joltage);
}

std::string Day03Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    long total_joltage = 0;

    while (std::getline(ss, line, '\n')) {
        if (line.length() < 2) {
            throw std::runtime_error("invalid bank length");
        }

        Bank bank = Bank(12);

        for (int i = 0; i < (int)line.length(); i++) {
            Battery curr{i, line[i] - '0'};

            if (i < (int)bank.size) {
                bank.batteries[i] = curr;
                continue;
            }

            long max_joltage = Bank::joltage(bank.batteries);
            int best = -1;
            for (int j = 0; j < (int)bank.size; j++) {
                Battery temp = bank.batteries[j];
                bank.batteries[j] = curr;

                long voltage = Bank::joltage(bank.batteries);
                if (voltage > max_joltage) {
                    max_joltage = voltage;
                    best = j;
                }

                bank.batteries[j] = temp;
            }

            if (best != -1) {
                bank.batteries[best] = curr;
            }
        }

        total_joltage += Bank::joltage(bank.batteries);
    }

    return std::to_string(total_joltage);

}

