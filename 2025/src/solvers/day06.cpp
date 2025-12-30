#include "day06.h"
#include <cctype>
#include <sstream>
#include <string>
#include <vector>

int Day06Solver::kDay() {
    return 6;
}

class Problem {
    public:
        std::vector<std::string> words;
        long solve() {
            long res(std::stol(words[0]));
            std::string op(words.back());
            for (auto it = words.begin() + 1; it != words.end() - 1; ++it) {
                long val = std::stol(*it);
                switch (op[0]) {
                    case '+':
                        res += val;
                        break;
                    case '*':
                        res *= val;
                        break;
                }
            }
            return res;
        };
        Problem() : words() {}
};

std::string Day06Solver::one(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<Problem> worksheet;

    while (std::getline(ss, line)) {
        std::string word;
        size_t i(0);

        for (auto c : line) {
            if (c == ' ') {
                if (word.empty()) {
                    continue;
                }

                if (i >= worksheet.size()) {
                    worksheet.emplace_back();
                }
                worksheet[i].words.push_back(word);

                i++;
                word = "";
            } else {
                word.push_back(c);
            }
        }

        if (!word.empty()) {
            if (i >= worksheet.size()) {
                worksheet.emplace_back();
            }
            worksheet[i].words.push_back(word);
        }
    }


    long res = 0;
    for (auto& problem : worksheet) {
        res += problem.solve();
    }

    return std::to_string(res);
}

std::string Day06Solver::two(std::string input) {
    std::stringstream ss(input);
    std::string line;

    std::vector<std::vector<char>> grid;
    size_t max_row_size(0);
    while (std::getline(ss, line)) {
        std::vector<char> row;
        for (auto c : line) {
            row.push_back(c);
        }
        grid.push_back(row);

        if (row.size() > max_row_size) {
            max_row_size = row.size();
        }
    }

    long res(0);
    Problem p;
    std::string op;
    for (int x = static_cast<int>(max_row_size) - 1; x >= 0; x--) {
        std::string word;
        bool is_empty_col(true);

        for (size_t y = 0; y < grid.size(); y++) {
            if (x >= static_cast<int>(grid[y].size())) {
                continue;
            }

            auto c = grid[y][x];
            if (c != ' ') {
                is_empty_col = false;
            }

            if (c >= '0' && c <= '9') {
                word.push_back(c);
            }

            if (c == '+' || c == '*') {
                op = c;
            }
        }

        if (!word.empty()) {
            p.words.push_back(word);
        }

        bool is_last_col = (x == 0);
        if (is_empty_col || is_last_col) {
            p.words.push_back(op);
            res += p.solve();
            p.words.clear();
            op.clear();
        }
    }

    return std::to_string(res);
}

